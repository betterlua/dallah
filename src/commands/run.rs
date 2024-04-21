use crate::cli::{MeteorCommand, MeteorResult};
use async_trait::async_trait;
use thiserror::Error;

#[derive(clap::Parser, Debug)]
#[clap(about = "Run a binary in the project")]
pub struct Run {}

#[derive(Debug, Error)]
pub enum RunError {}

#[async_trait]
impl MeteorCommand<RunError> for Run {
    async fn execute(&self) -> MeteorResult<RunError> {
        Ok(())
    }
}
