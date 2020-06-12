use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug, PartialEq)]
#[structopt(name = "basic")]
pub struct Params {
    #[structopt(short, long)]
    pub file: PathBuf,
}

#[cfg(test)]
mod test {
    use super::Params;
    use std::path::PathBuf;
    use structopt::StructOpt;

    #[test]
    fn parse_good_args() {
        let args: Vec<String> = vec!["program".to_string(), "-f".to_string(), "file".to_string()];
        assert_eq!(
            Params {
                file: PathBuf::from("file"),
            },
            Params::from_iter(args.into_iter())
        )
    }
}
