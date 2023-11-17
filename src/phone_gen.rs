use clap::{arg, Args};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::mem::size_of_val;

use crate::Error;

/// Brute force all possible phone numbers
#[derive(Args)]
pub struct Options {
    /// Specify string country code.
    /// For example, if country code '1' is specified
    /// Then all numbers will be generated starting with '1'
    #[arg(short = 'c', long = "country_code", default_value = "1")]
    pub country_code: String,
    /// Specify string prefix.
    /// For example, if prefix `800` is specified with country code '1'
    /// Then only numbers in `1800XXXXXXX` will be generated
    #[arg(short = 'p', long = "prefix", default_value = "800")]
    pub prefix: String,
    /// Output file name with valid path
    #[arg(short = 'f', long = "file", default_value = "phone_gen.txt")]
    pub file: String,
    /// Number of cartesian combinations
    /// This is the total count of `X` digits that need to be generated
    pub combinations: Option<usize>,
}

/// # Errors
///
/// Will return `Err` in case of error
pub fn main(opts: &Options) -> Result<(), Error> {
    println!("RUNNING PHONE GEN");
    println!("Country Code: {}", opts.country_code);
    println!("Prefix: {}", opts.prefix);
    println!("Output to: {}", opts.file);

    let gen = Options::new(&opts.country_code, &opts.prefix, &opts.file);
    gen.generate();
    Ok(())
}

/// General Phone number generation struct
impl Options {
    #[must_use]
    pub fn new(country_code: &String, prefix: &String, file: &String) -> Self {
        let combinations = Some(10 - prefix.to_string().len());
        Self {
            country_code: country_code.into(),
            prefix: format!("{country_code}{prefix}"),
            combinations,
            file: file.into(),
        }
    }

    pub fn generate(&self) {
        // Generate numbers from 0 to combinations size
        let nums: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        println!("Elements: {nums:?}");

        let file = open_file(&self.file);
        self.cartesian_product(&nums, &file);
    }

    fn cartesian_product(&self, numbers: &[i32], file: &File) {
        let buffer_size = 100_000;

        let mut buffer: VecDeque<String> = VecDeque::with_capacity(buffer_size);

        // Preallocate the buffer vector
        buffer.reserve(buffer_size);

        // Create a matrix of all numbers times number of combinations
        let mut v: Vec<Vec<i32>> = vec![numbers.to_vec(); self.combinations.unwrap()];

        let mut indices = vec![0; v.len()];
        let mut done = false;

        while !done {
            let r = self.prefix.clone() + &v.iter().map(|&i| i.to_string()).collect::<String>();

            buffer.push_back(r);

            if buffer.len() == buffer_size {
                write_string_array(&buffer, file);
                buffer.clear();
            }

            for i in (0..v.len()).rev() {
                indices[i] += 1;
                if indices[i] < v[i].len() {
                    break;
                }
                indices[i] = 0;
                if i == 0 {
                    done = true;
                }
            }
        }
        if !buffer.is_empty() {
            write_string_array(&buffer, file);
        }
    }
}

fn open_file(path: &String) -> File {
    File::create(path).expect("Unable to create file")
}

fn write_string_array(array: &VecDeque<String>, mut file: &File) {
    // Create a buffered writer for improved I/O performance
    let mut writer = BufWriter::new(file);

    // Join the strings and write to the file
    let buffer = array.join("\n");
    println!("Buffer size: {} KB", size_of_val(&*buffer) / 1024);

    writer
        .write_all(buffer.as_bytes())
        .expect("Unable to write to file");
}
