#![feature(test)]

use std::cmp::min;
use std::error;
use std::fmt;
use std::fs;
use std::io;
use std::io::BufRead;
use std::string;
use std::sync::{self, mpsc, Arc};
use threadpool::ThreadPool;

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

enum Message {
    Job(String),
    Quit,
}

pub fn run_threaded(params: cli::Params) -> Result<i32> {
    let pool = ThreadPool::new(params.threads as usize);

    let file_data = fs::read(&params.file)?;
    let header = &file_data[0..12];

    match header {
        b"VimCrypt~03!" => (),
        _ => {
            eprintln!("Provided file does not contain data encrypted using VimCrypt03 method.");
            return Err(BadInputFile.into());
        }
    }

    let encrypted_data: sync::Arc<Vec<u8>> = sync::Arc::new(file_data[12..].to_vec());
    let (sender, receiver) = mpsc::sync_channel((params.threads * 2) as usize);
    let receiver_mutex = Arc::new(sync::Mutex::new(receiver));

    for _ in 0..params.threads {
        let pointer = Arc::clone(&encrypted_data);
        let receiver_pointer = Arc::clone(&receiver_mutex);
        pool.execute(move || loop {
            match receiver_pointer.lock().unwrap().recv().unwrap() {
                Message::Job(password) => {
                    if attempt_decrypt(&pointer, &password) {
                        println!("{}", &password)
                    }
                }
                Message::Quit => break,
            }
        })
    }

    let mut counter = 0;
    for line in io::stdin().lock().lines() {
        match line {
            Ok(password) => {
                sender.send(Message::Job(password)).unwrap();
            }
            Err(_) => break,
        }
        counter += 1;
        if counter % 1000 == 0 {
            eprintln!("Tried {} passwords", counter)
        }
    }
    eprintln!("All lines",);
    for i in 0..params.threads {
        eprintln!("Send Quit to thread {}", i);
        sender.send(Message::Quit).unwrap();
    }
    pool.join();
    Ok(1)
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
