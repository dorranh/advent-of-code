use aoc_2023::days::one::one;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of the day to run
    #[arg(short, long)]
    day: u8,
}

fn main() {
    let args = Args::parse();

    if args.day == 1 {
        one::solution();
    } else {
        println!(
            "Unknown day {}. Make sure the supplied value ranges between 1-25.",
            args.day
        );
    }
}
