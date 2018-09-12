extern crate clap;

use clap::{App, Arg, SubCommand, ArgMatches};

fn run(matches: ArgMatches) -> Result<(), String> {
    match matches.subcommand() {
        ("send", Some(m)) => run_send(m),
        ("list", Some(m)) => run_list(m),
        ("monitor", Some(m)) => run_monitor(m),
        _ => Ok(())
    }
}

fn run_send(matches: &ArgMatches) -> Result<(), String> {
    Ok(())
}

fn run_list(matches: &ArgMatches) -> Result<(), String> {
    Ok(())
}

fn run_monitor(matches: &ArgMatches) -> Result<(), String> {
    Ok(())
}

fn main() {
    let matches = App::new("serial-unit-testing")
        .version("v0.1")
        .version_short("v")
        .about("Serial unit testing framework")
        .subcommand(SubCommand::with_name("send")
            .about("Send data to serial port")
            .arg(Arg::with_name("port")
                .long("port")
                .short("p")
                .help("Serial port OS specific name")))
        .get_matches();

    if let Err(e) = run(matches) {
        println!("Application error: {}", e);

        return;
    }
}
