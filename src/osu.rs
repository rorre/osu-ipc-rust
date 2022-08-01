use anyhow::{anyhow, Context, Result};

use rosu_pp::GameMode;
use rosu_pp::{AnyStars, Beatmap};

use crate::ipc::OsuMessageData;

pub fn calculate_sr(message: OsuMessageData) -> Result<f64> {
    let map = Beatmap::from_path(&message.beatmap_file)
        .with_context(|| format!("Cannot read .osu file at {}", message.beatmap_file))?;

    // https://github.com/ppy/osu/blob/master/osu.Desktop/LegacyIpc/LegacyTcpIpcProvider.cs#L101-L111
    let mode = match message.ruleset_id {
        0 => GameMode::Osu,
        1 => GameMode::Taiko,
        2 => GameMode::Catch,
        3 => GameMode::Mania,
        _ => return Err(anyhow!("Invalid ruleset id")),
    };

    let sr = AnyStars::new(&map)
        .mode(mode)
        .mods(message.mods)
        .calculate()
        .stars();

    return Ok(sr);
}
