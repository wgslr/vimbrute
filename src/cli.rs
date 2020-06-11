#[derive(Debug)]
pub struct Params {
    pub encrypted_file: String,
    pub password: String,
}

#[derive(Debug)]
pub enum Error {
    ArgsError,
}

impl Params {
    pub fn new<I: Iterator<Item = String>>(mut args: I) -> Result<Params, Error> {
        let mut encrypted_file: Option<String> = None;
        let mut password: Option<String> = None;

        args.next(); // skip executable path
        loop {
            match args.next() {
                None => break,
                Some(s) if s == "-f" => encrypted_file = args.next(),
                Some(s) if s == "-p" => password = args.next(),
                Some(_) => return Err(Error::ArgsError),
            }
        }
        if encrypted_file.is_none() || password.is_none() {
            Err(Error::ArgsError)
        } else {
            Ok(Params {
                encrypted_file: encrypted_file.unwrap().clone(),
                password: password.unwrap().clone(),
            })
        }
    }
}
