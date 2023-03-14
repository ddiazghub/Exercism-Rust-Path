static DNA_NUCLEOTIDES: [char; 4] = [
    'G',
    'C',
    'T',
    'A'
];

static RNA_NUCLEOTIDES: [char; 4] = [
    'C',
    'G',
    'A',
    'U'
];

#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Self, usize> {
        for (i, nucleotide) in dna.chars().enumerate() {
            if !DNA_NUCLEOTIDES.contains(&nucleotide) {
                return Err(i);
            }
        }

        Ok(Dna(dna.to_string()))
    }

    pub fn into_rna(self) -> Rna {
        let rna = self.0
            .chars()
            .map(|nucleotide| match nucleotide {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                _ => 'U'
            })
            .collect();

        Rna(rna)
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (i, nucleotide) in rna.chars().enumerate() {
            if !RNA_NUCLEOTIDES.contains(&nucleotide) {
                return Err(i);
            }
        }code 

        Ok(Rna(rna.to_string()))
    }
}
