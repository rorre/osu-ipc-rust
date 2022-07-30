pub mod ipc;
pub mod osu;

use cpu_endian::*;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::ipc::{deserealize_message, send_message, OsuIpcMessage, OsuResponse, ValueIpc};
use crate::osu::calculate_sr;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:45357").unwrap();
    println!("Server started.");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let result = handle_connection(&stream);
        match result {
            Some(x) => send_message(x, stream),
            None => println!("[INFO] skipping due to empty"),
        }
    }
}

fn handle_connection(mut stream: &TcpStream) -> Option<OsuIpcMessage<OsuResponse>> {
    // Read the header and get the size of the message
    let mut header = [0; 4];
    stream.read(&mut header).unwrap();
    println!("Header: {:?}", &header);

    let len = match working() {
        Endian::Little => i32::from_le_bytes(header),
        Endian::Big => i32::from_be_bytes(header),
        _ => panic!("Unexpected CPU endian"),
    };

    // Initialize buffer in the same size
    let mut buffer = vec![0; len.try_into().unwrap()];
    stream.read(&mut buffer).unwrap();

    // Then convert it to JSON string
    let json_str = String::from_utf8_lossy(&buffer.as_slice());
    let json_str = json_str.trim_matches(char::from(0));
    println!("JsonString: {:?}", json_str);

    // If either of header or string is empty, just don't do anything
    if i32::from_le_bytes(header) == 0 || json_str.is_empty() {
        return None;
    }

    // Attempt to decode the message
    let deserialized = match deserealize_message(json_str) {
        Ok(x) => x,
        Err(_x) => return None,
    };
    println!("Request: {:?}", deserialized);

    // Calculate the SR
    let sr = calculate_sr(deserialized);
    let response = OsuIpcMessage {
        type_field: "System.Object".to_owned(),
        value: ValueIpc {
            message_type: "LegacyIpcDifficultyCalculationResponse".to_owned(),
            message_data: OsuResponse {
                star_rating: sr.to_owned(),
            },
        },
    };

    return Some(response);
}
