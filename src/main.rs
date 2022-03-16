use std::fs::File;

use serde_json::Value;

use crate::models::level::Level;

mod models;

fn main() -> std::io::Result<()> {
    let parsed: Value = serde_json::from_reader(
        &File::open("resources/mlp_test_text_and_gps.json")?
    )?;
    let levels = &parsed["levels"];
    let levels: Vec<Level> = serde_json::from_value(levels.clone())?;

    println!("{}\n{}", levels[0], levels[1]);
    Ok(())
}
