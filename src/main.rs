use std::fs::File;

use serde_json::Value;

use crate::models::game::Game;
use crate::models::level::Level;

mod models;

fn main() -> std::io::Result<()> {
    let parsed: Value = serde_json::from_reader(
        &File::open("resources/mlp.json")?
    )?;
    let game: Game = serde_json::from_value(parsed)?;

    println!("{}\n{}", game.id, game.levels[1]);
    Ok(())
}
