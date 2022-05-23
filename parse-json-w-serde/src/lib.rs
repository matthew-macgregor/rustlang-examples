#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use serde_json::json;

mod from_reader;

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

const DATA: &str = r#"
{
    "name": "Conan the Barbarian",
    "age": 33,
    "swords": [
        "Vengeance of Crom",
        "Bloodstorm"
    ]
}"#;

fn untyped_example() -> Result<String> {
    // Some JSON input data as a &str. Maybe this comes from the user.

    let data = DATA;
    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("{}", v["name"]);
    // Note: .as_str().unwrap() removes the extra "" from around the string. The untyped result is a Value
    // and this is a side effect of serialization to string.
    Ok(format!("{} wields his sword, {}", v["name"].as_str().unwrap(), v["swords"][0].as_str().unwrap()))
}

fn json_macro() -> String {
    let conan = json!({
        "name": "Conan the Barbarian",
        "age": 33,
        "swords": vec_of_strings!("Vengeance of Crom", "Bloodstorm")
    });

    println!("{}", conan.to_string());
    conan.to_string()
}

#[derive(Serialize, Deserialize)]
struct Conan {
    name: String,
    age: u8,
    swords: Vec::<String>
}

fn json_serialize() -> Result<String> {
    let conan = Conan {
        name: "Conan the Pirate".to_owned(),
        age: 33,
        swords: vec_of_strings!("Lightning Shard")
    };

    let j = serde_json::to_string(&conan)?;

    Ok(format!("{}", j))
}

fn json_deserialize() -> Result<String> {
    let conan_str = json_serialize()?;
    let j: Conan = serde_json::from_str(&conan_str)?;

    Ok(format!("{}", j.name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_test() {
        println!("Hello, serde!");
        let phrase = untyped_example().unwrap();
        assert_eq!(phrase, "Conan the Barbarian wields his sword, Vengeance of Crom");
    }

    #[test]
    fn json_macro_test() {
        let s = json_macro();
        assert_eq!(s, r#"{"age":33,"name":"Conan the Barbarian","swords":["Vengeance of Crom","Bloodstorm"]}"#);
    }

    #[test]
    fn json_serialize_test() {
        let s = json_serialize().unwrap();
        assert_eq!(s, r#"{"name":"Conan the Pirate","age":33,"swords":["Lightning Shard"]}"#)
    }

    #[test]
    fn json_deserialize_test() {
        let s = json_deserialize().unwrap();
        assert_eq!(s, "Conan the Pirate");
    }
}