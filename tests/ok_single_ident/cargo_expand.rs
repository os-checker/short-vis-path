#![feature(prelude_import)]
#![feature(proc_macro_hygiene)]
#![feature(custom_inner_attributes)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
mod procfs {
    pub(in crate::procfs) trait T {}
}
mod fs {
    mod procfs {
        pub(in crate::fs::procfs) enum E {}
    }
}
mod fs2 {
    mod procfs {
        pub(in crate::fs2::procfs) struct S;
    }
}
fn main() {}
