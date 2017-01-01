use std::env as e;
use std::io;
use std::io::BufRead;
use std::io::Write;

fn main() {
    //
    // Check the args.
    //
    if e::args().len() == 1 {
        let basename = e::args().next().unwrap();
        let usage    = format!("Usage: {} [ COLUMNS... ]\n\nFunnel columns of data from stdin.\n\tCOLUMNS\t%1, %2, etc. Like awk: {{ print $1, $2; }}\n\n\tEx: {0} %1 %2", basename);
        println!("{}",usage);
        std::process::exit(1);
    }

    //
    // Process the args.
    //
    let mut cols = vec![];
    for a in e::args() {
        let   chars: Vec<char> = a.chars().collect();
        match chars[0] {
            '%' => { cols.push(chars[1].to_digit(10).unwrap()); },
            _   => {}
        };
    }

    //
    // Iterate over stdin.
    //
    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        let line              = l.unwrap();
        let pieces: Vec<&str> = line.split_whitespace().collect();
        let mut tmp:    Vec<&str> = vec![];
        for c in &cols {
            let   idx = (*c - 1) as usize;
            let   x   = pieces.get(idx);
            match x {
                None    => {}
                Some(m) => { tmp.push(m) },
            }
        }
        let output: String = intersperse_tab(tmp).into_iter().collect();
        println!("{}", output);
    }
}

fn intersperse_tab(xs: Vec<&str>) -> Vec<String> {
    let mut zs: Vec<String> = vec![];
    let mut coll            = xs.iter().peekable();
    let mut not_first       = false;

    loop {
        {// limit the scope of the .peek() so i can borrow w/ .next()
            let next = coll.peek();
            if  next.is_some() && not_first {
                zs.push("\t ".to_string());
            } else {
                not_first = true;
            }
        }
        match coll.next() {
            None    => { break },
            Some(v) => { zs.push(v.to_string()); }
        }
    }
    zs
}
