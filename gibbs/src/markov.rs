use crate::fasta::SequenceRecord;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub struct MarkovChain {
    transition_matrix: HashMap<String, HashMap<char, f64>>
}

impl MarkovChain {

    fn new(inputs: Vec<SequenceRecord>, order: u8) -> HashMap<String, HashMap<char, f64>> {
        let mut kmer = count_kmers(order, inputs)
        let mut lkmer = init_freqj_dict(order+1, inputs);


        let mut transition_matrix: HashMap<String, HashMap<char, f64>> = HashMap::new();
    
        for (four_mer, &count) in four_mers.iter() {
            let three_mer = &four_mer[..3]; // Get the first 3 characters of the 4-mer
            let next_nucleotide = four_mer.chars().nth(3).unwrap(); // Get the fourth character
    
            if let Some(&three_mer_count) = three_mers.get(three_mer) {
                let probability = count as f64 / three_mer_count as f64;
    
                transition_matrix.entry(three_mer.to_string())
                    .or_default()
                    .insert(next_nucleotide, probability);
            }
        }
    
        transition_matrix
    }

    pub fn init_freq_dict(k: u8) -> HashMap<String, u32> {
        let nucleotides = vec!['A', 'C', 'G', 'T'];
        let kmer_iterator = (0..k)
            .map(|_| nucleotides.iter())
            .multi_cartesian_product()
            .map(|product| product.into_iter().collect::<String>());
        let mut freq_dict = HashMap::new();
        for kmer in kmer_iterator {
            freq_dict.insert(kmer, 0);
        }
        freq_dict
    }

    pub fn count_kmers(k: usize, records_vec: Vec<SequenceRecord>) -> HashMap<String, u32>  {
        let mut kmer_counts = MarkovChain::init_kmer_dict(k.try_into().unwrap());
        let keys: HashSet<String> = kmer_counts.keys().cloned().collect();
        for record in records_vec.iter() {
            for i in 0..record.seq.len()-k+1 {
                let kmer = &record.seq[i..i+k];
                if keys.contains(kmer) {
                    *kmer_counts.get_mut(kmer).unwrap() += 1;
                }
            }
        }
        kmer_counts
    }

    


}