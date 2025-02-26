use clap::Parser;
use log::{info, warn};

/// ezw - Personal Easy Work CLI Util
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn print_progress() {
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}

fn main() {
    let args = Args::parse();

    let xs = vec![1, 2, 3];
    println!("list is: {:?}", xs);
    for _ in 0..args.count {
        println!("Hello {:?}!", args.name);
    }

    print_progress();

    info!("start up");
    warn!("oops! nothing implementation");
}

#[test]
fn test_print_progress() {
    assert_eq!(2, 2);
}
