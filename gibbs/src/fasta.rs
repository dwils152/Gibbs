use std::io::{BufReader, BufRead};
use std::fs::File;
use std::error::Error;
use regex::Regex;

pub struct SequenceRecord {
    pub header: String,
    pub seq: String,
}

pub struct FastaParser;

impl FastaParser {
    pub fn parse(file_path: &str) -> Result<Vec<SequenceRecord>, Box<dyn Error>> {
        let reader: BufReader<File> = BufReader::new(File::open(file_path)?);
        let mut records = Vec::new();
        let mut current_header = String::new();
        let mut current_seq = String::new();

        for line in reader.lines() {
            let line = line?;
            if line.starts_with('>') {
                if !current_header.is_empty() {
                    records.push(SequenceRecord {
                        header: current_header.clone(),
                        seq: Self::hard_mask(current_seq.clone()),
                    });
                    current_seq.clear();
                }
                current_header = line;
            } else {
                current_seq.push_str(&line);
            }
        }

        if !current_header.is_empty() {
            records.push(SequenceRecord {
                header: current_header,
                seq: Self::hard_mask(current_seq),
            });
        }

        Ok(records)
    }

    pub fn hard_mask(seq: String) -> String {
        let soft_chars = Regex::new("[a-z]").unwrap();
        soft_chars.replace(&seq, "N").to_string()
    }

    
}