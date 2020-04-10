use clap::{ArgMatches};
use fasta::{FastaIndex};
use std::path::Path;

pub fn index(args: ArgMatches) {
    let c = args.subcommand_matches("index").unwrap();
    let inpath = Path::new(c.value_of("input").unwrap());
    info!("Indexing: {:?};", inpath);
    let fasta_index = FastaIndex::new(&inpath);
    let outpath = inpath.with_extension("index");
    info!("Writing index to: {:?};", outpath);
    fasta_index.to_json(&outpath);
}

pub fn subset(args: ArgMatches) {
    // load ids, load index
    // use index to subset fasta file
    // write to new file one by one
    // if not found, continue (have to change `from_index_with_ids`)
    unimplemented!()
}