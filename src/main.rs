use tokio::runtime;

mod cli;
mod commands;
mod config;
mod core;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .max_blocking_threads(4)
        .thread_name("meteor")
        .enable_all()
        .build()
        .expect("Failed to create runtime")
        .block_on(cli::run())
}
