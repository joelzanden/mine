use mine::run;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "--version" {
        println!("{}", env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    if let Err(e) = run() {
        println!("Application error: {e}");
        process::exit(1);
    }
}
