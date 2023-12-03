use anyhow::Context;
use cargo_metadata::Metadata;
use indoc::{formatdoc, indoc};
use log::debug;
use reqwest::blocking as req;
use std::{
    fs::{self, read_to_string, File, OpenOptions},
    io::{self, Write},
    path::Path,
};

const AOC_YEAR: &str = "2023";

/// Only create file if path doesn't exist
fn create_new<P: AsRef<Path>>(path: P) -> io::Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(path.as_ref())
}

/// Generates the files for the new day
///
/// Scaffolds the project files for the new day of Advent of Code.
/// Then, we try to download the input file using the session key
/// in the `~/.adventofcode` file.
pub fn generate_day(day: u64, metadata: &Metadata) -> anyhow::Result<()> {
    let day_folder = format!("day-{day:0>2}");
    let location = metadata.workspace_root.as_std_path().join(day_folder);
    debug!("New folder location: {}", location.display());
    fs::create_dir_all(location.join("src"))?;
    if let Ok(mut file) = create_new(location.join("Cargo.toml")) {
        println!("Creating Cargo.toml");
        file.write_all(
            formatdoc! { r#"
[package]
name = "day-{day:0>2}"
version = "1.0.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
itertools = "0.12.0"
nom = "7.1.1"
utils = {{ path = "../utils", version = "*" }}

"# }
            .as_bytes(),
        )?;
    } else {
        println!("Cargo.toml exists");
    }
    if let Ok(mut file) = create_new(location.join("src").join("main.rs")) {
        println!("Creating main.rs");
        file.write_all(
            indoc! { r#"
use itertools::Itertools;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq)]
struct InputData {}

fn parse(input: &str) -> ParseResult<InputData> {
    todo!()
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<()> {
    Ok(())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<()> {
    Ok(())
}

aoc_main!(parse, part1, part2);

#[test]
fn test() {}

"# }
            .as_bytes(),
        )?;
    } else {
        println!("main.rs exists");
    }
    if location.join("input.txt").exists() {
        println!("input.txt exists");
    } else {
        generate_input(day, &location.join("input.txt"))?;
    }
    Ok(())
}

pub fn generate_input(day: u64, location: &Path) -> anyhow::Result<()> {
    println!("Retrieving input.txt");
    let aoc_session = read_to_string(
        dirs::home_dir()
            .context("No home directory")?
            .join(".adventofcode"),
    )?
    .trim()
    .to_string();
    let client = req::Client::new();
    let input_data = client
        .request(
            reqwest::Method::GET,
            format!("https://adventofcode.com/{AOC_YEAR}/day/{day}/input"),
        )
        .header(reqwest::header::COOKIE, format!("session={aoc_session}"))
        .send()?
        .error_for_status()?
        .text()?;
    fs::write(location, input_data)?;
    Ok(())
}
