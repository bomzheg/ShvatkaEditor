use std::fs::File;
use std::io::Read;

#[path = "models/key.rs"] mod key;

fn main() -> std::io::Result<()> {
    let mut file = File::open("resources/mlp.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    drop(file);
    let parsed = json::parse(&*contents).unwrap();
    let keys = &parsed["levels"][0]["keys"];
    let type_key = &keys["type"];
    let value_key = &keys["value"];
    // let key1 = key::Key {
    //     type_: type_key.as_str().unwrap(),
    //     value: value_key.as_str().unwrap()
    // };

    println!("{} - {}", type_key, value_key);
    Ok(())
}
