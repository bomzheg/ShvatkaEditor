use std::fs::File;
use serde_json::Value;
use crate::key::Key;

#[path = "models/key.rs"] mod key;

fn main() -> std::io::Result<()> {
    let file = File::open("resources/mlp.json")?;
    let parsed: Value = serde_json::from_reader(&file).expect("must be valid json");
    drop(file);
    let key = &parsed["levels"][0]["keys"][0];
    let key1: Key = serde_json::from_value(key.clone()).expect("not contains key");

    println!("{}", key1);
    Ok(())
}
