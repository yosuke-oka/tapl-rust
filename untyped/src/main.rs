#![feature(box_syntax, box_patterns)]
//use std::fmt;

#[derive(Clone, Debug, PartialEq)]
enum Term {
    Var(usize, usize),
    Abs(String, Box<Term>),
    App(Box<Term>, Box<Term>),
}

type Binding = String;
type Context = Vec<(String, Binding)>;

use Term::*;

//impl fmt::Display for Term {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        match self {
//            Var()
//        }
//    }
//}
//
fn print_term(ctx: &Context, t: &Term) {
    print_term_inner(&ctx, &t);
    println!()
}

fn print_term_inner(ctx: &Context, t: &Term) {
    match t {
        Var(index, context_length) => {
            if *context_length == ctx.len() {
                print!("{}", ctx[*index].0);
            } else {
                panic!("bad index")
            }
        }
        Abs(x, t1) => {
            let (new_ctx, new_x) = pickup_freshname(ctx, x);
            print!("(λ {}. ", new_x);
            print_term_inner(&new_ctx, t1);
            print!(")");
        }
        _ => unreachable!(),
    }
}

// Context内に変数名が既に含まれているか判定して、かぶっていたら新しい名前をつけて返す
fn pickup_freshname(ctx: &Context, x: &String) -> (Context, String) {
    let ret = ctx.iter().find(|(var, _)| var == x);
    match ret {
        Some(s) => return (ctx.clone(), s.0.clone()),
        None => {
            let new_name = format!("{}'", x);
            let mut new_ctx = ctx.clone();
            new_ctx.push((new_name.clone(), "NameBind".to_string()));
            return (new_ctx, new_name);
        }
    }
}

fn main() {
    let ctx = vec![("x".to_string(), "NameBind".to_string())];
    let t = Var(0, 1);
    print_term(&ctx, &t);

    let t = Abs("x".to_string(), box Var(0, 1));
    print_term(&ctx, &t);
}
