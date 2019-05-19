#![deny(warnings)]
extern crate rustana;
extern crate serde;
extern crate serde_json;

use rustana::rustana_types::Panels;
use rustana::rustana_types::MutateDashboardResponse;
use rustana::GrafanaClient;
use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fs::File;
use std::io::Read;


#[derive(Deserialize, Debug)]
struct Source {
    pub grafana_url: String,
    pub grafana_token: String
}
#[derive(Deserialize, Debug)]
struct Params {
    pub dashboard_id: String
}
#[derive(Deserialize, Debug)]
struct OutInput {
    pub source: Source,
    pub params: Params,
}

#[derive(Serialize, Deserialize, Debug)]
struct OutOutput {
    pub version: i64,
    pub meta: MutateDashboardResponse
}

fn print_done() {
    println!("\n\nDone.");
}

fn read_panels_from_file() -> Result<Vec<Panels>, Box<Error>> {
    // Read from file
    let mut file = File::open("src/panels.json")?;
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
    let input_output = read_out_input();
    if input_output.is_err() {
        println!("Error getting resource outInput! {:?}", input_output.unwrap_err());
        return;
    }
    let input = input_output.unwrap();
    

    let params = input.params;
    let source = input.source;
    let url = source.grafana_url;
    let token = source.grafana_token;
    let dashboard_id = params.dashboard_id;

    let mut client = GrafanaClient::new(&url, &token);
    match read_panels_from_file() {
        Ok(panels) => {
            match client.update_dashboard_by_id(&dashboard_id, panels) {
                Ok(res) => {
                    let out_output: OutOutput = OutOutput {
                        version: res.id,
                        meta: res
                    };
                    println!("{:?}", out_output);
                },
                Err(e) => println!("error updating dashboard: {:?}", e),
            }
        }
        Err(e) => println!("error reading dashboard panels: {:?}", e),
    }
    print_done();
}
