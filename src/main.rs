extern crate regex;
extern crate skim;

use std::io::Cursor;
use std::io::{self, Read};
use std::process::Command;

use regex::Regex;
use skim::{Skim, SkimOptionsBuilder};

pub fn main() {
    let options = SkimOptionsBuilder::default()
        .multi(true)
        .build()
        .unwrap();

    let re = Regex::new(r"(http(?:s)://[a-zA-Z0-9_/?+&.=-]+)").unwrap();
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let lines = buffer.split("\n");

    let mut matches: Vec<&str> = Vec::new();

    for line in lines {
        for capture in re.captures_iter(line) {
            let url_match = capture.get(1).unwrap().as_str();
            matches.push(url_match);
        }
    }

    let items = matches.join("\n");
    // `run_with` would read and show items from the stream
    let selected_items = Skim::run_with(&options, Some(Box::new(Cursor::new(items))))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    for item in selected_items.iter() {
        let url = item.clone();
        Command::new("firefox")
            .arg(url.get_text())
            .output()
            .unwrap();
    }
}
