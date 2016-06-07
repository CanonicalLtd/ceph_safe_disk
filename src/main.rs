extern crate getopts;
extern crate ceph;

use std::env;
use std::process;

use ceph::exit::*;
use ceph::diag::*;

pub static NAME: &'static str = "ceph-safe-disk";

use getopts::Options;

fn print_help(opts: Options) {
    println!("{0}", opts.usage("Usage: ceph-safe-disk [OPTION]"));
    println!("Exit statuses:
    1: General error
    2: Safe to remove an OSD
    3: Not safe to remove an OSD");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut options = Options::new();

    options.optflag("h", "help", "Print help information");
    options.optflag("q", "quick", "Give a quick, non-exhaustive status of removable OSDs");

    let matches = match options.parse(&args[1..]) {
        Ok(m) => m,
        Err(err) => {
            println!("{}: {}", NAME, err.to_string());
            process::exit(ExitStatus::Err as i32);
        }
    };

    if matches.opt_present("h") {
        print_help(options);
    } else if matches.opt_present("q") {
        match DiagMap::new() {
            Ok(diag_map) => {
                match diag_map.quick_diag() {
                    Status::Safe => { process::exit(ExitStatus::SafeRm as i32) },
                    Status::NonSafe => { process::exit(ExitStatus::NonSafeRm as i32) },
                    _ => { process::exit(ExitStatus::Err as i32) }
                }
            },
            Err(err) => {
                print!("{}: {}", NAME, err.to_string());
                process::exit(ExitStatus::Err as i32);
            }
        }
    } else {
        println!("{0}: No options given, please see {0} -h for help", NAME);
    }
}
