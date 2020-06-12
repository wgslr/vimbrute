use std::error;
use std::fmt;
use std::fs;

pub mod cli;
pub mod crypto;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct BadInputFile;

impl fmt::Display for BadInputFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid encrypted file format")
    }
}

impl error::Error for BadInputFile {}

pub fn run(params: cli::Params) -> Result<()> {
    let file_data = fs::read(&params.file)?;

    match &file_data[0..12] {
        b"VimCrypt~03!" => (),
        _ => {
            eprintln!("Provided file does not contain data encrypted using VimCrypt03 method.");
            return Err(BadInputFile.into());
        }
    }


    let result = crypto::blowfish2_decrypt(&file_data[12..], "12345678912345");
    print!("{}", String::from_utf8(result).unwrap());

    Ok(())
}
