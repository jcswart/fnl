use std::env as e;
use std::io;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = e::args().skip(1).collect();
    let stdin             = io::stdin();
    if args.len() == 1 && (args[0] == "-h".to_string()   ||
                           args[0] == "--help".to_string()) {
        print_help();
    }

    for l in stdin.lock().lines() {
        let line = l.unwrap();
        if line == "".to_string() {
            println!("");
        } else {
            let pieces: Vec<&str> = line.split_whitespace().collect();
            if args.len() <= 0 {
                print_all(pieces);
            } else {
                print_selected(&args, pieces);
            }
        }
    }
}

fn print_selected(args: &Vec<String>, pieces: Vec<&str>) -> () {
    let mut output = vec![];
    for i in args {
        let j = i.parse::<u32>().unwrap() - 1;
        output.push(pieces.get(j as usize).unwrap());
    }
    let z: Vec<String> = output.iter().map(|s| String::from(**s)).collect();
    println!("{}", intersperse(z, "\t".to_string()).into_iter().collect::<String>());
}

fn print_all(pieces: Vec<&str>) -> () {
    let y: Vec<String> = pieces.iter().map(|s| String::from(*s)).collect();
    println!("{}", intersperse(y, "\t".to_string()).into_iter().collect::<String>());
}

fn print_help() -> ! {
    println!("Usage: fnl [COLUMNS...]\n\nCOLUMNS\tOptional column numbers to select.\n\nExamples:\n\n\tex:     echo 'a     b c' | fnl 2 1\n\toutput: b\ta\n\n\tex:     echo 'a     b c' | fnl \n\toutput: a\tb\tc");
    std::process::exit(1);
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
