//! Hello world: build a univrs model in Rust and print its JSON serialization.

use univrs::{Allele, SequenceLocation};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let allele = Allele {
        location: SequenceLocation { sequence_id: "NC_000001.11".into(), start: 100, end: 101 },
        state: "T".parse()?,
    };
    println!("{}", serde_json::to_string(&allele)?);
    Ok(())
}
