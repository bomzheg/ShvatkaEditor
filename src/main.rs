use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = File::open("resources/mlp.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let parsed = json::parse(&*contents).unwrap();
    println!("{}", parsed);
    Ok(())
}
