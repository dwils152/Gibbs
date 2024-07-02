use crate::fasta::SequenceRecord;
use itertools::Itertools;
use rand::Rng;
use std::collections::HashMap;

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

    pub fn sample_start(&self) -> String {
        let kmers: Vec<String> = self.starting_states.keys().cloned().collect();
        let probabilities: Vec<f64> = self.starting_states.values().cloned().collect();
        let mut cdf = Vec::with_capacity(probabilities.len() + 1);
        cdf.push(0.0);

        for (idx, prob) in probabilities.iter().enumerate() {
            cdf.push(cdf[idx] + prob);
        }

        // generate a random number between 0 and 1
        let mut rng = rand::thread_rng();
        let mut random_number = rng.gen::<f64>();

        // sample the starting state
        let mut idx = 0;
        while random_number > cdf[idx] {
            idx += 1;
        }

        let starting_state = kmers[idx].clone();
        starting_state
    }

    fn get_next_state(&self, current_state: &str) -> char {
        let nucleotides = vec!['A', 'C', 'G', 'T'];
        let probabilities = self.transition_probabilities.get(current_state).unwrap();
        let mut cdf = Vec::with_capacity(probabilities.len() + 1);
        cdf.push(0.0);

        for nucleotide in nucleotides.iter() {
            cdf.push(cdf.last().unwrap() + probabilities.get(nucleotide).unwrap());
        }

        let mut rng = rand::thread_rng();
        let mut random_number = rng.gen::<f64>();

        let mut idx = 0;
        while random_number > cdf[idx] {
            idx += 1;
        }

        nucleotides[idx - 1]
    }

    pub fn get_current_state(&self, sequence: &str, k: usize) -> String {
        let current_state = &sequence[sequence.len() - k..];
        current_state.to_string()
    }

    pub fn generate_bg_seq(&self, starting_state: &str, length: usize) -> String {
        let current_state = starting_state;
        let mut bg = String::with_capacity(length);
        bg.push_str(current_state);

        while bg.len() < length {
            let nucleotide = self.get_next_state(&current_state);
            bg.push(nucleotide);
            //println!("{}", bg);
            let current_state = self.get_current_state(&bg, 3);
        }

        bg
    }
}
