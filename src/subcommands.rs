use clap::ArgMatches;
use fasta::index::FastaIndex;
use fasta::pieces::{FastaAccessions, FastaEntry, FastaLengths};
use log::{error, info};
use rayon::prelude::*;
use std::ffi::OsStr;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn accessions(args: ArgMatches) {
    let c = args.subcommand_matches("accessions").unwrap();
    let inpath = Path::new(c.value_of("input").unwrap());
    let accessions = FastaAccessions::from_fasta(&inpaths);
    let outpath = inpath.with_extension("accessions");
    info!("Writing accessions to: {:?};", outpath);
    accessions
        .to_tsv(&outpath)
        .expect("Writing accessions failed!");
    info!("All done.");
}

pub fn index(args: ArgMatches) {
    let c = args.subcommand_matches("index").unwrap();
    let inpath = Path::new(c.value_of("input").unwrap());
    if inpath.extension() == Some(OsStr::new("gz")) {
        error!("Attempted to index compressed file: {:?}", inpath);
    }
    let separator = c.value_of("separator").unwrap_or("|");
    info!("Using separator: {:?};", separator);

    let id_index = if let Some(val) = c.value_of("id-index") {
        val.parse()
            .expect("Could not parse provided id-index as integer")
    } else {
        1
    };
    info!("Using id index: {:?};", id_index);

    info!("Indexing: {:?};", inpath);
    let fasta_index = FastaIndex::new(&inpath, separator, id_index);
    let outpath = inpath.with_extension("index");
    info!("Writing index to: {:?};", outpath);
    fasta_index
        .to_json(&outpath)
        .expect("Writing the index file failed!");
    info!("All done.");
}

pub fn get_entry(args: ArgMatches) {
    let c = args.subcommand_matches("get-entry").unwrap();
    let index_path = Path::new(c.value_of("fasta index").unwrap());
    let fasta_path = Path::new(c.value_of("fasta").unwrap());
    if fasta_path.extension() == Some(OsStr::new("gz")) {
        error!(
            "Attempted to use index on compressed file: {:?}",
            fasta_path
        );
    }
    // load index
    let index = FastaIndex::from_json(index_path).expect("Reading index from file failed!");
    let target_id = c.value_of("target id").unwrap();
    if let Some(target_index) = index.id_to_offset.get(target_id) {
        let entry = FastaEntry::from_index(&fasta_path, *target_index)
            .expect("Error while accessing fasta at index.");
        println!("{}\n{}\n", entry.description, entry.sequence);
    } else {
        error!("Target id not found in index!")
    }
}

pub fn subset(args: ArgMatches) {
    let c = args.subcommand_matches("subset").unwrap();
    let index_path = Path::new(c.value_of("fasta index").unwrap());
    let id_path = Path::new(c.value_of("target ids").unwrap());
    let fasta_path = Path::new(c.value_of("fasta").unwrap());
    if fasta_path.extension() == Some(OsStr::new("gz")) {
        error!(
            "Attempted to use index on compressed file: {:?}",
            fasta_path
        );
    }
    // load index
    info!("Loading index from: {:?};", index_path);
    let index = FastaIndex::from_json(index_path).expect("Reading index from file failed!");
    // load ids
    let mut ids = Vec::new();
    for line in BufReader::new(&mut fasta::helpers::open(id_path)).lines() {
        let l = line.unwrap();
        if l != "" {
            ids.push(l);
        }
    }
    info!("Subsetting: {:?};", fasta_path);
    ids.par_iter()
        .filter_map(|e| index.id_to_offset.get(e))
        .for_each(|i| {
            let entry = FastaEntry::from_index(&fasta_path, *i).unwrap();
            println!("{}\n{}\n", entry.description, entry.sequence);
        });
    info!("All done.");
}

pub fn lengths(args: ArgMatches) {
    let c = args.subcommand_matches("lengths").unwrap();
    let inpath = Path::new(c.value_of("input").unwrap());
    info!("Getting sequence lengths for: {:?};", inpath);
    let lengths = FastaLengths::from_fasta(&inpath);
    let outpath = inpath.with_extension("lengths");
    info!("Writing lengths to: {:?};", outpath);
    lengths
        .to_json(&outpath)
        .expect("Writing the lengths failed!");
    info!("All done.");
}
