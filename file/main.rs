// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod models;
extern crate serde;
extern crate serde_xml_rs;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct XmlConfig {
    #[serde(rename(deserialize = "deserialize_name", serialize = "serialize_name"))]
    name: String,
    #[serde(rename(deserialize = "deserialize_id", serialize = "serialize_id"))]
    id: i32,
    #[serde(rename(deserialize = "deserialize_score", serialize = "serialize_score"))]
    score: i32,
    #[serde(rename(deserialize = "country", deserialize = "province", deserialize = "city", deserialize = "block"))]
    home: String
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("https://www.39kan.com/api.php/provide/vod/at/xml")
        .await?
        .text()
        .await?;

    println!("body = {:?}", body);

    Ok(())
}


