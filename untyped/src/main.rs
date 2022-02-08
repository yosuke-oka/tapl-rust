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
    fn inner(ctx: &Context, t: &Term) {
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
                inner(&new_ctx, t1);
                print!(")");
            }
            App(t1, t2) => {
                print!("(");
                inner(&ctx, t1);
                print!(" ");
                inner(&ctx, t2);
                print!(")");
            }
        }
    }

    inner(ctx, t);
    println!();
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

fn term_shift(d: usize, t: &Term) -> Term {
    struct Env {
        d: usize,
    }
    fn walk(env: &Env, c: usize, t: &Term) -> Term {
        match t {
            Var(x, n) => {
                if x >= &c {
                    Var(x + env.d, n + env.d)
                } else {
                    Var(*x, n + env.d)
                }
            }
            Abs(x, t1) => Abs(x.clone(), box walk(env, c + 1, t1)),
            App(t1, t2) => App(box walk(env, c, t1), box walk(env, c, t2)),
        }
    }

    let env = Env { d: d };
    walk(&env, 0, t)
}

fn term_subst(j: usize, s: &Term, t: &Term) -> Term {
    struct Env {
        j: usize,
        s: Term,
    }
    fn walk(env: &Env, c: usize, t: &Term) -> Term {
        match t {
            Var(x, n) => {
                if *x == env.j + c {
                    walk(env, c, &env.s)
                } else {
                    Var(*x, *n)
                }
            }
            Abs(x, t1) => Abs(x.clone(), box walk(env, c + 1, t1)),
            App(t1, t2) => App(box walk(env, c, t1), box walk(env, c, t2)),
        }
    }

    let env = Env { j: j, s: s.clone() };

    walk(&env, 0, t)
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
