use async_trait::async_trait;
use thiserror::Error;

use crate::cli::{MeteorCommand, MeteorResult};

#[derive(clap::Parser, Debug)]
#[clap(about = "Add dependency to manifest")]
pub struct Add {}

#[derive(Error, Debug)]
pub enum AddError {}

#[async_trait]
impl MeteorCommand<AddError> for Add {
    async fn execute(&self) -> MeteorResult<AddError> {
        Ok(())
    }
}
