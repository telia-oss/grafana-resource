#![deny(warnings)]
extern crate rustana;
extern crate serde;
extern crate serde_json;

use rustana::rustana_types::MutateDashboardResponse;
use rustana::rustana_types::Panels;
use rustana::GrafanaClient;
use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fs::File;
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
struct Params {
    pub dashboard_id: String,
    pub panels: String,
}

#[derive(Deserialize, Debug)]
struct OutInput {
    pub source: Source,
    pub params: Params,
}

#[derive(Serialize, Deserialize, Debug)]
struct OutOutput {
    pub version: Version,
    pub metadata: MutateDashboardResponse,
}

fn read_panels_from_file(path: String, dir: String) -> Result<Vec<Panels>, Box<Error>> {
    let file_path = format!("{}/{}", dir, path);
    // Read from file
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // Deserialize
    let panels: Vec<Panels> = serde_json::from_str(&contents)?;

    Ok(panels)
}

fn read_out_input() -> Result<OutInput, Box<Error>> {
    // Read from stdin
    let mut input_buffer = String::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut input_buffer)?;
    // Deserialize
    let input: OutInput = serde_json::from_str(&input_buffer)?;

    Ok(input)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let ref dir = &args[1];

    let out_input = read_out_input();
    if out_input.is_err() {
        eprintln!(
            "Error getting resource outInput! {:?}",
            out_input.unwrap_err()
        );
        return;
    }
    let input = out_input.unwrap();

    let params = input.params;
    let source = input.source;
    let url = source.grafana_url;
    let token = source.grafana_token;
    let dashboard_id = params.dashboard_id;
    let panels = params.panels;

    let mut client = GrafanaClient::new(&url, &token);
    match read_panels_from_file(panels, dir.to_string()) {
        Ok(panels) => match client.update_dashboard_by_id(&dashboard_id, panels) {
            Ok(res) => {
                let ver = Version {
                    r#ref: dashboard_id,
                };
                let out_output: OutOutput = OutOutput {
                    version: ver,
                    metadata: res,
                };
                println!(
                    "{}",
                    serde_json::to_string(&out_output).expect("error serializing output")
                );
            }
            Err(e) => eprintln!("error updating dashboard: {:?}", e),
        },
        Err(e) => eprintln!("error reading dashboard panels: {:?}", e),
    }
}
