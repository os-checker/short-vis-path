#![feature(prelude_import)]
#![feature(proc_macro_hygiene)]
#![feature(custom_inner_attributes)]
#![allow(dead_code)]
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
mod mod_rs {
    mod procfs {
        pub(in crate::mod_rs::procfs) struct S;
    }
}
mod override_ {
    #[path = "procfs.rs"]
    pub mod procfs {
        pub(in crate::override_::procfs) const UNIT: () = ();
    }
}
mod adhoc {
    pub mod outer {
        pub(in crate::adhoc::outer) type Unit = ();
    }
}
fn main() {}
