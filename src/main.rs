extern crate io_project;

use std::env;
use std::process;
use io_project::Config;
use std::io::Write;

fn main() {
    let args:Vec<String> = env::args().collect();
    let mut stderr = std::io::stderr();
    let config = Config::new(&args[1..]).unwrap_or_else(|err| {
        writeln!(&mut stderr, "Problem parsing args: {}", err).expect("couldn't write to stderr");
        process::exit(1);
    });
    match io_project::run(config) {
      Err(e) => {
        writeln!(stderr, "There was an error: {}", e).expect("Couldn't write to stderr");
        process::exit(1);
      },
      _ => ()
    };
}


