use crate::fasta::SequenceRecord;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct MarkovChain {
    pub transition_probabilities: HashMap<String, HashMap<char, f64>>,
    pub starting_states: HashMap<String, f64>,
}

impl MarkovChain {
    pub fn new(records: Vec<SequenceRecord>) -> MarkovChain {
        let k = 4;

        // Count kmers
        let mut markov_chain: HashMap<String, HashMap<char, u32>> = HashMap::new();
        let mut starting_states: HashMap<String, f64> = HashMap::new();
        for record in records.iter() {
            if record.seq.len() >= 4 {
                for i in 0..record.seq.len() - k + 1 {
                    let triplet = &record.seq[i..i + 3];
                    let next_nucleotide = record.seq.as_bytes()[i + 3] as char;

                    if triplet.contains('N') || next_nucleotide == 'N' {
                        continue;
                    }

                    starting_states
                        .entry(triplet.to_string())
                        .and_modify(|count| *count += 1.0)
                        .or_insert(1.0);

                    markov_chain
                        .entry(triplet.to_string())
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
            let transition_probabilities = transitions
                .iter()
                .map(|(nucleotide, count)| (*nucleotide, *count as f64 / total_transitions as f64))
                .collect::<HashMap<_, _>>();

            probabilities.insert(triplet.clone(), transition_probabilities);
        }

        // Normalize starting states
        let total_starting_states: f64 = starting_states.values().sum();
        for (_, count) in starting_states.iter_mut() {
            *count /= total_starting_states;
        }

        MarkovChain {
            transition_probabilities: probabilities,
            starting_states: starting_states,
        }
    }

    pub fn generate_sequence(&self, records: Vec<SequenceRecord>) {
        for record in records.iter() {
           todo!(); 
        }
    }
}
