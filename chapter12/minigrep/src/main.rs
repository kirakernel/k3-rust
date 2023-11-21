use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // let (query, file_path) = parse_config(&args);
    // let config = parse_config(&args);
    // let config = Config::new(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    /*println!("Searching for {}", query);
    println!("In file {}", file_path);*/
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // run(config);
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}