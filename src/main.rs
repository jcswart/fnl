use std::env as e;
use std::io;
use std::io::BufRead;
use StackOp::*;

fn main() {
    if e::args().len() == 1 {
        let basename = e::args().next().unwrap();
        let usage    = format!("Usage: {} [ COLUMNS... ]\n\nFunnel columns of data from stdin.\n\tCOLUMNS\t%1, %2, etc. Like awk: {{ print $1, $2; }}\n\n\tEx: {0} %1 %2", basename);
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
    Word(String),
    Column(u32),
    Number(u32),
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
            _ => {}
        }
    }
    let StackExpr{ops} = expr;
    for o in ops {
        // eval stack exprs here
    }
    1.to_string()
}

///
/// Tests
///

// Helper b/c I don't know how to alias String::from
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

    let expr2 = StackExpr { ops: vec![Column(1), Number(100)] };
    assert_eq!(false, expr2.is_simple());
}
