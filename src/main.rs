pub mod ipc;
pub mod osu;

use anyhow::{anyhow, Context, Result};
use cpu_endian::*;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::ipc::{deserialize_message, send_message, OsuIpcMessage, OsuResponse, ValueIpc};
use crate::osu::calculate_sr;

fn main() {
    let listener =
        TcpListener::bind("127.0.0.1:45357").expect("Failed to boot up server on osu!'s IPC port.");

    println!("Server started.");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let result = handle_connection(&stream);
        match result {
            Ok(res) => send_message(res, stream),
            Err(x) => {
                eprintln!("Failed to process osu!'s IPC message: {}", x);
                let response = OsuIpcMessage {
                    type_field: "System.Object".to_owned(),
                    value: ValueIpc {
                        message_type: "LegacyIpcDifficultyCalculationResponse".to_owned(),
                        message_data: OsuResponse { star_rating: 0.0 },
                    },
                };
                send_message(response, stream)
            }
        }
    }
}

fn handle_connection(mut stream: &TcpStream) -> Result<OsuIpcMessage<OsuResponse>> {
    // Read the header and get the size of the message
    let mut header = [0; 4];
    stream
        .read(&mut header)
        .context("Failed to read header from request")?;

    let len = match working() {
        Endian::Little => i32::from_le_bytes(header),
        Endian::Big => i32::from_be_bytes(header),
        _ => panic!("Unexpected CPU endian"),
    };

    // Initialize buffer in the same size
    let mut buffer = vec![0; len.try_into().unwrap()];
    stream
        .read(&mut buffer)
        .with_context(|| format!("Failed to read content with length {}", &len))?;

    // Then convert it to JSON string
    let json_str = String::from_utf8_lossy(&buffer.as_slice());
    let json_str = json_str.trim_matches(char::from(0));

    // If either of header or string is empty, just don't do anything
    if i32::from_le_bytes(header) == 0 || json_str.is_empty() {
        return Err(anyhow!("Malformed request from osu!"));
    }

    // Attempt to decode the message
    let deserialized =
        deserialize_message(json_str).context("Failed to deserialize osu! IPC message.")?;
    println!("Request: {:?}", deserialized);

    // Calculate the SR
    let sr = calculate_sr(deserialized).context("Failed to calculate star rating")?;
    let response = OsuIpcMessage {
        type_field: "System.Object".to_owned(),
        value: ValueIpc {
            message_type: "LegacyIpcDifficultyCalculationResponse".to_owned(),
            message_data: OsuResponse {
                star_rating: sr.to_owned(),
            },
        },
    };

    return Ok(response);
}
