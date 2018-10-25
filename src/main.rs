#[macro_use]

// The JSON Schema -> Types uses serde, to use the macros that this generates
// we have to declare that this entire bin will use macros
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::io::{self, Read};

// Contains the incoming JSON schema -> Types mapping
mod dsl;

fn main() {
    // Grab the incoming JSON
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer).unwrap();

    let danger_dsl: dsl::Schema = serde_json::from_str(&buffer).unwrap();
    let created = danger_dsl.danger.danger.git.created_files;

    println!("New Files! {:#?}", created);
}
