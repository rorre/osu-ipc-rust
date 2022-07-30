use rosu_pp::GameMode;
use rosu_pp::{AnyStars, Beatmap};

use crate::ipc::OsuMessageData;

pub fn calculate_sr(message: OsuMessageData) -> f64 {
    let map = match Beatmap::from_path(message.beatmap_file) {
        Ok(map) => map,
        Err(why) => panic!("Error while parsing map: {}", why),
    };

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

    return sr;
}
