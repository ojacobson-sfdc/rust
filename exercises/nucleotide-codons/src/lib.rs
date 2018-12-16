// This exercise is deprecated.
// Consider working on protein-translation instead.

use std::marker::PhantomData;

pub struct CodonsInfo<'a> {
    // This field is here to make the template compile and not to
    // complain about unused type lifetime parameter "'a". Once you start
    // solving the exercise, delete this field and the 'std::marker::PhantomData'
    // import.
    phantom: PhantomData<&'a ()>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Result<&'a str, ()> {
        unimplemented!(
            "Return the protein name for a '{}' codon or Err, if codon string is invalid",
            codon
        );
    }

    pub fn of_rna(&self, rna: &str) -> Result<Vec<&'a str>, ()> {
        unimplemented!("Return a list of protein names that correspond to the '{}' RNA string or Err if the RNA string is invalid", rna);
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    unimplemented!(
        "Construct a new CodonsInfo struct from given pairs: {:?}",
        pairs
    );
}
