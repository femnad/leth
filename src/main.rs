extern crate regex;
extern crate skim;
extern crate structopt;

use std::collections::HashMap;
use std::io::Cursor;
use std::io::{self, Read};
use std::process::Command;

use regex::Regex;
use skim::{Skim, SkimOptionsBuilder};
use structopt::StructOpt;

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

    let mut matches: HashMap<&str, u8> = HashMap::new();
    let mut match_index = 1;

    for line in lines {
        for capture in re.captures_iter(line) {
            let url_match = capture.get(1).unwrap().as_str();
            if matches.contains_key(url_match) {
                continue;
            }
            matches.insert(url_match, match_index);
            match_index += 1;
        }
    }

    let mut ordered_items: Vec<_> = matches.into_iter().collect();
    ordered_items.sort_by(|a, b| a.1.cmp(&b.1));

    let item_list: Vec<_> = ordered_items.iter().map(|item| item.0).collect();
    let items = item_list.join("\n");

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
