extern crate clap;
use clap::{App,Arg};
use portscan::Config;
use std::process;

fn main() {
    let matches = App::new("portscan")
       .version("0.1")
       .about("A portscanner")
       .author("Martin Pugh")
       .arg(Arg::with_name("target")
            .short("t")
            .value_name("TARGET")
            .help("Specifies the target IP address")
            .required(true))
       .arg(Arg::with_name("ports")
            .short("p")
            .value_name("PORTS")
            .help("Specifies the target ports")
            .required(true))
       .get_matches();
    let config = Config::new(
        matches.value_of("target").unwrap(),
        matches.value_of("ports").unwrap()
        ).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    portscan::run(config);
}


