use std::error;
use std::fmt;
use std::fs;
use std::io;
use vimdecrypt;

pub mod cli;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
pub enum Error {
    ArgsError,
    DecryptionError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ArgsError => write!(f, "Incorrect CLI argments"),
            Error::DecryptionError => write!(f, "Decryption failed"),
        }
    }
}

impl error::Error for Error {}

// pub fn decrypt_file(path: &str, password: &str) -> Result<Vec<u8>> {
//     let data = fs::read(path).map_err(|x| x)?;
//     // {
//     //     Ok(data) => data,
//     //     Err(e) => return Err(Error::DecryptionError),
//     // };
//     // vimdecrypt::decrypt(&data, &password);
//     Ok(Vec::new())
// }
