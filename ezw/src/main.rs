use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    name: String,
}
fn main() {
    let args = Args::parse();
    println!("hello, world! {}", args.name);
}
