use std::fs::File;
use std::io::Read;

#[path = "models/key.rs"] mod key;

fn main() -> std::io::Result<()> {
    let mut file = File::open("resources/mlp.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    drop(file);
    let parsed = json::parse(&*contents).expect("must be valid json");
    let key = &parsed["levels"][0]["keys"][0];
    let key1 = key::Key {
        type_: &key["type"].as_str().unwrap(),
        value: &key["value"].as_str().unwrap()
    };

    println!("{}", key1);
    Ok(())
}
