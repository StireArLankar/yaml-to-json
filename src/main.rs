use clap::{ArgEnum, Parser, Subcommand};
use itertools::Itertools;
use json5;
use regex::Regex;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use wax::Glob;

extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Args {
    #[clap(subcommand)]
    subcmd: SubCommand,

    /// delete source file or not
    #[clap(global = true, short, long)]
    prune: bool,

    #[clap(global = true, long, default_value_t = 2)]
    indent: usize,

    #[clap(global = true, long, arg_enum, default_value_t = IndentStyle::Space)]
    indent_style: IndentStyle,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    #[clap()]
    Single(Single),

    #[clap()]
    Dir(Directory),
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
}

/// Mode for converting a all files in directory
#[derive(Parser, Debug)]
struct Directory {
    /// root directory
    #[clap(short, long)]
    root: String,

    /// all extensions to be converted
    #[clap(arg_enum, short, long, multiple_occurrences(true))]
    input: Vec<Extensions>,

    /// resulting extension
    #[clap(arg_enum, short, long)]
    output: Extensions,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Hash)]
enum Extensions {
    Yaml,
    Yml,
    Json,
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

    let Args {
        indent,
        indent_style,
        subcmd,
        prune,
    } = args;

    match subcmd {
        SubCommand::Single(Single { input, output }) => {
            convert_file(indent_style, &input, &output, indent);

            if prune && input != output {
                std::fs::remove_file(input).unwrap();
            }
        }

        SubCommand::Dir(Directory {
            input,
            output: out_ext,
            root,
        }) => {
            let ext = input
                .iter()
                .unique()
                .map(|x| extension_to_string(x))
                .join(",");

            let files = get_files(&ext, &root);

            files.iter().for_each(|file| {
                let input = file.to_str().unwrap();
                let base_ext = file.extension().unwrap().to_str().unwrap();
                let output = input[..input.len() - base_ext.len()].to_owned()
                    + &extension_to_string(&out_ext);

                convert_file(indent_style, input, &output, indent);

                if prune && input != output {
                    std::fs::remove_file(file).unwrap();
                }
            })
        }
    }
}

fn extension_to_string(x: &Extensions) -> String {
    match x {
        Extensions::Yaml => "yaml".to_owned(),
        Extensions::Yml => "yml".to_owned(),
        Extensions::Json => "json".to_owned(),
    }
}

fn convert_file(_: IndentStyle, input: &str, output: &str, indent: usize) {
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
        ("yaml" | "yml", "yaml" | "yml") => {
            let value: serde_yaml::Value = serde_yaml::from_str(&input_data).unwrap();

            serde_yaml::to_string(&value).unwrap()
        }
        ("json", "json") => {
            let value: serde_json::Value = json5::from_str(&input_data).unwrap();

            serde_json::to_string(&value).unwrap()
        }
        t => panic!("Unsupported extension {:?}", t),
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

// function that takes file extension and root directory
// and returns a list of files by glob in the directory
// with the given extension
fn get_files(ext: &str, root: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();

    let expression = format!("**/*.{{{}}}", ext);

    let glob = Glob::new(&expression).unwrap();

    for entry in glob.walk(root, usize::MAX) {
        let entry = entry.unwrap();

        files.push(entry.path().to_owned());
    }

    files
}

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
        // assert_eq!(replace_spaces_with_tabs(" hello world",), "  hello world");
        assert_eq!(replace_spaces_with_tabs(" hello world",), "  hello world");
    }
}
