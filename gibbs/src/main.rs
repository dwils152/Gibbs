use gibbs::fasta::FastaParser;
use gibbs::markov::MarkovChain;
use clap::{arg, command, value_parser, ArgAction, Command};

fn main() {
    let matches = Command::new("gibbs sampler")
        .arg(arg!(-i --input <FILE> "Input fasta file"))
        .get_matches();

    let fasta_path: String = matches
        .get_one::<String>("input")
        .expect("Fasta file is required")
        .to_string();
    
    let fasta_records = FastaParser::parse(&fasta_path).unwrap();
    let markov_chain = MarkovChain::init_kmer_dict(4);
    let counts = MarkovChain::count_kmers(4, fasta_records);


    

}
