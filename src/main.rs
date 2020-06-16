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

    let separator = Arg::with_name("separator")
        .long("separator")
        .short("s")
        .required(false)
        .takes_value(true)
        .global(true)
        .default_value("|")
        .help(
            "Separator string in the description file. \
                    Is used to split the description and get the sequence ID, \
                    assumed to be at the first position of the split description.\
                    Default: `|`",
        );

    let id_index = Arg::with_name("id-index")
        .long("id-index")
        .short("i")
        .required(false)
        .takes_value(true)
        .default_value("1")
        .help(
            "0-based index of the description field that is used \
                    as a unique sequence id after splitting with the specified \
                    separator. \
                    Default: `1`",
        );

    let index = SubCommand::with_name("index")
        .about(
            "Indexes a fasta file. \
            Writes to input file location with '.index' extension.",
        )
        .arg(
            Arg::with_name("input")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("Path to input fasta file (uncompressed)"),
        );

    let accessions = SubCommand::with_name("accessions")
        .about(
            "Gets accessions from fastas deflines and writes them to file, one per line. \
            Accessions are written to <input>.accessions",
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
            "Extracts the sequence lengths from a fasta file. \
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
        .about("Subsets a fasta file with a set of protein ids protein ids. Print to stdout.")
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
            Arg::with_name("target ids")
                .required(true)
                .takes_value(true)
                .index(3)
                .help("Extract sequences with these ids from the fasta. One id per row."),
        );

    let get_entry = SubCommand::with_name("get-entry")
        .about("Prints an entry with a given index id from a fasta file.")
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
            Arg::with_name("target id")
                .required(true)
                .takes_value(true)
                .index(3)
                .help("ID of the target entry, as specified in the fasta index."),
        );

    let args = App::new("fastatools")
        .version("0.1.0")
        .author("Nick Noel Machnik <nick.machnik@gmail.com>")
        .about("Toolset for the manipulation of fasta files.")
        .arg(id_index)
        .arg(separator)
        .subcommand(index)
        .subcommand(subset)
        .subcommand(lengths)
        .subcommand(accessions)
        .subcommand(get_entry)
        .setting(AppSettings::SubcommandRequiredElseHelp)
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
        Some("get-entry") => {
            subcommands::get_entry(args);
        }
        Some(other) => unimplemented!("{}", other),
        None => panic!("what is supposed to happen here"),
    }
}
