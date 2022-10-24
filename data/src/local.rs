use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::{chemicals::*, sql::{add_reaction, add_reactions, get_reactions}};
use serde_json;

pub fn serialize(compounds: &Data, serialize_path: String) {
    let data = serde_json::to_string(compounds).unwrap();

    let mut file = File::create(serialize_path).expect("Error while creating file");

    write!(file, "{}", data).expect("Failed to write to file");
}

pub fn serialize_to_sql(compounds: Vec<Reaction>) {
    add_reactions(add_reaction(compounds));
}

pub fn deserialize_from_sql() -> Vec<Reaction> {
    get_reactions().unwrap()
}

pub fn deserialize(deserialize_path: String) -> Vec<Reaction> {
    let file = fs::read_to_string(deserialize_path).expect("cannot read file");
    match serde_json::from_str(file.as_str()) {
        Ok(result) => return result,
        Err(e) => panic!("Failed to deserialize data: {}", e),
    }
}

pub fn data_exists(path: &String) -> bool {
    let cur_path = Path::new(path);
    cur_path.exists()
}
