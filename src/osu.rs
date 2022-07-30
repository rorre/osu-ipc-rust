use anyhow::{Context, Result};

use rosu_pp::GameMode;
use rosu_pp::{AnyStars, Beatmap};

use crate::ipc::OsuMessageData;

pub fn calculate_sr(message: OsuMessageData) -> Result<f64> {
    let map = Beatmap::from_path(&message.beatmap_file)
        .with_context(|| format!("Cannot read .osu file at {}", message.beatmap_file))?;

    let mode = match message.ruleset_id {
        0 => GameMode::Osu,
        1 => GameMode::Taiko,
        2 => GameMode::Catch,
        _ => GameMode::Mania,
    };

    let sr = AnyStars::new(&map)
        .mode(mode)
        .mods(message.mods)
        .calculate()
        .stars();

    return Ok(sr);
}
