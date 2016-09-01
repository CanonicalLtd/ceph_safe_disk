extern crate getopts;
extern crate ceph;

use std::env;
use std::process;

use ceph::exit::*;
use ceph::diag::*;
use ceph::exec::check_user;

pub static NAME: &'static str = "ceph-safe-disk";

use getopts::Options;

fn print_help(opts: Options) {
    println!("{0}", opts.usage("Usage: ceph-safe-disk [OPTION]"));
    println!("Exit statuses:
    0: Safe to remove an OSD
    1: Not safe to remove an OSD
    2: General error");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut options = Options::new();

    options.optflag("h", "help", "Print help information");
    options.optflag("q",
                    "quick",
                    "Give a quick, non-exhaustive status of removable OSDs");
    options.optflag("e",
                    "exhaustive",
                    "Give an exhaustive status of removable OSDs");

    let matches = match options.parse(&args[1..]) {
        Ok(m) => m,
        Err(err) => {
            println!("{}: {}", NAME, err.to_string());
            process::exit(ExitStatus::Err as i32);
        }
    };

    if matches.opt_present("h") {
        print_help(options);
    } else {
        if let Err(user_err) = check_user() {
            println!("{}: {}", NAME, user_err.to_string());
            process::exit(ExitStatus::Err as i32);
        };
        match DiagMap::new() {
            Ok(diag_map) => {
                if matches.opt_present("q") {
                    match diag_map.quick_diag() {
                        true => process::exit(ExitStatus::SafeRm as i32),
                        false => process::exit(ExitStatus::NonSafeRm as i32),
                    }
                } else if matches.opt_present("e") {
                    match diag_map.exhaustive_diag() {
                        Status::Safe => process::exit(ExitStatus::SafeRm as i32),
                        Status::NonSafe => process::exit(ExitStatus::NonSafeRm as i32),
                        _ => process::exit(ExitStatus::Err as i32),
                    }
                }
            }
            Err(err) => {
                print!("{}: {}", NAME, err.to_string());
                process::exit(ExitStatus::Err as i32);
            }
        }
    }
}
