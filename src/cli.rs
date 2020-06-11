use super::Error;

#[derive(Debug, PartialEq)]
pub struct Params {
    pub encrypted_file: String,
    pub password: String,
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

#[cfg(test)]
mod test {
    use super::Params;

    #[test]
    fn parse_good_args() {
        let args: Vec<String> = vec![
            "program".to_string(),
            "-f".to_string(),
            "file".to_string(),
            "-p".to_string(),
            "pass".to_string(),
        ];
        assert_eq!(
            Params {
                encrypted_file: String::from("file"),
                password: String::from("pass"),
            },
            Params::new(args.into_iter()).unwrap()
        )
    }
}
