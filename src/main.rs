#[macro_use]
extern crate clap;

use std::env as e;
use std::io;
use std::io::BufRead;

use clap::App;


fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let delim: &str = matches.value_of("delimiter").unwrap_or("\t");
    let columns: Vec<&str> = matches.values_of("columns").unwrap_or(clap::Values::default()).collect();

    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        if line == "" {
            println!();
        } else {
            let pieces: Vec<&str> = line.split(delim).collect();
            if columns.is_empty() {
                print_all(&pieces);
            } else {
                print_selected(&columns, &pieces);
            }
        }
    }
}

fn print_selected(columns: &Vec<&str>, pieces: &[&str]) -> () {
    let mut output = vec![];
    for i in columns {
        let j = i.parse::<u32>().unwrap() - 1;
        if j < pieces.len() as u32 {
            output.push(&pieces[j as usize]);
        };
    }
    let z: Vec<String> = output.iter().map(|s| String::from(**s)).collect();
    if ! z.is_empty() {
        println!("{}", intersperse(&z, &"\t".to_string()).into_iter().collect::<String>());
    }
}

fn print_all(pieces: &[&str]) -> () {
    let y: Vec<String> = pieces.iter().map(|s| String::from(*s)).collect();
    println!("{}", intersperse(&y, &"\t".to_string()).into_iter().collect::<String>());
}

fn intersperse<'a>(xs: &'a [String], seperator: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = vec![];
    let end = xs.last().unwrap();
    for x in xs.iter() {
        result.push(x);
        if x.ne(end) {
            result.push(seperator)
        }
    }
    result
}
