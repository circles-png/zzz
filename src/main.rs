#![warn(clippy::pedantic, clippy::nursery)]
use humantime::format_duration;
use std::{
    fs::{read_to_string, File},
    io::Write, path::Path,
};

use chrono::{Days, Local, NaiveDateTime, NaiveTime};
use clap::{Parser, Subcommand};

fn main() {
    // path to file that stores the wake-up time
    const TIME_PATH: &str = "time";
    // parse the cli with std::env::args through clap
    let cli = Cli::parse();
    // match the command
    match cli.command {
        Commands::Now => {
            if !Path::new(TIME_PATH).exists() {
                eprintln!("Time not set");
                return;
            }
            let date_time = NaiveDateTime::parse_from_str(
                read_to_string(TIME_PATH).unwrap().trim(),
                "%Y-%m-%d %H:%M:%S%.f",
            )
                .unwrap();
            let now = Local::now().naive_local();
            let remaining = format_duration(date_time.signed_duration_since(now).to_std().unwrap());
            println!("Remaining sleep: {remaining}");
        }
        Commands::Time { time } => {
            let Ok(time) = NaiveTime::parse_from_str(&time, "%H:%M:%S") else {
                eprintln!("Invalid time format (expected 24-hour HH:MM:SS)");
                return;
            };
            let date_time = NaiveDateTime::new(
                Local::now()
                    .naive_local()
                    .checked_add_days(Days::new(1))
                    .unwrap()
                    .date(),
                time,
            );
            File::create(TIME_PATH)
                .unwrap()
                .write_all(date_time.to_string().as_bytes())
                .unwrap();
            println!("Time set to {time} tomorrow");
        }
        Commands::Sleep => {
            println!(include_str!("lullaby"));
        }
    }
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "View remaining sleep")]
    Now,
    #[command(about = "Set sleep time")]
    Time {
        #[arg(help = "Time to set (24-hour HH:MM:SS)")]
        time: String,
    },
    #[command(about = "Sing a lullaby")]
    Sleep,
}