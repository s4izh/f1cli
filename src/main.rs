use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use chrono::Datelike;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Location {
    lat: String,
    long: String,
    locality: String,
    country: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Circuit {
    #[serde(rename = "circuitId")]
    circuit_id: String,
    url: String,
    #[serde(rename = "circuitName")]
    circuit_name: String,
    #[serde(rename = "Location")]
    location: Location,
}

#[derive(Serialize, Deserialize, Debug)]
struct CircuitTable {
    #[serde(rename = "Circuits")]
    circuits: Vec<Circuit>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct Circuits {
    xmlns: String,
    series: String,
    url: String,
    limit: String,
    offset: String,
    total: String,
    #[serde(rename = "CircuitTable")]
    circuit_table: CircuitTable,
}

#[derive(Serialize, Deserialize, Debug)]
struct MRData {
    #[serde(rename = "MRData")]
    mrdata: Circuits,
}

fn main() {
    let cache_dir = match f1cli::ensure_cache_dir(Some(chrono::Utc::now().year())) {
        Ok(dir) => dir,
        Err(err) => {
            println!("Error: {}", err);
            std::process::exit(1);
        }
    };

    println!("cache_dir = {}", cache_dir);

    let circuits_url = "http://ergast.com/api/f1/circuits.json";
    let circuits_cache = "circuits.json";

    let circuits_content = f1cli::web::fetch_or_cache(circuits_url, circuits_cache, &cache_dir);

    let parsed_json = serde_json::from_str::<MRData>(&circuits_content).unwrap();

    println!("circuits = {:?}", parsed_json);

    let mut circuit_map: HashMap<String, Circuit> = HashMap::new();

    // Populate the HashMap with circuit IDs and corresponding Circuit structs
    for circuit in &parsed_json.mrdata.circuit_table.circuits {
        circuit_map.insert(circuit.circuit_id.clone(), circuit.clone());
    }

    // Now you can work with the HashMap
    // for (circuit_id, circuit) in &circuit_map {
    //     println!("Circuit ID: {}", circuit_id);
    //     println!("Circuit Name: {}", circuit.circuitName);
    //     println!("---");
    // }

    let current_year: i32 = chrono::Utc::now().year();

    let circuits_current_year_url =
        format!("http://ergast.com/api/f1/{}/circuits.json", current_year);
    let circuits_current_year_cache = format!("{}/circuits.json", current_year);

    let circuits_current_year_content = f1cli::web::fetch_or_cache(
        &circuits_current_year_url,
        &circuits_current_year_cache,
        &cache_dir,
    );

    println!("Current year circuits: {}", circuits_current_year_content);
}
