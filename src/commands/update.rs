use crate::cli::{MeteorCommand, MeteorResult};
use async_trait::async_trait;
use thiserror::Error;

#[derive(clap::Parser, Debug)]
#[clap(about = "Update project dependencies in meteor.lock")]
pub struct Update {}

#[derive(Debug, Error)]
pub enum UpdateError {}

#[async_trait]
impl MeteorCommand<UpdateError> for Update {
    async fn execute(&self) -> MeteorResult<UpdateError> {
        Ok(())
    }
}
