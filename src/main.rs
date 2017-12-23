use std::env as e;
use std::io;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = e::args().skip(1).collect();
    let stdin             = io::stdin();
    if args.len() == 1 && (args[0] == "-h" ||
                           args[0] == "--help") {
        print_help();
    }

    for l in stdin.lock().lines() {
        let line = l.unwrap();
        if line == "" {
            println!();
        } else {
            let pieces: Vec<&str> = line.split_whitespace().collect();
            if args.is_empty() {
                print_all(&pieces);
            } else {
                print_selected(&args, &pieces);
            }
        }
    }
}

fn print_selected(args: &[String], pieces: &[&str]) -> () {
    let mut output = vec![];
    for i in args {
        let j = i.parse::<u32>().unwrap() - 1;
        output.push(&pieces[j as usize]);
    }
    let z: Vec<String> = output.iter().map(|s| String::from(**s)).collect();
    println!("{}", intersperse(&z, &"\t".to_string()).into_iter().collect::<String>());
}

fn print_all(pieces: &[&str]) -> () {
    let y: Vec<String> = pieces.iter().map(|s| String::from(*s)).collect();
    println!("{}", intersperse(&y, &"\t".to_string()).into_iter().collect::<String>());
}

fn print_help() -> ! {
    println!("Usage: fnl [COLUMNS...]\n\nCOLUMNS\tOptional column numbers to select.\n\nExamples:\n\n\tex:     echo 'a     b c' | fnl 2 1\n\toutput: b\ta\n\n\tex:     echo 'a     b c' | fnl \n\toutput: a\tb\tc");
    std::process::exit(1);
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
