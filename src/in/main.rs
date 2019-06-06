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
struct InOutput {
    pub version: Version,
}

fn main() {
    let ver = Version {
        r#ref: "ref".to_owned(),
    };

    let in_output: InOutput = InOutput { version: ver };
    println!(
        "{}",
        serde_json::to_string(&in_output).expect("error serializing in output")
    );
}
