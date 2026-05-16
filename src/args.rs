use proc_macro2::Span;
use std::collections::BTreeMap;
use syn::{parse::Parse, punctuated::Punctuated, visit_mut::*, *};

pub enum Argument {
    Single(Ident),
    Override(Ident, Path),
}

impl Parse for Argument {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        Ok(if input.peek2(Token![=]) {
            let ident: Ident = input.parse()?;
            let _: Token![=] = input.parse()?;
            let path: Path = input.parse()?;
            Argument::Override(ident, path)
        } else {
            let ident: Ident = input.parse()?;
            Argument::Single(ident)
        })
    }
}

pub struct AddArguments {
    pub args: BTreeMap<Ident, Path>,
}

impl Parse for AddArguments {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        // Parse multiple arguments.
        let args = Punctuated::<Argument, Token![,]>::parse_terminated(input)?;

        // Default module path inferred from file path.
        let path = ExpandedPath::new();

        Ok(AddArguments {
            args: args
                .into_iter()
                .map(|arg| match arg {
                    Argument::Single(ident) => {
                        let Some(tokens) = path.to_syn_path(&ident) else {
                            panic!(
                                "The path `{}` doesn't contain `{ident}`. \
                                 Please choose a corrent short module name.",
                                path.segment.join("::")
                            )
                        };
                        (ident, tokens)
                    }
                    Argument::Override(ident, path) => (ident, path.clone()),
                })
                .collect(),
        })
    }
}

impl visit_mut::VisitMut for AddArguments {
    fn visit_item_mut(&mut self, item: &mut Item) {
        let vis = match item {
            Item::Fn(f) => {
                visit_item_fn_mut(self, f);
                &mut f.vis
            }
            Item::Struct(s) => {
                visit_item_struct_mut(self, s);
                &mut s.vis
            }
            Item::Enum(e) => {
                visit_item_enum_mut(self, e);
                &mut e.vis
            }
            Item::Union(u) => {
                visit_item_union_mut(self, u);
                &mut u.vis
            }
            Item::Static(s) => {
                visit_item_static_mut(self, s);
                &mut s.vis
            }
            Item::Const(c) => {
                visit_item_const_mut(self, c);
                &mut c.vis
            }
            Item::Trait(t) => {
                visit_item_trait_mut(self, t);
                &mut t.vis
            }
            Item::Type(t) => &mut t.vis,
            Item::TraitAlias(t) => &mut t.vis,
            Item::Mod(m) => {
                visit_item_mod_mut(self, m);
                &mut m.vis
            }
            Item::Impl(i) => {
                visit_item_impl_mut(self, i);
                return;
            }
            _ => return,
        };

        self.replace_restricted_vis_path(vis);
    }

    fn visit_impl_item_mut(&mut self, i: &mut ImplItem) {
        let vis = match i {
            ImplItem::Const(c) => &mut c.vis,
            ImplItem::Fn(f) => &mut f.vis,
            _ => return,
        };
        self.replace_restricted_vis_path(vis);

        // Recurse.
        visit_impl_item_mut(self, i);
    }
}

impl AddArguments {
    /// Replace `pub(in subsystem)` by `pub(in crate::to::subsystem)`.
    pub fn replace_restricted_vis_path(&self, vis: &mut Visibility) {
        if let Visibility::Restricted(vis) = vis
            && let Some(input) = vis.path.get_ident()
            && let Some(path) = self.args.get(input)
        {
            vis.path = Box::clone_from_ref(path);
        }
    }
}

/// Module path to be replaced with, at best effort of guess basis on source file layout.
struct ExpandedPath {
    /// Starting from `crate`.
    segment: Vec<String>,
    /// Callsite span.
    callsite_span: Span,
}

impl ExpandedPath {
    /// Get the module path to be used in `pub(in ...)`, based on directory structure.
    /// For example, if the attribute is in `a/src/procfs.rs`, this function returns
    /// `crate::procfs`; if in `a/src/fs/procfs/mod.rs`, returns `crate::fs::procfs`.
    fn new() -> Self {
        let callsite_span = Span::call_site();
        let Some(local_path) = callsite_span.local_file() else {
            panic!("Unknown local file path to call site span {callsite_span:?}.");
        };
        let Ok(local_path) = local_path.canonicalize() else {
            panic!("Unable to canonicalize {local_path:?}.")
        };

        let manifest_dir =
            std::env::var("CARGO_MANIFEST_DIR").expect("Failed to get manifest dir.");

        let Ok(relative_path) = local_path.strip_prefix(&manifest_dir) else {
            panic!("{manifest_dir:?} must be a prefix of {local_path:?}.")
        };

        let Ok(module_path) = relative_path.strip_prefix("src") else {
            panic!("`src/` must be a prefix of {relative_path:?}.")
        };

        let module_str = module_path.to_str().unwrap();
        // Handle `xx/mod_name/mod.rs` module style.
        let module_str = module_str.strip_suffix("/mod.rs").unwrap_or(module_str);
        // Handle `xx/mod_name.rs` module style.
        let module_str = module_str.strip_suffix(".rs").unwrap_or(module_str);

        ExpandedPath {
            segment: std::iter::once("crate")
                .chain(
                    std::path::Path::new(module_str)
                        .iter()
                        .map(|m| m.to_str().unwrap()),
                )
                .map(String::from)
                .collect(),
            callsite_span,
        }
    }

    /// Generate the Path tokens, starting from `crate` to `end` (both included).
    /// Returns None when the module path doesn't contain `end`.
    fn to_syn_path(&self, end: &Ident) -> Option<Path> {
        let pos = self.segment.iter().rposition(|seg| end == seg.as_str())?;
        Some(Path {
            leading_colon: None,
            segments: self.segment[..pos + 1]
                .iter()
                .map(|s| PathSegment::from(Ident::new(s, self.callsite_span)))
                .collect(),
        })
    }
}
