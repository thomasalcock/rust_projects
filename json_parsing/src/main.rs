use serde::{ser::Error, Deserialize, Serialize};
use serde_aux::prelude::*;
use serde_json::Value;

#[derive(Deserialize, Serialize)]
struct User {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    fingerprint: u32,
    location: String,
}

#[derive(Deserialize, Serialize)]
struct User2 {
    fingerprint: u32,
    location: String,
}

impl User {
    fn print(&self) {
        println!("{} {}", self.fingerprint, self.location)
    }
}

impl User2 {
    fn print(&self) {
        println!("{} {}", self.fingerprint, self.location)
    }
}

fn main() -> Result<(), serde_json::Error> {
    // The type of `j` is `&str`
    let json_text = "
        {
            \"fingerprint\": \"1\",
            \"location\": \"Menlo Park, CA\"
        }";

    let user: User = serde_json::from_str(json_text).unwrap();
    let value: Value = serde_json::from_str(json_text).unwrap();

    let user2: User2 = User2 {
        fingerprint: "43".parse().unwrap(),
        location: "Berlin".to_string(),
    };

    println!("{}", value.to_string());
    // let user3: User2 = User2 {
    //     fingerprint: value["fingerprint"].as_object().unwrap(),
    //     location: value["location"].as_object().unwrap(),
    // };

    //println!("{}, {}", value["fingerprint"], value["location"]);
    user.print();
    user2.print();

    Ok(())
}
