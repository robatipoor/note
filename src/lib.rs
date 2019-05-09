
extern crate colored;


pub mod file;
pub mod note;

#[cfg(test)]
pub mod test;

pub use file::FileHandler;
pub use note::*;
