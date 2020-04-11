use clap::{ArgMatches};
use fasta::{FastaIndex, FastaMap};
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::ffi::OsStr;

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
    fasta_index.to_json(&outpath);
}

pub fn subset(args: ArgMatches) {
    let c = args.subcommand_matches("subset").unwrap();
    let index_path = Path::new(c.value_of("fasta index").unwrap());
    let id_path = Path::new(c.value_of("protein ids").unwrap());
    let fasta_path = Path::new(c.value_of("fasta").unwrap());
    let outpath = Path::new(c.value_of("output file").unwrap());
    // load index
    let index = FastaIndex::from_json(index_path);
    // load ids
    let mut ids: Vec<_> = Vec::new();
    for line in BufReader::new(&mut fasta::open(id_path)).lines() {
        let l = line.unwrap();
        if l != "" {
            ids.push(l);
        }
    }
    if fasta_path.extension() == Some(OsStr::new("gz")) {
        error!("Attempted to use index on compressed file: {:?}", fasta_path);
    }
    info!("Subsetting: {:?};", fasta_path);
    let subset_map = FastaMap::from_index_with_ids(&fasta_path, &index, &ids);
    info!("Writing results: {:?};", outpath);
    subset_map.to_fasta(&outpath);
}