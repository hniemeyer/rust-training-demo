use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct InterestingObject {
    position_x: f32,
    position_y: f32,
    name: String,
    damage: Option<u32>,
    checked: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct ObjectList {
    contents: Vec<InterestingObject>,
    project_id: u32,
    creator: String,
}

fn main() {
    let data = ObjectList {
        contents: vec![
            InterestingObject {
                position_x: 12.0,
                position_y: 33.0,
                name: "Hallo".to_owned(),
                damage: None,
                checked: true,
            },
            InterestingObject {
                position_x: 22.0,
                position_y: 98.0,
                name: "Welt".to_owned(),
                damage: Some(45),
                checked: false,
            },
        ],
        project_id: 12,
        creator: "Hendrik".to_owned(),
    };

    let serialized = serde_json::to_string(&data).unwrap();
    println!("serialized = {}", serialized);
    let serialized_yaml = serde_yaml::to_string(&data).unwrap();
    println!("serialized = {}", serialized_yaml);
}
