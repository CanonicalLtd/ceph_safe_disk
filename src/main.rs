extern crate getopts;

use std::env;
use std::process;

pub static NAME: &'static str = "ceph-safe-disk";

use getopts::Options;

fn print_help(opts: Options) {
    println!("{0}", opts.usage("Usage: ceph-safe-disk [OPTION]"));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut options = Options::new();

    options.optflag("h", "help", "Print help information");

    let matches = match options.parse(&args[1..]) {
        Ok(m) => m,
        Err(err) => {
            println!("{}: {}", NAME, err.to_string());
            process::exit(1);
        }
    };

    if matches.opt_present("h") {
        print_help(options);
    }
}
