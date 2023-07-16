use clap::{Args, Parser, Subcommand};
use std::path::Path;

mod config;
mod manager;
mod my_format;
mod third_party_library_details;

#[derive(Parser)]
#[clap(version = "1.0", author = "Jesus Bracho")]
struct Opts {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
enum Command {
    SetRoot(SetRootArgs),
    ClearRoot,
}

#[derive(Args)]
struct SetRootArgs {
    root_dir: String,
}

trait Executable {
    fn execute(&self);
}

impl Executable for Command {
    fn execute(&self) {
        match self {
            Command::SetRoot(args) => {
                todo!()
            }
            Command::ClearRoot => {}
        }
    }
}

fn main() {
    let opts: Opts = Opts::parse();
    opts.command.execute();
}

fn set_engine_root_override(root_dir: &str) {
    if Path::new(root_dir).exists() {
        println!("Engine root directory set to {}", root_dir);
    } else {
        println!("Directory {} does not exist", root_dir);
    }
}

fn clear_engine_root_override() {
    println!("Engine root directory cleared");
}
