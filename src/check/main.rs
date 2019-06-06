#![deny(warnings)]
extern crate rustana;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Version {
    pub r#ref: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CheckOutput {
    pub version: Vec<Version>,
}

fn main() {
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
