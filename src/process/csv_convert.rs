use std::fs;

use csv::Reader;
use serde_json::Value;

use crate::opts::OutputFormat;

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// struct Player {
//     // #[serde(rename = "Name")]
//     name: String,
//     // #[serde(rename = "Position")]
//     position: String,
//     #[serde(rename = "DOB")]
//     dob: String,
//     // #[serde(rename = "Nationality")]
//     nationality: String,
//     #[serde(rename = "Kit Number")]
//     kit_number: u8,
// }

pub fn process_csv(input: &str, output: &str, format: &OutputFormat) -> anyhow::Result<()> {
    let mut rdr = Reader::from_path(input)?;
    // ***** from for loop to map *****
    // let mut records = Vec::new();
    // for result in rdr.deserialize() {
    //     let record: Player = result.unwrap();
    //     records.push(record);
    // }
    // let records = rdr
    //     .deserialize()
    //     .map(|record| record.unwrap())
    //     .collect::<Vec<Player>>();

    // ***** more universal way with map *****
    // let headers = rdr.headers()?.clone();
    // let ret = rdr
    //     .records()
    //     .map(|record| {
    //         let record = record.unwrap();
    //         headers.iter().zip(record.iter()).collect::<Value>() // return Value type
    //     })
    //     .collect::<Value>();

    // ***** more universal way with for loop *****
    let headers = rdr.headers()?.clone();
    let mut ret = Vec::with_capacity(128);
    for record in rdr.records() {
        let record = record?;
        // headers.iter() -> use the iterator of headers
        // record.iter() -> use the iterator of record
        // zip() -> combine the two iterators into one iterator of tuples [(header, record), ...]
        // collect::<Value>() -> convert the iterator of tuples into a Value type
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }
    // let json = serde_json::to_string_pretty(&ret)?;
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        // OutputFormat::Toml => toml::to_string(&ret)?,
    };
    fs::write(output, content)?;
    Ok(())
}
