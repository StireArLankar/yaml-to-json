use json5;
use regex::Regex;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::Path;

extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

use clap::{ArgEnum, Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Args {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    #[clap()]
    Single(Single),
}

/// Mode for converting a single file
#[derive(Parser, Debug)]
struct Single {
    /// Input file
    #[clap(short, long, parse(try_from_str=valid_ext))]
    input: String,

    /// Output file
    #[clap(short, long, parse(try_from_str=valid_ext))]
    output: String,

    #[clap(long, default_value_t = 2)]
    indent: usize,

    #[clap(long, arg_enum, default_value_t = IndentStyle::Space)]
    indent_style: IndentStyle,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum IndentStyle {
    Space,
    Tab,
}

fn valid_ext(s: &str) -> Result<String, String> {
    let ext = Path::new(s).extension().unwrap();

    match ext.to_str() {
        Some("yaml") | Some("yml") | Some("json") => Ok(s.to_owned()),
        _ => Err(format!("Invalid extension: {}", ext.to_str().unwrap())),
    }
}

fn main() {
    let args = Args::parse();

    match args.subcmd {
        SubCommand::Single(Single {
            input,
            output,
            indent,
            indent_style: _,
        }) => {
            let mut input_file = File::open(&input).unwrap();

            let mut input_data = String::new();
            input_file.read_to_string(&mut input_data).unwrap();

            let input_ext = Path::new(&input).extension().unwrap().to_str().unwrap();
            let output_ext = Path::new(&output).extension().unwrap().to_str().unwrap();

            let mut output_data = match (input_ext, output_ext) {
                ("yaml" | "yml", "json") => {
                    let value: serde_json::Value = serde_yaml::from_str(&input_data).unwrap();

                    serde_json::to_string_pretty(&value).unwrap()
                }
                ("json", "yaml" | "yml") => {
                    let value: serde_yaml::Value = json5::from_str(&input_data).unwrap();

                    serde_yaml::to_string(&value).unwrap()
                }
                _ => panic!("Unsupported extension"),
            };

            if indent != 2 {
                let rg = Regex::new(r"\n(\s*)").unwrap();
                let indent_str = format!("\n{}", "$1".repeat(indent / 2));
                output_data = rg.replace_all(&output_data, indent_str).to_string();
            }

            let new_path = Path::new(&output);

            let prefix = new_path.parent().unwrap();
            create_dir_all(prefix).unwrap();

            let mut output_file = File::create(new_path).unwrap();
            output_file.write_all(output_data.as_bytes()).unwrap();
        }
    }
}

// fn main() {
//     let obj = json!({"foo":1,"bar":2});

//     let buf = Vec::new();
//     let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
//     let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
//     obj.serialize(&mut ser).unwrap();
//     println!("{}", String::from_utf8(ser.into_inner()).unwrap());
// }

// function to replace spaces at start of string with tabs
// todo - make this work
#[allow(dead_code)]
fn replace_spaces_with_tabs(s: &str) -> String {
    let rg = Regex::new(r"^(\s*)").unwrap();

    let mut item = rg.split(s);
    let temp = item.next();

    let start = match temp {
        Some(int) => Regex::new(r"\s")
            .unwrap()
            .replace_all(int, r"\t\t\t")
            .to_string(),
        None => "".to_owned(),
    };

    format!("{}{}", start, "".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(replace_spaces_with_tabs(" hello world",), "  hello world");
    }
}
