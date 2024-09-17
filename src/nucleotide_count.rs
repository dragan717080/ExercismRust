use std::collections::HashMap;

fn main() {
    let s = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";

    println!("{:?}", nucleotide_counts(s));
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut count = 0;

    if !['A', 'C', 'G', 'T'].contains(&&nucleotide) {
        return Err(nucleotide);
    }

    for c in dna.chars() {
        if !['A', 'C', 'G', 'T'].contains(&c) {
            return Err(c);
        }

        if c == nucleotide {
            count += 1;
        }
    }

    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::from([
        ('A', 0),
        ('C', 0),
        ('G', 0),
        ('T', 0),
    ]);

    for c in dna.chars() {
        if !['A', 'C', 'G', 'T'].contains(&c) {
            return Err(c);
        }

        *counts.entry(c).or_insert(0) += 1;
    }

    Ok(counts)
}
