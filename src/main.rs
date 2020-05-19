use chrono::Local;
use clap::{App, AppSettings, Arg, SubCommand};
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

mod subcommands;

fn main() {
    // log time stamp
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    let index = SubCommand::with_name("index")
        .about(
            "Index the input fasta file. \
            Writes to input file location with '.index' extension.",
        )
        .arg(
            Arg::with_name("input")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("Path to input fasta file (uncompressed)"),
        )
        .arg(
            Arg::with_name("separator")
                .long("separator")
                .short("s")
                .required(false)
                .takes_value(true)
                .help(
                    "Separator string in the description file. \
                    Is used to split the description and get the sequence ID, \
                    assumed to be at the first position of the split description.\
                    Default: `|`",
                ),
        )
        .arg(
            Arg::with_name("id-index")
                .long("id-index")
                .short("i")
                .required(false)
                .takes_value(true)
                .help(
                    "0-based index of the description field that is used \
                    as a unique sequence id after splitting with the specified \
                    separator. \
                    Default: `0`",
                ),
        );

    let accessions = SubCommand::with_name("accessions")
        .about(
            "Get the accessions from the deflines and write them to file, one per line.\
            Accessions will be written to <input>.accessions",
        )
        .arg(
            Arg::with_name("input")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("Path to input fasta file"),
        );

    let lengths = SubCommand::with_name("lengths")
        .about(
            "Extract the sequence lengths from the input fasta file. \
            Writes to input file location with '.lengths' extension.",
        )
        .arg(
            Arg::with_name("input")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("Path to input fasta file"),
        );

    let subset = SubCommand::with_name("subset")
        .about("Subset the input fasta with the given protein ids. Print to stdout.")
        .arg(
            Arg::with_name("fasta")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("Path to input fasta file (uncompressed)"),
        )
        .arg(
            Arg::with_name("fasta index")
                .required(true)
                .takes_value(true)
                .index(2)
                .help("Path to input fasta index (create with `fastatools index`)"),
        )
        .arg(
            Arg::with_name("protein ids")
                .required(true)
                .takes_value(true)
                .index(3)
                .help("Extract sequences with these ids from the fasta. One id per row."),
        );

    let args = App::new("fastatools")
        .version("0.1.0")
        .author("Nick Noel Machnik <nick.machnik@gmail.com>")
        .about("Toolset for the manipulation of fasta files.")
        .subcommand(index)
        .subcommand(subset)
        .subcommand(lengths)
        .subcommand(accessions)
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    match args.subcommand_name() {
        Some("index") => {
            subcommands::index(args);
        }
        Some("subset") => {
            subcommands::subset(args);
        }
        Some("lengths") => {
            subcommands::lengths(args);
        }
        Some("accessions") => {
            subcommands::accessions(args);
        }
        Some(other) => unimplemented!("{}", other),
        None => panic!("what is supposed to happen here"),
    }
}
