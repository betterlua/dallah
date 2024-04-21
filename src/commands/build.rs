use async_trait::async_trait;
use crate::cli::{MeteorCommand, MeteorResult};
use thiserror::Error;


#[derive(clap::Parser, Debug)]
#[clap(about = "Build the current project")]
pub struct Build {


}


#[derive(Debug, Error)]
pub enum BuildError {}


#[async_trait]
impl MeteorCommand<BuildError> for Build {
    async fn execute(&self) -> MeteorResult<BuildError> {
        Ok(())
    }
}
