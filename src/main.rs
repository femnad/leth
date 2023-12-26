extern crate regex;
extern crate skim;
extern crate structopt;

use std::collections::HashMap;
use std::io::Cursor;
use std::io::{self, Read};
use std::process::{Command, Stdio};

use regex::Regex;
use skim::prelude::*;
use structopt::StructOpt;

const LINE_SPLITTER: char = '=';
const URL_REGEX: &str = r"(http(s)?://[a-zA-Z0-9_/?+&.=@%#;~:-]+)";

#[derive(Debug, StructOpt)]
#[structopt(name = "leth", about = "URL extractor intended to be used within mutt")]
struct Opt {}

pub fn main() {
    Opt::from_args();

    let options = SkimOptionsBuilder::default()
        .multi(true)
        .bind(vec!["ctrl-k:kill-line"])
        .build()
        .unwrap();

    let re = Regex::new(URL_REGEX).unwrap();
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let lines = buffer.split("\n");

    let mut split_lines = false;
    let mut split_line_buffer: Vec<&str> = Vec::new();
    let mut merged_lines: Vec<String> = Vec::new();
    for line in lines {
        if line.len() == 0 {
            continue
        }

        if line.ends_with(LINE_SPLITTER) {
            let mergable = line.get(0..line.len() - 1).unwrap_or("");
            split_line_buffer.push(mergable);
            split_lines = true;
            continue;
        }

        if split_lines {
            split_lines = false;
            split_line_buffer.push(line);
            let merged_line = &split_line_buffer.join("");
            merged_lines.push(merged_line.to_string());
            split_line_buffer = Vec::new();
        } else {
            merged_lines.push(line.to_string());
        }
    }

    let mut matches: HashMap<String, u8> = HashMap::new();
    let mut match_index = 1;
    for line in merged_lines {
        for capture in re.captures_iter(&line) {
            let url_match = capture.get(1).unwrap().as_str();
            if matches.contains_key(url_match) {
                continue;
            }
            matches.insert(url_match.to_string(), match_index);
            match_index += 1;
        }
    }

    let mut ordered_items: Vec<_> = matches.into_iter().collect();
    ordered_items.sort_by(|a, b| a.1.cmp(&b.1));

    let item_list: Vec<_> = ordered_items.iter().map(|item| item.0.as_str()).collect();
    let items = item_list.join("\n");

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(items));

    let output = Skim::run_with(&options, Some(items)).unwrap();
    if output.is_abort {
        return;
    }

    for item in output.selected_items.iter() {
        let url = item.clone();
        Command::new("firefox")
            .arg(url.output().as_ref())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
    }
}
