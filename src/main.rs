use std::process;

use structopt::StructOpt;
use vimcrypto::cli;

fn main() {
    let opts = cli::Params::from_args();
    match vimcrypto::run(opts) {
        Ok(_) => (),
        Err(_) => process::exit(1),
    }
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
