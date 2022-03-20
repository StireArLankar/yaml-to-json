use std::env;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::Path;

extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

fn main() {
    let options: Vec<String> = env::args().skip(1).collect();

    println!("{}", options[1]);

    let mut file = File::open(&options[0]).expect("Could not load file");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Could not read file contents");

    let json_value: serde_json::Value = serde_yaml::from_str(&contents).unwrap();

    let res = serde_json::to_string_pretty(&json_value).unwrap();

    println!("{}", res);

    let new_path = Path::new(&options[1]);
    let temp = new_path.display();

    println!("{:?}", temp);

    let prefix = new_path.parent().unwrap();
    create_dir_all(prefix).unwrap();

    let mut file = File::create(new_path).unwrap();

    writeln!(&mut file, "{}", res).unwrap();
}
