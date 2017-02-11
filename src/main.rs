use std::env as e;
use std::io;
use std::io::BufRead;

fn main() {
    let args: Vec<String>  = e::args().skip(1).collect();
    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        let x = l.unwrap();
        if x == "".to_string() {
            println!("");
        } else {
            let pieces: Vec<&str> = x.split_whitespace().collect();
            let mut output = vec![];
            if args.len() <= 0 {
                let y: Vec<String> = pieces.iter().map(|s| String::from(*s)).collect();
                println!("{}", intersperse(y, "\t".to_string()).into_iter().collect::<String>());
            } else {
                for i in &args {
                    let j = i.parse::<u32>().unwrap() - 1;
                    output.push(pieces.get(j as usize).unwrap());
                }
                let z: Vec<String> = output.iter().map(|s| String::from(**s)).collect();
                println!("{}", intersperse(z, "\t".to_string()).into_iter().collect::<String>());
            }
        }
    }
}

fn intersperse(xs: Vec<String>, seperator: String) -> Vec<String> {
    let mut result = vec![];
    let end = xs.last().unwrap();
    for x in &xs {
        result.push(x.clone());
        if x.ne(end) {
            result.push(seperator.clone())
        }
    }
    result
}
