#![short_vis_path::add(procfs)]

mod inner {
    pub(in procfs) struct S;
}
