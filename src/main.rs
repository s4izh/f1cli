use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use chrono::{offset, DateTime, Datelike, TimeZone, Utc};

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
struct CircuitsJson {
    #[serde(rename = "MRData")]
    mrdata: Circuits,
}

#[derive(Serialize, Deserialize, Debug)]
struct Schedule {
    date: String,
    time: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Race {
    season: String,
    round: String,
    url: String,
    #[serde(rename = "raceName")]
    race_name: String,
    #[serde(rename = "Circuit")]
    circuit: Circuit,
    date: Option<String>,
    time: Option<String>,
    #[serde(rename = "FirstPractice")]
    first_practice: Schedule,
    #[serde(rename = "SecondPractice")]
    second_practice: Schedule,
    #[serde(rename = "ThirdPractice")]
    third_practice: Option<Schedule>,
    #[serde(rename = "Qualifying")]
    qualifying: Schedule,
    #[serde(rename = "Sprint")]
    sprint: Option<Schedule>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RaceTable {
    season: String,
    #[serde(rename = "Races")]
    races: Vec<Race>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Races {
    xmlns: String,
    series: String,
    url: String,
    limit: String,
    offset: String,
    total: String,
    #[serde(rename = "RaceTable")]
    race_table: RaceTable,
}

#[derive(Serialize, Deserialize, Debug)]
struct SeasonJson {
    #[serde(rename = "MRData")]
    mrdata: Races,
}

fn main() {
    let cache_dir = match f1rs::ensure_cache_dir(Some(Utc::now().year())) {
        Ok(dir) => dir,
        Err(err) => {
            println!("Error: {}", err);
            std::process::exit(1);
        }
    };

    // println!("cache_dir = {}", cache_dir);

    let circuits_url = "http://ergast.com/api/f1/circuits.json";
    let circuits_cache = "circuits.json";

    let circuits_content = f1rs::web::fetch_or_cache(circuits_url, circuits_cache, &cache_dir);

    let circuits_json = serde_json::from_str::<CircuitsJson>(&circuits_content).unwrap();

    // println!("{:?}", circuits_json);

    let mut circuit_map: HashMap<String, Circuit> = HashMap::new();

    // Populate the HashMap with circuit IDs and corresponding Circuit structs
    for circuit in &circuits_json.mrdata.circuit_table.circuits {
        circuit_map.insert(circuit.circuit_id.clone(), circuit.clone());
    }

    // Now you can work with the HashMap
    // for (circuit_id, circuit) in &circuit_map {
    //     println!("Circuit ID: {}", circuit_id);
    //     println!("Circuit Name: {}", circuit.circuitName);
    //     println!("---");
    // }

    let current_year: i32 = Utc::now().year();

    let circuits_current_year_url =
        format!("http://ergast.com/api/f1/{}/circuits.json", current_year);
    let circuits_current_year_cache = format!("{}/circuits.json", current_year);

    let circuits_current_year_content = f1rs::web::fetch_or_cache(
        &circuits_current_year_url,
        &circuits_current_year_cache,
        &cache_dir,
    );

    let circuits_current_year_json =
        serde_json::from_str::<CircuitsJson>(&circuits_current_year_content).unwrap();

    // println!(
    //     "Here are the circuits_id for the current year ({}):",
    //     current_year
    // );

    let mut circuits: Vec<(String, String)> = vec![];

    for circuit in &circuits_current_year_json.mrdata.circuit_table.circuits {
        circuits.push((circuit.circuit_name.clone(), circuit.circuit_id.clone()));
    }

    let _max_name_len = circuits
        .iter()
        .map(|(name, _)| name.len())
        .max()
        .unwrap_or(0);

    // for (name, id) in circuits {
    //     // Create a formatted string with the circuit name and ID
    //     let formatted_line = format!("{:<width$} : {}", name, id, width = max_name_len);

    //     // Print the formatted line
    //     println!("{}", formatted_line);
    // }

    let season_2023 = f1rs::web::fetch_or_cache(
        "http://ergast.com/api/f1/2023.json",
        "season_2023.json",
        &cache_dir,
    );

    let season_2023_json = serde_json::from_str::<SeasonJson>(&season_2023).unwrap();

    let mut races: Vec<(String, String)> = vec![];

    for race in &season_2023_json.mrdata.race_table.races {
        races.push((
            race.circuit.circuit_name.clone(),
            race.circuit.circuit_id.clone(),
        ));
    }

    let _max_name_len = races.iter().map(|(name, _)| name.len()).max().unwrap_or(0);

    //     for (i, (name, id)) in races.iter().enumerate() {
    //         // Create a formatted string with the circuit name and ID
    //         let s = if i < 10 {
    //             format!(" [{}]", i)
    //         } else {
    //             format!("[{}]", i)
    //         };
    //         let formatted_line = format!("{} {:<width$} : {}", s, name, id, width = max_name_len);

    //         // Print the formatted line
    //         println!("{}", formatted_line);
    //     }

    // print!("Schedule for Monza: ");
    // println!("Race:");
    // println!(
    //     "- {}",
    //     &season_2023_json.mrdata.race_table.races[13]
    //         .date
    //         .clone()
    //         .unwrap()
    // );
    // println!(
    //     "- {}",
    //     &season_2023_json.mrdata.race_table.races[13]
    //         .time
    //         .clone()
    //         .unwrap()
    // );

    // let date = &season_2023_json.mrdata.race_table.races[13].date;
    // let time = &season_2023_json.mrdata.race_table.races[13].time;
    // let date_time = format!("{}T{}", date.clone().unwrap(), time.clone().unwrap());

    println!("Race Schedules for {}: ", current_year);

    for race in &season_2023_json.mrdata.race_table.races {
        let utc_time = Utc
            .datetime_from_str(
                &format!(
                    "{}T{}",
                    race.date.clone().unwrap(),
                    race.time.clone().unwrap()
                ),
                "%Y-%m-%dT%H:%M:%SZ",
            )
            .unwrap();
        let madrid_time: DateTime<chrono::FixedOffset> =
            utc_time.with_timezone(&offset::FixedOffset::east_opt(2 * 3600).unwrap());

        let transformed_time = madrid_time.format("%b %d %H:%M").to_string();
        // println!("{}", transformed_time);

        let formatted_line = format!(
            "{:<width1$} {:<width2$} {}",
            race.race_name,
            race.circuit.circuit_name,
            // madrid_time,
            transformed_time,
            width1 = 30,
            width2 = 35,
        );
        println!("{}", formatted_line);
    }
}
