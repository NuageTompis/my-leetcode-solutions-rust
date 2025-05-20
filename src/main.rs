use args::LCArgs;
use clap::Parser;

mod args;
mod clip;
mod config;
mod create;
mod fetch;
mod linked_list;
mod molds_helper;
mod parse_api;
mod read_write;
mod solutions;
mod tree;

#[tokio::main]
async fn main() {
    let args = LCArgs::parse();
    use args::MainCommand as MC;
    match args.arg_type {
        MC::Config(config) => {
            config::handle_config_command(config);
        }
        MC::Create(create) => {
            let _res = create::handle_create_command(create).await;
        }
        MC::Fetch(fetch) => {
            fetch::handle_fetch_command(fetch);
        }
        MC::Clip(problem_id_command) => {
            let _res = clip::handle_clip_command(problem_id_command).await;
        }
    }
}
