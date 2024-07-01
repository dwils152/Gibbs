use crate::fasta::SequenceRecord;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub struct MarkovChain {
    pub transition_probabilities: HashMap<String, HashMap<char, f64>>
}

impl MarkovChain {
    
    pub fn new(records: Vec<SequenceRecord>) -> MarkovChain {
        let k = 4;
    
        // Count kmers
        let mut markov_chain: HashMap<String, HashMap<char, u32>> = HashMap::new();
        for record in records.iter() {
            if record.seq.len() >= 4 {
                for i in 0..record.seq.len() - k + 1 {
                    let triplet = &record.seq[i..i+3];
                    let next_nucleotide = record.seq.as_bytes()[i+3] as char;

                    if triplet.contains('N') || next_nucleotide == 'N' {
                        continue;
                    }

                    markov_chain.entry(triplet.to_string())
                        .or_insert_with(HashMap::new)
                        .entry(next_nucleotide)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }
        }

        // Convert counts to probabilities
        let mut probabilities: HashMap<String, HashMap<char, f64>> = HashMap::new();
        for (triplet, transitions) in &markov_chain {
            let total_transitions: u32 = transitions.values().sum();
            let transition_probabilities = transitions.iter()
                .map(|(nucleotide, count)| {
                    (*nucleotide, *count as f64 / total_transitions as f64)
                })
                .collect::<HashMap<_, _>>();

            probabilities.insert(triplet.clone(), transition_probabilities);
        }

    MarkovChain { transition_probabilities: probabilities }
    }

}