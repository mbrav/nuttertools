use clap::{arg, ArgAction, Args};

use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::Error;

/// A program that will rat on all files you pass to it
#[derive(Args)]
pub struct Options {
    /// Specify files to concat
    #[arg(required=true, value_name="FILES", action=ArgAction::Append)]
    files: Vec<String>,
    /// Specify whether to print line numbers
    #[arg(short = 'l', long = "lines", action=ArgAction::SetTrue)]
    line_numbers: bool,
    /// Specify print blanks
    #[arg(short = 'n', long = "nonblank", action=ArgAction::SetTrue)]
    nonblank_lines: bool,
}

/// # Errors
///
/// Will return `Err` if any error is encountered
pub fn main(
    Options {
        files,
        line_numbers,
        nonblank_lines,
    }: &Options,
) -> Result<(), Error> {
    for filename in files {
        let file = open(filename);
        let mut last_num = 0;
        for (line_num, line_result) in file.lines().enumerate() {
            let line = line_result.expect("No lines");
            if *line_numbers {
                println!("{:6}\t{line}", line_num + 1);
            } else if *nonblank_lines {
                if line.is_empty() {
                    println!();
                } else {
                    last_num += 1;
                    println!("{last_num:6}\t{line}");
                }
            } else {
                println!("{line}");
            }
        }
    }

    Ok(())
}

fn open(filename: &str) -> Box<dyn BufRead> {
    let file = File::open(filename).expect("Error reading file");
    Box::new(BufReader::new(file))
}
