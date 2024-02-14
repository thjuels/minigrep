use std::env;

use std::process;

extern crate minigrep;

use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();

    //let config= parse_config(&args); //assigns config using function
    //using Config constructor
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        //changing to eprintln means errors don't output to output.txt
        //when running 'cargo run > output.txt'
        process::exit(1);
    }
}

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();
//     Config { query, filename }
// }
