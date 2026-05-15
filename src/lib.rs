#![feature(if_let_guard)]
#![feature(clone_from_ref)]

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::*;

mod args;

#[proc_macro_attribute]
pub fn add(attr: TokenStream, item: TokenStream) -> TokenStream {
    if attr.is_empty() {
        // Do nothing if argument hasn't been provided yet.
        return item;
    }

    let args = syn::parse_macro_input!(attr as args::AddArguments);
    let mut file = syn::parse_macro_input!(item as File);

    let items = get_items(&mut file);
    for item in items {
        let vis = match item {
            Item::Fn(f) => &mut f.vis,
            Item::Struct(s) => &mut s.vis,
            Item::Enum(e) => &mut e.vis,
            Item::Union(u) => &mut u.vis,
            Item::Static(e) => &mut e.vis,
            Item::Const(c) => &mut c.vis,
            Item::Trait(t) => &mut t.vis,
            Item::Type(t) => &mut t.vis,
            Item::TraitAlias(t) => &mut t.vis,
            _ => continue,
        };
        args.replace_restricted_vis_path(vis);
    }

    file.into_token_stream().into()
}

/// Get the items in the annotated module.
fn get_items(file: &mut File) -> &mut [Item] {
    const ERR_MOD: &str = "The annotated file must be a module.";
    assert_eq!(file.items.len(), 1, "{ERR_MOD}");
    let Item::Mod(module) = &mut file.items[0] else {
        panic!("{ERR_MOD}")
    };
    let module = module
        .content
        .as_mut()
        .expect("The attribute must be inside a module. i.e. `#![short_vis_path::add]`");
    &mut module.1
}
