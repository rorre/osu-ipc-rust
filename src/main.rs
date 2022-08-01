pub mod ipc;
pub mod osu;
pub mod update;

use anyhow::{anyhow, Context, Result};
use cpu_endian::*;
use log::*;
use simplelog::*;

use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::ipc::{deserialize_message, send_message, OsuIpcMessage, OsuResponse, ValueIpc};
use crate::osu::calculate_sr;
use crate::update::update_check;

fn main() {
    CombinedLogger::init(vec![
        SimpleLogger::new(LevelFilter::Info, Config::default()),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create("osu-ipc-rust.log").unwrap(),
        ),
    ])
    .expect("Cannot set up logger.");

    update_check().expect("Failed to check for updates.");

    let listener =
        TcpListener::bind("127.0.0.1:45357").expect("Failed to boot up server on osu!'s IPC port.");

    info!("Server started.");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        info!("osu! IPC message received.");
        let result = handle_connection(&stream);
        match result {
            Ok(res) => send_message(res, stream),
            Err(x) => error!("Failed to process osu!'s IPC message: {}", x),
        }
    }
}

fn handle_connection(mut stream: &TcpStream) -> Result<OsuIpcMessage<OsuResponse>> {
    // Read the header and get the size of the message

    // Length of header is based on the size of C#'s int
    // https://github.com/ppy/osu-framework/blob/master/osu.Framework/Platform/TcpIpcProvider.cs#L183-L185
    let mut header = [0; 4];
    let consumed = stream
        .read(&mut header)
        .context("Failed to read header from request")?;

    if consumed != 4 {
        return Err(anyhow!("Header does not match expected size"));
    }

    let len = match working() {
        Endian::Little => i32::from_le_bytes(header),
        Endian::Big => i32::from_be_bytes(header),
        _ => panic!("Unexpected CPU endian"),
    };

    // Ignore if the message is empty
    if len == 0 {
        return Err(anyhow!("Message length is 0"));
    }

    // Initialize buffer in the same size
    let mut buffer = vec![0; len.try_into().unwrap()];
    stream
        .read(&mut buffer)
        .with_context(|| format!("Failed to read content with length {}", &len))?;

    // Then convert it to JSON string
    let json_str = String::from_utf8_lossy(&buffer.as_slice());

    // Attempt to decode the message
    let deserialized =
        deserialize_message(&json_str).context("Failed to deserialize osu! IPC message.")?;
    debug!("Request: {:?}", deserialized);

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
