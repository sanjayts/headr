use headr::{get_args, run};
use std::env;
use std::process::exit;

fn main() {
    let cmd_args: Vec<String> = env::args().collect();
    if let Err(e) = get_args(cmd_args).and_then(|cfg| run(&cfg)) {
        eprintln!("{}", e);
        exit(1);
    }
}
