use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf, // file system paths work for cross-platform
}
fn main() {
    let args = Cli::parse();
    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}
