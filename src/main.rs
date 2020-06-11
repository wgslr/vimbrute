use std::env;
use std::process;
use structopt::StructOpt;
use vimcrypto::cli;
use vimcrypto::Error;

fn main() {
    let opts = cli::Params::from_args();
    eprintln!("{:?}", opts);
    // let args = cli::Params::new(env::args()).unwrap_or_else(|err| match err {
    //     Error::ArgsError => {
    //         eprintln!("Incorrect CLI params.");
    //         print_help(false);
    //         process::exit(2);
    //     }
    // });
}

fn print_help(stderr: bool) {
    const HELP_STR: &str = "\
-f FILE\t\tEncrypted file to decrypt
-p PASSWORD\tPassword to use
";
    if stderr {
        eprint!("{}", HELP_STR);
    } else {
        print!("{}", HELP_STR);
    }
}
