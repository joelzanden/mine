use mine::run;
use std::process;

fn main() {
    if let Err(e) = run() {
        println!("Application error: {e}");
        process::exit(1);
    }
}
