
//! # SSH-LOAD
//!
//! `SSH-LOAD` is a binary tool wriiten in rust to simplify the processs of adding, chaging or removing ssh keys from your local machine
pub mod engine;

use std::{ env, process::exit };
use crate::engine::run::{ load_ssh_key, delete_ssh_key, add_ssh_key };

fn main() {
    let load_string = "--load";
    let delete_string = "--delete";
    let add_string = "--add";

    ///extract the ssh-string
    let args = env::args().collect::<Vec<String>>();
    let ssh_string = env::args().nth(3).unwrap();
    let load = env::args().any(|x| x == load_string);
    let delete = env::args().any(|x| x == delete_string);
    let add = env::args().any(|x| x == add_string);

    println!("ssh-String {}", ssh_string);
    if args.len() < 2 {
        println!("Usage: {} <command> [--load <ssh_key>]", args[0]);
        exit(0);
    } else if load {
        load_ssh_key(&ssh_string);
    } else if add {
        add_ssh_key(&ssh_string);
    } else if delete {
        delete_ssh_key(&ssh_string);
    } else {
        println!("Something is wrong");
        exit(1);
    }
    print!("{:?}", args);
}
