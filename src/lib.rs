#![feature(if_let_guard)]
#![feature(clone_from_ref)]

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;
use syn::*;

#[proc_macro_attribute]
pub fn add(attr: TokenStream, item: TokenStream) -> TokenStream {
    let path = expanded_path();

    let arg = syn::parse_macro_input!(attr as Ident);
    let mut file = syn::parse_macro_input!(item as File);

    const ERR_MOD: &str = "The annotated file must be a module.";
    assert_eq!(file.items.len(), 1, "{ERR_MOD}");
    let Item::Mod(module) = &mut file.items[0] else {
        panic!("{ERR_MOD}")
    };
    let module = module
        .content
        .as_mut()
        .expect("The attribute must be inside a module. i.e. `#![short_vis_path::add]`");
    for item in &mut module.1 {
        match item {
            Item::Fn(f) => replace_restricted_vis_path(&path, &arg, &mut f.vis),
            Item::Struct(s) => replace_restricted_vis_path(&path, &arg, &mut s.vis),
            Item::Enum(e) => replace_restricted_vis_path(&path, &arg, &mut e.vis),
            Item::Static(e) => replace_restricted_vis_path(&path, &arg, &mut e.vis),
            Item::Const(c) => replace_restricted_vis_path(&path, &arg, &mut c.vis),
            _ => (),
        }
    }

    file.into_token_stream().into()
}

fn replace_restricted_vis_path(path: &Path, ident: &Ident, vis: &mut Visibility) {
    if let Visibility::Restricted(vis) = vis
        && let Some(input) = vis.path.get_ident()
        && input == ident
    {
        vis.path = Box::clone_from_ref(path);
        println!("replaced: {:?}", quote::quote! {#vis});
    }
}

fn expanded_path() -> Path {
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
    let Some(module) = module_path.to_str().unwrap().strip_suffix(".rs") else {
        panic!("{module_path:?} must be a rs file.")
    };

    Path {
        leading_colon: None,
        segments: std::iter::once("crate")
            .chain(
                std::path::Path::new(module)
                    .iter()
                    .map(|m| m.to_str().unwrap()),
            )
            .map(|m| PathSegment::from(Ident::new(m, callsite_span)))
            .collect(),
    }
}
