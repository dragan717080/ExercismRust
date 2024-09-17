use std::collections::HashMap;

fn main() {
    let input = "ACGTGGTCTTAA";

    println!("{:?}", Dna::new(input).unwrap());
}

#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    chain: String
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (i, c) in dna.char_indices() {
            if !"CTGA".contains(c) {
                return Err(i);
            }
        }

        Ok(Dna { chain: dna.to_string() })
    }

    pub fn into_rna(self) -> Rna {
        let mut values_map: HashMap<char, char> = HashMap::new();
        values_map.insert('G', 'C');
        values_map.insert('C', 'G');
        values_map.insert('T', 'A');
        values_map.insert('A', 'U');

        let mut rna_chain = String::new();

        for (_, c) in self.chain.char_indices() {
            rna_chain += &values_map.get(&c).unwrap().to_string();
        }

        Rna::new(&rna_chain).unwrap()
    }
}

/**
 * Given a DNA strand, its transcribed RNA strand is formed by replacing each nucleotide with its complement:
  *  G -> C
  *  C -> G
  *  T -> A
  *  A -> U
 */
#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    chain: String
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (i, c) in rna.char_indices() {
            if !"ACGU".contains(c) {
                return Err(i);
            }
        }

        Ok(Rna { chain: rna.to_string() })
    }
}
