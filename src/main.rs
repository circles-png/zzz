#![warn(clippy::pedantic, clippy::nursery, clippy::unwrap_used)]
use anyhow::{anyhow, Result};
use colored::{Color, Colorize};
use humantime::format_duration;
use rand::{seq::SliceRandom, thread_rng};
use std::{
    fs::{read_to_string, remove_file, File},
    io::Write,
};

use chrono::{Days, Local, NaiveDateTime, NaiveTime};
use clap::{Parser, Subcommand};

fn main() -> Result<()> {
    // path to file that stores the wake-up time
    const TIME_PATH: &str = "time";
    // parse the cli with std::env::args through clap
    let cli = Cli::parse();
    // match the command
    match cli.command {
        Commands::Now => {
            let string = &read_to_string(TIME_PATH).map_err(|_| {
                anyhow!("Time not set or invalid data. Use `zzz time` to set the time.")
            })?;
            let string = string.trim();
            // parse the time from the file
            let date_time =
                NaiveDateTime::parse_from_str(string, "%Y-%m-%d %H:%M:%S%.f").map_err(|_| {
                    anyhow!("Invalid time format in time file. Use `zzz time` to set the time.")
                })?;
            // get the current time
            let now = Local::now().naive_local();
            // calculate the remaining time
            let Ok(remaining) = date_time.signed_duration_since(now).to_std() else {
                println!("Current time is after the set time, deleting the time file.");
                remove_file(TIME_PATH).map_err(|_| {
                    anyhow!("Failed to delete the time file. Please delete it manually.")
                })?;
                return Ok(());
            };
            let remaining = format_duration(remaining);
            println!(
                "{} {remaining}\n              {} {string}",
                "Remaining sleep:".dimmed(),
                "to".dimmed(),
            );
        }
        Commands::Time { time } => {
            // parse the time
            let time = NaiveTime::parse_from_str(&time, "%H:%M:%S")
                .map_err(|_| anyhow!("Invalid time format (expected 24-hour HH:MM:SS)"))?;
            // set the time to tomorrow
            let date_time = NaiveDateTime::new(
                Local::now()
                    .naive_local()
                    .checked_add_days(Days::new(1))
                    .ok_or_else(|| {
                        anyhow!("Failed to calculate tomorrow's date because it is out of range.")
                    })?
                    .date(),
                time,
            );
            // write the time to the file
            File::create(TIME_PATH)
                .map_err(|_| anyhow!("Failed to create the time file"))?
                .write_all(date_time.to_string().as_bytes())
                .map_err(|_| anyhow!("Failed to write to the time file"))?;
            println!(
                "{} {} {}",
                "Time set to".dimmed(),
                time,
                "tomorrow.".dimmed()
            );
        }
        Commands::Sleep => {
            // print the lullaby
            for line in include_str!("lullaby").lines() {
                let colour = unsafe {
                    [
                        Color::BrightRed,
                        Color::BrightGreen,
                        Color::BrightYellow,
                        Color::BrightBlue,
                        Color::BrightMagenta,
                        Color::BrightCyan,
                    ]
                    .choose(&mut thread_rng())
                    .unwrap_unchecked()
                };
                println!("{}", line.color(*colour));
            }
        }
    }
    Ok(())
}

/// The sleep counter for night owls
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
