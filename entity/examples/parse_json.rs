use entity::record::Record;
use serde_json::from_str;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} <json>", args[0]);
        std::process::exit(1);
    }
    let file = std::fs::read_to_string(&args[1]).unwrap();
    let records: Vec<Record> = from_str(&file).unwrap();
    println!("{} records loaded.", records.len());
}
