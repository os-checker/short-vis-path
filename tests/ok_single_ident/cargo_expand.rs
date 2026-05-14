#![feature(prelude_import)]
#![feature(proc_macro_hygiene)]
#![feature(custom_inner_attributes)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
mod procfs {
    pub(in crate::procfs) fn foo() {}
}
mod fs {
    mod procfs {
        pub(in crate::fs::procfs) fn bar() {}
    }
}
mod fs2 {
    mod procfs {
        pub(in crate::fs2::procfs) fn bar() {}
    }
}
fn main() {}
