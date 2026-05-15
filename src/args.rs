use proc_macro2::Span;
use std::collections::BTreeMap;
use syn::{parse::Parse, punctuated::Punctuated, *};

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
        let path = get_expanded_module_path();

        Ok(AddArguments {
            args: args
                .into_iter()
                .map(|arg| match arg {
                    Argument::Single(ident) => (ident, path.clone()),
                    Argument::Override(ident, path) => (ident, path.clone()),
                })
                .collect(),
        })
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

/// Get the module path to be used in `pub(in ...)`, based on directory structure.
/// For example, if the attribute is in `a/src/procfs.rs`, this function returns
/// `crate::procfs`; if in `a/src/fs/procfs/mod.rs`, returns `crate::fs::procfs`.
fn get_expanded_module_path() -> Path {
    let callsite_span = Span::call_site();
    let Some(local_path) = callsite_span.local_file() else {
        panic!("Unknown local file path to call site span {callsite_span:?}.");
    };
    let Ok(local_path) = local_path.canonicalize() else {
        panic!("Unable to canonicalize {local_path:?}.")
    };

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("Failed to get manifest dir.");

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

    Path {
        leading_colon: None,
        segments: std::iter::once("crate")
            .chain(
                std::path::Path::new(module_str)
                    .iter()
                    .map(|m| m.to_str().unwrap()),
            )
            .map(|m| PathSegment::from(Ident::new(m, callsite_span)))
            .collect(),
    }
}
