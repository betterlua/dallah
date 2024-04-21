use std::path::Path;

use async_trait::async_trait;
use thiserror::Error;

use crate::{
    cli::{MeteorCommand, MeteorResult},
    commands::project_create::*,
    config::{ManifestConfig, MANIFEST_FILE},
};

#[derive(clap::Parser, Debug)]
#[clap(about = "Initialize Meteor project in existing directory")]
pub struct Init {
    #[clap(short, long, default_value = ".")]
    path: String,
    #[clap(short, long)]
    lib: bool,
    #[clap(short, long)]
    name: Option<String>,
}

#[derive(Error, Debug)]
pub enum InitError {
    #[error("Directory  not empty")]
    DirectoryNotEmpty,
    #[error("Directory does not exist")]
    DirectoryDoesNotExist,
}

#[async_trait]
impl MeteorCommand<InitError> for Init {
    async fn execute(&self) -> MeteorResult<InitError> {
        let path = Path::new(&self.path);

        let name = match &self.name {
            Some(name) => name,
            None => path.file_name().unwrap().to_str().unwrap(),
        };

        if !path.exists() {
            return Err(InitError::DirectoryDoesNotExist);
        }

        if path.is_dir() {
            let dir = path.read_dir().unwrap().collect::<Vec<_>>();
            if dir.len() > 0 {
                return Err(InitError::DirectoryNotEmpty);
            }
        }

        let config = ManifestConfig::create(name.to_string());
        create_file(path, MANIFEST_FILE, config.to_string().as_str());

        if self.lib {
            r#type::lib(path);
        } else {
            r#type::bin(path);
        }

        println!(
            "  Initialized {} `{}`",
            (if self.lib { "library" } else { "binary" }),
            name
        );

        Ok(())
    }
}
