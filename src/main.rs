use std::process;

use structopt::StructOpt;
use vimbrute::cli;

fn main() {
    let opts = cli::Params::from_args();
    match vimbrute::run_threaded(opts) {
        Ok(0) => {
            eprintln!("0 tried passwords yielded valid utf-8",);
            process::exit(1);
        }
        Ok(matches) => {
            eprintln!("{} tried passwords yielded valid utf-8", matches);
            process::exit(0);
        }
        Err(_) => process::exit(2),
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
