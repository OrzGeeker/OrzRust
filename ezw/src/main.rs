use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
enum Commands {
    Config {
        #[command(subcommand)]
        command: GitCommands,
    },
}
#[derive(Debug, Subcommand)]
enum GitCommands {
    
}
#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
fn main() {
    let args = Cli::parse();
    println!("{:?}", args);
}
