#![short_vis_path::add(fs)]

pub struct S;

impl S {
    pub(in fs) fn nested_item() {}
}

pub(in fs) fn baz() {}
