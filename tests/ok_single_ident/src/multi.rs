#![short_vis_path::add(procfs, fs = crate::multi)]

pub(in procfs) fn foo() {}
pub(in fs) fn bar() {}
