use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug, PartialEq)]
#[structopt(name = "basic")]
pub struct Params {
    #[structopt(short, long)]
    pub file: PathBuf,

    #[structopt(short, long, default_value = "1")]
    pub threads: u8,
}

#[cfg(test)]
mod test {
    use super::Params;
    use std::path::PathBuf;
    use structopt::StructOpt;

    #[test]
    fn parse_good_args() {
        let args: Vec<String> = vec![
            "program".to_string(),
            "-f".to_string(),
            "file".to_string(),
            "-t".to_string(),
            "3".to_string(),
        ];
        assert_eq!(
            Params {
                file: PathBuf::from("file"),
                threads: 3
            },
            Params::from_iter(args[..].iter())
        );
        assert_eq!(
            Params {
                file: PathBuf::from("file"),
                threads: 1
            },
            Params::from_iter(args[..3].iter())
        );
    }
}
