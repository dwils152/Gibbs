use crate::fasta::SequenceRecord;
use std::collections::HashMap;
use itertools::Itertools;

pub struct MarkovChain;

impl MarkovChain {

    pub fn init_kmer_dict(k: u8) -> HashMap<String, u16> {
        let nucleotides = vec!['A', 'C', 'G', 'T'];
        let kmer_iterator = (0..k)
            .map(|_| nucleotides.iter())
            .multi_cartesian_product()
            .map(|product| product.into_iter().collect::<String>());

        let mut kmer_dict = HashMap::new();
        for kmer in kmer_iterator {
            kmer_dict.insert(kmer, 0);
        }

        kmer_dict
    }

    pub fn count_kmers(k: usize, records_vec: Vec<SequenceRecord>) -> HashMap<String, u16>  {

        let kmer_counts = MarkovChain::init_kmer_dict(4);

        for record in records_vec.iter() {
            for i in 0..record.seq.len() {
                let kmer = &record.seq[i..i+k];
                println!("{:?}", kmer);
            }
        }
        todo!("Finish");
    }
}