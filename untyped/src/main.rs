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
        App(t1, t2) => {
            print!("(");
            print_term_inner(&ctx, t1);
            print!(" ");
            print_term_inner(&ctx, t2);
            print!(")");
        }
    }
}

// Context内に変数名が既に含まれているか判定して、かぶっていたら新しい名前をつけて返す
fn pickup_freshname(ctx: &Context, x: &String) -> (Context, String) {
    let ret = ctx.iter().find(|(var, _)| var == x);
    match ret {
        Some(_) => {
            return pickup_freshname(&ctx, &format!("{}'", x));
        }
        None => {
            let mut new_ctx = ctx.clone();
            new_ctx.push((x.clone(), "NameBind".to_string()));
            return (new_ctx, x.clone());
        }
    }
}

fn main() {
    let ctx = vec![("x".to_string(), "NameBind".to_string())];
    let var = Var(0, 1);
    print_term(&ctx, &var);

    let abs = Abs("x".to_string(), box Var(1, 2));
    print_term(&ctx, &abs);

    let app = App(box abs, box var);
    print_term(&ctx, &app);
}
