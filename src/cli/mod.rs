use async_trait::async_trait;
use clap::Parser;

use crate::commands::{Add, Build, Init, New, Remove, Run, Update};

#[async_trait]
pub trait MeteorCommand<T> {
    async fn execute(&self) -> MeteorResult<T>;
}

pub type MeteorResult<T> = Result<(), T>;

#[derive(clap::Parser)]
pub enum MeteorCli {
    Init(Init),
    New(New),
    Add(Add),
    Remove(Remove),
    Update(Update),
    Build(Build),
    Run(Run),
}

impl MeteorCli {
    async fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let res: Result<(), Box<dyn std::error::Error>> = match self {
            MeteorCli::Init(init) => {
                let res = init.execute().await;
                match res {
                    Ok(_) => Ok(()),
                    Err(err) => Err(Box::new(err)),
                }
            }
            MeteorCli::Add(add) => {
                let res = add.execute().await;
                match res {
                    Ok(_) => Ok(()),
                    Err(err) => Err(Box::new(err)),
                }
            }
            MeteorCli::Update(update) => {
                let res = update.execute().await;
                match res {
                    Ok(_) => Ok(()),
                    Err(err) => Err(Box::new(err)),
                }
            }
            MeteorCli::Remove(remove) => {
                let res = remove.execute().await;
                match res {
                    Ok(_) => Ok(()),
                    Err(err) => Err(Box::new(err)),
                }
            }
            MeteorCli::Build(build) => {
                let res = build.execute().await;
                match res {
                    Ok(_) => Ok(()),
                    Err(err) => Err(Box::new(err)),
                }
            }
            MeteorCli::Run(run) => {
                let res = run.execute().await;
                match res {
                    Ok(_) => Ok(()),
                    Err(err) => Err(Box::new(err)),
                }
            }
            MeteorCli::New(new) => {
                let res = new.execute().await;
                match res {
                    Ok(_) => Ok(()),
                    Err(err) => Err(Box::new(err)),
                }
            }
        };

        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = MeteorCli::parse();
    cli.execute().await
}
