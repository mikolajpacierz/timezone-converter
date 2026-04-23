mod converter;

use crate::converter::convert_time;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    date: String,
    #[arg(short, long)]
    time: String,
    #[arg(short, long)]
    abbr: String,
}

fn main() {
    let args = Args::parse();

    let date = args.date;
    let time = args.time;
    let abbr = &args.abbr;
    let datetime = format!("{date} {time}");
    println!("{}", convert_time(&datetime, abbr, "Europe/Warsaw"));
}
