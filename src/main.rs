#[macro_use]
extern crate log;
extern crate chrono;
extern crate env_logger;
extern crate clap;
extern crate fasta;

use std::io::Write;
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use clap::{Arg, App, SubCommand, AppSettings};

mod subcommands;

fn main() {
    // log time stamp
    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    let index = SubCommand::with_name("index")
    .about("Index the input fasta file.")
    .arg(
        Arg::with_name("input")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("Path to input fasta file."),
    );

    let args = App::new("fastatools")
        .version("0.1.0")
        .author("Nick Noel Machnik <nick.machnik@gmail.com>")
        .about("Toolset for the manipulation of fasta files.")
        .subcommand(index)
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    match args.subcommand_name() {
        Some("index") => {
            subcommands::index(args);
        },
        Some(other) => unimplemented!("{}", other),
        None => panic!("what is supposed to happen here"),
    }
}