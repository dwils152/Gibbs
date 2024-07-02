use clap::{arg, command, value_parser, ArgAction, Command};
use gibbs::fasta::FastaParser;
use gibbs::markov_2::MarkovChain;

fn get_bg_seqs(lengths: Vec<u32>, markov_chain: &MarkovChain) -> Vec<String> {
    lengths
        .iter()
        .map(|&length| {
            let start = markov_chain.sample_start();
            markov_chain.generate_bg_seq(&start, length)
        })
        .collect()
}

fn main() {
    let matches = Command::new("gibbs sampler")
        .arg(arg!(-i --input <FILE> "Input fasta file"))
        .get_matches();

    let fasta_path: String = matches
        .get_one::<String>("input")
        .expect("Fasta file is required")
        .to_string();

    let fasta_records = FastaParser::parse(&fasta_path).unwrap();
    let markov_chain = MarkovChain::new(fasta_records);
    //println!("{:?}", markov_chain.transition_probabilities);
    //println!("{:?}", markov_chain.starting_states);
    let start = markov_chain.sample_start();
    let test_seq = markov_chain.generate_bg_seq(&start, 100);
    println!("{}", test_seq);
}
