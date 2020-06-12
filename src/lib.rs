#![feature(test)]

use std::cmp::min;
use std::error;
use std::fmt;
use std::fs;
use std::io;
use std::io::BufRead;
use std::string;

extern crate test;

pub mod cli;
pub mod crypto;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

const PEEK_SIZE: usize = 256;

#[derive(Debug, Clone)]
struct BadInputFile;

impl fmt::Display for BadInputFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid encrypted file format")
    }
}

impl error::Error for BadInputFile {}

/// # Returns
/// - Ok: number of matches
/// - Err: error
pub fn run(params: cli::Params) -> Result<i32> {
    let file_data = fs::read(&params.file)?;
    let header = &file_data[0..12];
    let encrypted_data = &file_data[12..];

    match header {
        b"VimCrypt~03!" => (),
        _ => {
            eprintln!("Provided file does not contain data encrypted using VimCrypt03 method.");
            return Err(BadInputFile.into());
        }
    }

    let mut counter = 0;
    let mut matches = 0;
    for line in io::stdin().lock().lines() {
        match line {
            Ok(password) => {
                if attempt_decrypt(&encrypted_data, &password) {
                    matches += 1;
                    println!("{}", &password)
                }
            }
            Err(_) => break,
        }
        counter += 1;
        if counter % 1000 == 0 {
            eprintln!("Tried {} passwords", counter)
        }
    }

    Ok(matches)
}

// Tries to decrypt the file using given password.
// If the obtained plaintext is valid UTF8, the operation
// is considered succesful.
fn attempt_decrypt(encrypted_data: &[u8], password: &str) -> bool {
    let result = crypto::blowfish2_decrypt(
        &encrypted_data[0..PEEK_SIZE.min(encrypted_data.len())],
        password,
    );
    match String::from_utf8(result) {
        Ok(_) => true,
        Err(string::FromUtf8Error { .. }) => false,
    }
}
