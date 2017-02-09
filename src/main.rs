use std::env as e;
use std::io;
use std::io::BufRead;
use StackOp::*;

fn main() {
    if e::args().len() == 1 {
        let basename = e::args().next().unwrap();
        let usage    = format!("Usage: {} [ COLUMNS... ]\n\nFunnel columns of data from stdin.\n\tCOLUMNS\t1, 2, etc. Like awk: {{ print $1, $2; }}\n\n\tEx: {0} 1 2", basename);
        println!("{}",usage);
        std::process::exit(1);
    }

    let exprs = process_args(e::args().skip(1).collect());
    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        let output = process_line(l.unwrap(), &exprs);
        println!("{}", output);
    }
}

fn process_line(line: String, exprs: &Vec<StackExpr>) -> String {
    let pieces:  Vec<&str> = line.split_whitespace().collect();
    let mut tmp: Vec<String> = vec![];
    for e in exprs {
        let foo = eval(pieces.clone(), e.clone());
        tmp.push(foo);
    }
    let output: String = intersperse_tab(tmp).into_iter().collect();
    output
}

/// Arguments to this program are a stack DSL.
fn process_args(args: Vec<String>) -> Vec<StackExpr> {
    let mut results = vec![];
    for a in args {
        let   chars: Vec<char> = a.chars().collect();
        match chars[0] {
            // This arg is a number.
            '0'...'9' => {
                let num = Column(a.parse::<u32>().unwrap());
                results.push(StackExpr::simple(num))
            },
            // This arg could be anything, make it a word. We will look the
            // word up later to see if it is valid when we eval.
            _   => { }
        };
    }
    results
}

/// Output formatting.
fn intersperse_tab(xs: Vec<String>) -> Vec<String> {
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

///
/// Types
///

#[derive(Debug,PartialEq,Clone)]
struct StackExpr {
    ops: Vec<StackOp>
}

impl StackExpr {
    fn len(&self) -> usize {
        self.ops.len()
    }
    fn is_simple(&self) -> bool {
        self.len() == 1
    }
    fn simple(op: StackOp) -> StackExpr {
        StackExpr{ ops: vec![op]}
    }
}

#[derive(Debug,PartialEq,Clone)]
enum StackOp {
    Column(u32)
}

/// Evaluate a StackExpr.
fn eval(pieces: Vec<&str>, expr: StackExpr) -> String {
    if expr.is_simple() {
        match expr.ops[0] {
            Column(idx) => {
                let zero_based = idx - 1;
                let res = pieces.get(zero_based as usize).unwrap();
                return String::from(*res)
            }
        }
    }
    let mut stack = vec![];
    let StackExpr{ops} = expr;
    for o in ops {
        match o {
            Column(idx) => {
                let zero_based = idx - 1;
                let res        = pieces.get(zero_based as usize).unwrap();
                let num: u32   = res.parse().unwrap();
                stack.push(num);
            }
        }
    }
    stack.pop().unwrap().to_string()
}

///
/// Tests
///

// Helper b/c I don't know how to alias String::from
// TODO: how do i make this "invisbile" to dead code
// b/c its only used for tests
fn str(lit: &str) -> String {
    String::from(lit)
}

#[test]
fn test_process_args_1() {
    let args = vec![str("1"), str("2")];
    let t1   = StackExpr::simple(Column(1));
    let t2   = StackExpr::simple(Column(2));
    assert_eq!(vec![t1,t2], process_args(args));
}

#[test]
fn test_eval_simple() {
    let expr   = StackExpr { ops: vec![Column(1)] };
    let s      = "a".to_string();
    let pieces = s.split_whitespace().collect();
    assert_eq!(str("a"), eval(pieces, expr));
}

#[test]
fn test_simple_expr() {
    let expr  = StackExpr { ops: vec![Column(1)] };
    assert_eq!(true, expr.is_simple());
}

