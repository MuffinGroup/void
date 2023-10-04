use std::{fs::File, io::Write};

use super::{Project, ProjType};

#[derive(Debug)]
pub struct CargoProject {
    name: String,
    proj_dir: String,
}

impl Project for CargoProject {
    fn new(name: &str, proj_dir: String) -> Self {
        Self { name: name.to_string(), proj_dir: proj_dir.to_string() }
    }

    fn create_proj(&self) {
        let cfg_layout = format!(
r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
"#, self.name
        );
        let mut file = match File::create(format!("{}/Cargo.toml", self.proj_dir)) {
            Ok(file) => file,
            Err(err) => panic!("{}", err),
        };

        match file.write_all(cfg_layout.as_bytes()) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }

    fn get_cfg_file(&self) -> String {
        let cfg_file = format!("{}/Cargo.toml", self.proj_dir);
        cfg_file
    }

    fn get_proj_type(&self) -> ProjType {
        ProjType::RUST
    }
}
