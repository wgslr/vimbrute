//! Code based on MIT-licensed vimdecrypt-rs:
//! <https://github.com/SirVer/vimdecrypt-rs/blob/master/src/lib.rs>

use generic_array::GenericArray;
use sha2;
use sha2::Digest;

use blowfish::block_cipher::{BlockCipher, NewBlockCipher};

type BlowfishBE = blowfish::Blowfish<byteorder::BigEndian>;

/// # Arguments
///
/// - `all_data` - encrypted data, without vim's encryption type marker
///
/// # Examples
/// ```
/// let data = [
///     0x79, 0x1d, 0x67, 0xb8, 0x3b, 0xfd, 0x7a, 0x1e, 0x68, 0xcb, 0xab,
///     0x17, 0x0c, 0x42, 0xae, 0x39, 0x70, 0x93, 0xab, 0xa3, 0xc9, 0x32
/// ];
/// assert_eq!(
///     vimcrypto::crypto::blowfish2_decrypt(&data, "123"),
///     b"short\n"
/// );
/// ```
pub fn blowfish2_decrypt(all_data: &[u8], password: &str) -> Vec<u8> {
    let salt = &all_data[0..8];
    let mut iv = all_data[8..16].to_vec();
    let data = all_data[16..].to_vec();

    let key = hashpw(password, salt);
    let bf = BlowfishBE::new_varkey(&key).unwrap();

    let mut xor = vec![8; 0];
    let mut plaintext = Vec::new();
    for o in 0..data.len() {
        if o % 8 == 0 {
            wordswap(&mut iv);
            bf.encrypt_block(&mut GenericArray::from_mut_slice(&mut iv));
            wordswap(&mut iv);
            xor = iv;
            iv = data[o..(o + 8).min(data.len())].to_vec();
        }
        plaintext.push(xor[o % 8] ^ data[o]);
    }
    plaintext
}

fn sha256(password: &[u8], salt: &[u8]) -> Vec<u8> {
    let mut hasher = sha2::Sha256::default();
    hasher.input(password);
    hasher.input(salt);
    hasher.result().to_vec()
}

fn to_hex_string(bytes: &[u8]) -> String {
    let strs: Vec<String> = bytes.iter().map(|b| format!("{:02x}", b)).collect();
    strs.join("")
}

fn hashpw(password: &str, salt: &[u8]) -> Vec<u8> {
    let mut key = sha256(password.as_bytes(), salt);
    for _ in 0..1000 {
        key = sha256(to_hex_string(&key).as_bytes(), salt);
    }
    key
}

fn wordswap(a: &mut [u8]) {
    assert_eq!(a.len(), 8);
    a.swap(0, 3);
    a.swap(1, 2);
    a.swap(4, 7);
    a.swap(5, 6);
}
