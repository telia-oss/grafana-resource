#![deny(warnings)]
extern crate rustana;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

use std::error::Error;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
struct Version {
    pub r#ref: String,
}

#[derive(Deserialize, Debug)]
struct Source {
    pub grafana_url: String,
    pub grafana_token: String,
}

#[derive(Deserialize, Debug)]
struct InInput {
    pub source: Source,
}

#[derive(Serialize, Deserialize, Debug)]
struct InOutput {
    pub version: Version,
}

fn read_in_input() -> Result<InInput, Box<Error>> {
    // Read from stdin
    let mut input_buffer = String::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut input_buffer)?;
    // Deserialize
    let input: InInput = serde_json::from_str(&input_buffer)?;

    Ok(input)
}

fn main() {
    let in_input = read_in_input();
    if in_input.is_err() {
        eprintln!(
            "Error getting resource inInput! {:?}",
            in_input.unwrap_err()
        );
        return;
    }
    let ver = Version {
        r#ref: "ref".to_owned(),
    };

    let in_output: InOutput = InOutput { version: ver };
    println!(
        "{}",
        serde_json::to_string(&in_output).expect("error serializing in output")
    );
}
