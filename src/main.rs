/*
 * File: src/main.rs
 * Date: 12.09.2018
 * Author: MarkAtk
 * 
 * MIT License
 * 
 * Copyright (c) 2018 MarkAtk
 * 
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
 * of the Software, and to permit persons to whom the Software is furnished to do
 * so, subject to the following conditions:
 * 
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

#[macro_use]
extern crate clap;

extern crate serial_unit_testing;

use clap::{App, ArgMatches, AppSettings};

mod commands;

mod list;
mod send;
mod monitor;
mod check;
mod run;
mod verify;

fn run(matches: ArgMatches) -> Result<(), String> {
    match matches.subcommand() {
        ("send", Some(m)) => send::run(m),
        ("list", Some(m)) => list::run(m),
        ("monitor", Some(m)) => monitor::run(m),
        ("check", Some(m)) => check::run(m),
        ("run", Some(m)) => run::run(m),
        ("verify", Some(m)) => verify::run(m),
        _ => Ok(())
    }
}

fn main() {
    let matches = App::new("serial-unit-testing")
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version(crate_version!())
        .version_short("v")
        .about("Serial unit testing framework")
        .subcommand(send::command())
        .subcommand(list::command())
        .subcommand(monitor::command())
        .subcommand(check::command())
        .subcommand(run::command())
        .subcommand(verify::command())
        .get_matches();

    if let Err(e) = run(matches) {
        println!("{}", e);

        return;
    }
}
