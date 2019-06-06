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
struct CheckInput {
    pub source: Source,
}

#[derive(Serialize, Deserialize, Debug)]
struct CheckOutput {
    pub version: Vec<Version>,
}

fn read_check_input() -> Result<CheckInput, Box<Error>> {
    // Read from stdin
    let mut input_buffer = String::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut input_buffer)?;
    // Deserialize
    let input: CheckInput = serde_json::from_str(&input_buffer)?;

    Ok(input)
}

fn main() {
    let check_input = read_check_input();
    if check_input.is_err() {
        eprintln!(
            "Error getting resource checkInput! {:?}",
            check_input.unwrap_err()
        );
        return;
    }
    let ver = Version {
        r#ref: "ref".to_owned(),
    };
    let mut version = Vec::new();
    version.push(ver);

    let check_output: CheckOutput = CheckOutput { version: version };
    println!(
        "{}",
        serde_json::to_string(&check_output).expect("error serializing check output")
    );
}
