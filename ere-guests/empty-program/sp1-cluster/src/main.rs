//! SP1 Cluster empty guest program
//!
//! This is identical to the SP1 guest program since SP1 Cluster uses the same
//! program format and runtime.

#![no_main]

sp1_zkvm::entrypoint!(main);
pub fn main() {}
