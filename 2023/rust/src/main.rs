use aoc_2023::days::one::one;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of the day to run
    #[arg(short, long)]
    day: u8,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if args.day == 1 {
        // FIXME: Make the input path a CLI arg
        let file = File::open("data/puzzle-input-1.txt")?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();
        one::solution(lines);
    } else {
        println!(
            "Unknown day {}. Make sure the supplied value ranges between 1-25.",
            args.day
        );
    }
    Ok(())
}
