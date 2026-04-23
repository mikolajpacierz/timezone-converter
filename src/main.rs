mod converter;

use crate::converter::convert_time;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, required = true)]
    date: String,
    #[arg(short, long, required = true)]
    time: String,
    #[arg(short, long, required = true)]
    abbr: String,
    #[arg(short, long, default_value = "Europe/Warsaw")]
    local_timezone: String,
}

fn main() {
    let args = Args::parse();

    let datetime = format!("{} {}", args.date, args.time);
    println!("{}", convert_time(&datetime, &args.abbr, &args.local_timezone));
}
