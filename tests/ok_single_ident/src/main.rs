#![feature(proc_macro_hygiene)]
#![feature(custom_inner_attributes)]

mod procfs;

mod fs;

mod fs2 {
    mod procfs;
}

fn main() {}
