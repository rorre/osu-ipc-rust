use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::process;
use std::{io::Write, net::TcpStream};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OsuIpcMessage<T> {
    #[serde(rename = "Type")]
    pub type_field: String,
    #[serde(rename = "Value")]
    pub value: ValueIpc<T>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ValueIpc<T> {
    #[serde(rename = "MessageType")]
    pub message_type: String,
    #[serde(rename = "MessageData")]
    pub message_data: T,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OsuMessageData {
    #[serde(rename = "BeatmapFile")]
    pub beatmap_file: String,
    #[serde(rename = "RulesetId")]
    pub ruleset_id: u8,
    #[serde(rename = "Mods")]
    pub mods: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct OsuResponse {
    pub star_rating: f64,
}

pub fn send_message(message: OsuIpcMessage<OsuResponse>, mut stream: TcpStream) {
    // Rebuild in the same format as what osu! understands
    // First 4 bytes: header, size of data
    // Last n bytes: the data as JSON
    let json_str = serde_json::to_string(&message).unwrap();
    let arr = json_str.as_bytes();
    let len: i32 = arr.len().try_into().unwrap();

    // TODO: This would be COMPLETELY WRONG on big endian devices
    let header = len.to_le_bytes();
    stream.write(&header).unwrap();
    stream.write(&arr).unwrap();
}

pub fn deserealize_message(json_str: &str) -> Result<OsuMessageData> {
    let full_message: OsuIpcMessage<OsuMessageData> = serde_json::from_str(&json_str)
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
    let message = full_message.value.message_data;

    return Ok(message);
}
