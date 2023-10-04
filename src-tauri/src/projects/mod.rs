use std::fmt::Debug;

pub mod cargo;

pub enum ProjType {
    RUST, // rs
    NEXUS, // nx
    TYPESCRIPT, // ts
}

pub trait Project {
    fn new(name: &str, proj_dir: String) -> Self;

    fn create_proj(&self);

    fn get_proj_type(&self) -> ProjType;

    fn get_cfg_file(&self) -> String;
}

pub fn create_proj<T: Project + Debug>(project: T) {
    project.create_proj();
}