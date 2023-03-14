use std::collections::HashMap;

pub struct StrChunks<'a> {
    slice: &'a str,
    chunk_size: usize
}

impl<'a> StrChunks<'a> {
    pub fn new(slice: &'a str, chunk_size: usize) -> Self {
        Self {
            slice,
            chunk_size
        }
    }
}

impl<'a> Iterator for StrChunks<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        (self.slice.len() > 0)
            .then(|| {
                let (chunk, new_slice) = match self.slice.char_indices().skip(self.chunk_size).next() {
                    None => (&self.slice[..], ""),
                    Some((i, _)) => (&self.slice[..i], &self.slice[i..])
                };

                self.slice = new_slice;
                chunk
            })
    }
}

pub trait StrChunksIterator<'a> {
    fn chunks(&'a self, chunk_size: usize) -> StrChunks<'a>;
}

impl<'a> StrChunksIterator<'a> for str {
    fn chunks(&'a self, chunk_size: usize) -> StrChunks<'a> {
        StrChunks::new(self, chunk_size)
    }
}

pub struct CodonsInfo<'a>(HashMap<&'a str, &'a str>);

impl<'a> CodonsInfo<'a> {
    pub fn new(pairs: Vec<(&'a str, &'a str)>) -> Self {
        Self(pairs.into_iter().collect())
    }

    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.0.get(codon).copied()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        rna.chunks(3)
            .take_while(|codon| match self.0.get(codon) {
                Some(&protein) => !protein.starts_with("stop"),
                None => true
            })
            .map(|codon| self.0.get(codon).copied())
            .collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo::new(pairs)
}
