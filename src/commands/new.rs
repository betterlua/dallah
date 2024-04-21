use std::{fs, path::Path};

use crate::{cli::{MeteorCommand, MeteorResult}, config::{ManifestConfig, MANIFEST_FILE}};
use async_trait::async_trait;
use thiserror::Error;

use super::project_create::{create_file, r#type};

#[derive(clap::Parser, Debug)]
#[clap(about = "Create new Meteor project in existing directory")]
pub struct New {
    path: String,
    #[clap(short, long)]
    name: Option<String>,
    #[clap(short, long)]
    lib: bool,
}

#[derive(Debug, Error)]
pub enum NewError {
    #[error("Directory already exists")]
    DirectoryAlreadyExists,
}

#[async_trait]
impl MeteorCommand<NewError> for New {
    async fn execute(&self) -> MeteorResult<NewError> {
        let path = Path::new(&self.path);

        let name = match &self.name {
            Some(name) => name,
            None => path.file_name().unwrap().to_str().unwrap(),
        };

        if path.exists() {
            return Err(NewError::DirectoryAlreadyExists);
        }

        fs::create_dir_all(path).unwrap();

        let config = ManifestConfig::create(name.to_string());
        create_file(path, MANIFEST_FILE, config.to_string().as_str());

        if self.lib {
            r#type::lib(path);
        } else {
            r#type::bin(path);
        }

        Ok(())
    }
}
