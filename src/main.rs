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

    let ext = Path::new(&options[0]).extension().unwrap();

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Could not read file contents");

    let res = match ext.to_str() {
        Some("yaml") | Some("yml") => {
            let value: serde_json::Value = serde_yaml::from_str(&contents).unwrap();

            serde_json::to_string_pretty(&value).unwrap()
        }
        Some("json") => {
            let value: serde_yaml::Value = serde_json::from_str(&contents).unwrap();

            serde_yaml::to_string(&value).unwrap()
        }
        _ => panic!("You forgot to specify this case!"),
    };

    println!("{}", res);

    let new_path = Path::new(&options[1]);

    println!("{:?}", new_path.display());

    let prefix = new_path.parent().unwrap();
    create_dir_all(prefix).unwrap();

    let mut file = File::create(new_path).unwrap();

    writeln!(&mut file, "{}", res).unwrap();
}

// fn main() {
//     let obj = json!({"foo":1,"bar":2});

//     let buf = Vec::new();
//     let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
//     let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
//     obj.serialize(&mut ser).unwrap();
//     println!("{}", String::from_utf8(ser.into_inner()).unwrap());
// }
