#![deny(warnings)]
extern crate rustana;

use rustana::rustana_types::Panels;
use rustana::GrafanaClient;

use std::error::Error;
use std::fs::File;
use std::io::Read;

fn print_done() {
    println!("\n\nDone.");
}

fn read_panels_from_file() -> Result<Vec<Panels>, Box<Error>> {
    // Read
    let mut file = File::open("src/panels.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // Deserialize
    let panels: Vec<Panels> = serde_json::from_str(&contents)?;

    Ok(panels)
}

fn main() {
    let url = env!("GRAFANA_URL");
    let token = env!("GRAFANA_TOKEN");
    let mut client = GrafanaClient::new(url, token);
    let panels = read_panels_from_file();
    match panels {
        Ok(panels) => {
            let _res = client.update_dashboard_by_id("<id>", panels);

            match _res {
                Ok(res) => println!("{:#?}", res),
                Err(e) => println!("error parsing the get response: {:?}", e),
            }
        }
        Err(e) => println!("error parsing the get response: {:?}", e),
    }

    print_done();
}
