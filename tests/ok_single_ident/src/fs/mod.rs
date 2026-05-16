mod nested;
mod procfs;

mod mbe {
    #![short_vis_path::add(fs)]
    pub(in fs) macro mbe() {}
}
