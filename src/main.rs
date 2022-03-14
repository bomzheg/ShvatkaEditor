mod models;

use std::fs::File;
use serde_json::Value;
use crate::models::key::Key;


fn main() -> std::io::Result<()> {
    let parsed: Value = serde_json::from_reader(&File::open("resources/mlp.json")?)?;
    let key = &parsed["levels"][0]["keys"][0];
    let key1: Key = serde_json::from_value(key.clone())?;

    println!("{}", key1);
    Ok(())
}
