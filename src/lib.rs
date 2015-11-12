extern crate num;
extern crate edit_distance;

#[cfg(feature = "stemming")]
extern crate stem;

pub mod distance;
pub mod ngram;
pub mod tokenize;
pub mod phonetics;
pub mod classifier;
