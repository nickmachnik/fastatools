use clap::ArgMatches;
use fasta::{FastaAccessions, FastaEntry, FastaIndex, FastaLengths};
use log::{error, info};
use rayon::prelude::*;
use std::ffi::OsStr;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn accessions(args: ArgMatches) {
    let c = args.subcommand_matches("accessions").unwrap();
    let inpath = Path::new(c.value_of("input").unwrap());
    let accessions = FastaAccessions::from_fasta(&inpath);
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
    info!("Indexing: {:?};", inpath);
    let fasta_index = FastaIndex::new(&inpath);
    let outpath = inpath.with_extension("index");
    info!("Writing index to: {:?};", outpath);
    fasta_index
        .to_json(&outpath)
        .expect("Writing the index file failed!");
    info!("All done.");
}

pub fn subset(args: ArgMatches) {
    let c = args.subcommand_matches("subset").unwrap();
    let index_path = Path::new(c.value_of("fasta index").unwrap());
    let id_path = Path::new(c.value_of("protein ids").unwrap());
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
    for line in BufReader::new(&mut fasta::open(id_path)).lines() {
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
