use failure::Fail;
use generic_array::GenericArray;
use sha2;
use sha2::Digest;
use std::error;
use std::fs;

use blowfish::block_cipher::{BlockCipher, NewBlockCipher};
use std::fmt;

type BlowfishBE = blowfish::Blowfish<byteorder::BigEndian>;
pub mod cli;
pub mod crypto;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn decrypt_file(path: &str, password: &str) -> Result<Vec<u8>> {
    let data = fs::read(path)?;
    // {
    //     Ok(data) => data,
    //     Err(e) => return Err(Error::DecryptionError),
    // };
    // vimdecrypt::decrypt(&data, &password);
    Ok(Vec::new())
}
