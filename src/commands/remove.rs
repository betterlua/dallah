use async_trait::async_trait;
use crate::cli::{MeteorCommand, MeteorResult};
use thiserror::Error;


#[derive(clap::Parser, Debug)]
#[clap(about = "Remove dependencies from manifest")]
pub struct Remove {


}


#[derive(Debug, Error)]
pub enum RemoveError {}


#[async_trait]
impl MeteorCommand<RemoveError> for Remove {
    async fn execute(&self) -> MeteorResult<RemoveError> {
        Ok(())
    }
}
