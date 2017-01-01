use std::env as e;
use std::io;
use std::io::BufRead;
use StackOp::*;

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
    let cols = process_args(e::args().skip(1).collect());

    //
    // Iterate over stdin.
    //
    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        let line              = l.unwrap();
        let pieces: Vec<&str> = line.split_whitespace().collect();
        let mut tmp:    Vec<&str> = vec![];
        // for c in &cols {
        //     println!("c: {:?}", c);
        //     match *c {
        //         Column(i) => {
        //             let   idx = (i - 1) as usize;
        //             let   x   = pieces.get(idx);
        //             match x {
        //                 None    => {}
        //                 Some(m) => { tmp.push(m) },
        //             }
        //         },
        //         _ => {}
        //     }
        // }
        let output: String = intersperse_tab(tmp).into_iter().collect();
        println!("{}", output);
    }
}

#[derive(Debug,PartialEq,Clone)]
struct StackExpr {
    ops: Vec<StackOp>
}

#[derive(Debug,PartialEq,Clone)]
enum StackOp {
    Word(String),
    Column(u32),
    Number(u32),
}

fn process_args(args: Vec<String>) -> Vec<StackExpr> {
    let mut results = vec![];
    let mut tmp     = vec![];
    let mut in_expr = false;
    for a in args {
        let   chars: Vec<char> = a.chars().collect();
        match chars[0] {
            // This arg is a simple expression.
            '%' => {
                let op = Column(chars[1].to_digit(10).unwrap());
                if in_expr {
                    tmp.push(op)
                } else {
                    let expr = StackExpr::simple(op);
                    results.push(expr);
                }
            },
            // This arg begins a StackExpr
            '[' => { in_expr = true; }
            // This arg ends a StackExpr
            ']' => {
                in_expr = false;
                results.push(StackExpr{ops: tmp.clone()});
                tmp.clear();
            },
            // This arg is a number.
            '0'...'9' => {
                let num = Number(a.parse::<u32>().unwrap());
                if in_expr {
                    tmp.push(num)
                } else {
                    results.push(StackExpr::simple(num))
                }
            },
            // This arg could be anything, make it a word. We will look the
            // word up later to see if it is valid when we eval.
            _   => {
                let op = Word(a);
                if in_expr {
                    tmp.push(op)
                } else {
                    results.push(StackExpr::simple(op))
                }
            }
        };
    }
    println!("r -> {:?}", results);
    results
}

impl StackExpr {
    fn simple(op: StackOp) -> StackExpr {
        StackExpr{ ops: vec![op]}
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

fn str(lit: &str) -> String {
    String::from(lit)
}

#[test]
fn test_process_args_1() {
    let args = vec![str("%1"), str("%2")];
    let t1   = StackExpr::simple(Column(1));
    let t2   = StackExpr::simple(Column(2));
    assert_eq!(vec![t1,t2], process_args(args));
}

#[test]
fn test_process_args_2() {
    let args = vec![str("%1"), str("["), str("%2"), str("]")];
    let t1   = StackExpr::simple(Column(1));
    let t2   = StackExpr::simple(Column(2));
    assert_eq!(vec![t1,t2], process_args(args));
}

#[test]
fn test_process_args_3() {
    let args = vec![str("%1"), str("["), str("%2"), str("100"), str("add"), str("]")];
    let t1   = StackExpr::simple(Column(1));
    let t2   = StackExpr{ops: vec![Column(2), Number(100), Word(str("add"))]};
    assert_eq!(vec![t1,t2], process_args(args));
}
