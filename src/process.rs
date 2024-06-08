use std::fs;

use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    // #[serde(rename = "Name")]
    name: String,
    // #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    // #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit_number: u8,
}

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut rdr = Reader::from_path(input)?;
    // let mut records = Vec::new();
    // for result in rdr.deserialize() {
    //     let record: Player = result.unwrap();
    //     records.push(record);
    // }
    let records = rdr
        .deserialize()
        .map(|record| record.unwrap())
        .collect::<Vec<Player>>();
    // println!("{:?}", records);
    let json = serde_json::to_string_pretty(&records)?;
    fs::write(output, json)?;
    Ok(())
}
