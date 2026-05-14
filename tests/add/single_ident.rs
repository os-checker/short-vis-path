#![feature(proc_macro_hygiene)]
#![feature(custom_inner_attributes)]

#[macro_use]
extern crate short_vis_path;

mod procfs {
    #![short_vis_path::add(procfs)]

    pub(in procfs) fn foo() {}
}

fn main() {
    procfs::foo()
}
