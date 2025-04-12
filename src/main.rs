use args::LCArgs;
use clap::Parser;

mod args;
mod config;
mod create;
mod fetch;
mod read_write;
mod parse_api;
mod solutions;

#[tokio::main]
async fn main() {
    let args = LCArgs::parse();
    match args.arg_type {
        args::MainCommand::Config(config) => {
            config::handle_config_command(config);
        }
        args::MainCommand::Create(create) => {
            create::handle_create_command(create).await;
        }
        args::MainCommand::Fetch(fetch) => {
            fetch::handle_fetch_command(fetch);
        }
    }
}
