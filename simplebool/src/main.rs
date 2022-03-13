#![feature(box_syntax, box_patterns)]
//use std::fmt;

#[derive(Clone, Debug)]
enum Term {
    Var(usize, usize),
    Abs(String, Type, Box<Term>),
    App(Box<Term>, Box<Term>),
    True,
    False,
    If(Box<Term>, Box<Term>, Box<Term>),
}

#[derive(Clone, Debug)]
enum Type {
    Arrow(Box<Type>, Box<Type>),
    Bool,
}

#[derive(Clone, Debug)]
enum Binding {
    NameBind,
    VarBind(Type),
}
type Context = Vec<(String, Binding)>;

use Binding::*;
use Term::*;
use Type::*;

fn add_binding(ctx: &Context, x: &String, bind: Binding) -> Context {
    let mut new_ctx = ctx.clone();
    new_ctx.insert(0, (x.clone(), bind));
    new_ctx
}

fn get_type_from_context(ctx: &Context, i: usize) -> Type {
    match &ctx[i] {
        (_, VarBind(ty)) => ty.clone(),
        _ => panic!("get_type_from_context: Wrong kind of binding for variable"),
    }
}

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
            Abs(x, t, t1) => {
                let (new_ctx, new_x) = pickup_freshname(ctx, x);
                print!("(λ {}:{:?}. ", new_x, t);
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
            True => println!("True"),
            False => println!("False"),
            If(t1, t2, t3) => {
                print!("If ");
                inner(&ctx, t1);
                print!(" then ");
                inner(&ctx, t2);
                print!(" else ");
                inner(&ctx, t3);
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
            new_ctx.push((x.clone(), NameBind));
            return (new_ctx, x.clone());
        }
    }
}

fn term_shift(d: isize, t: &Term) -> Term {
    struct Env {
        d: isize,
    }
    fn walk(env: &Env, c: usize, t: &Term) -> Term {
        match t {
            Var(x, n) => {
                if x >= &c {
                    Var(
                        (*x as isize + env.d) as usize,
                        (*n as isize + env.d) as usize,
                    )
                } else {
                    Var(*x, (*n as isize + env.d) as usize)
                }
            }
            Abs(x, t, t1) => Abs(x.clone(), t.clone(), box walk(env, c + 1, t1)),
            App(t1, t2) => App(box walk(env, c, t1), box walk(env, c, t2)),
            True => t.clone(),
            False => t.clone(),
            If(t1, t2, t3) => If(
                box walk(env, c, t1),
                box walk(env, c, t2),
                box walk(env, c, t3),
            ),
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
            Abs(x, t, t1) => Abs(x.clone(), t.clone(), box walk(env, c + 1, t1)),
            App(t1, t2) => App(box walk(env, c, t1), box walk(env, c, t2)),
            True => t.clone(),
            False => t.clone(),
            If(t1, t2, t3) => If(
                box walk(env, c, t1),
                box walk(env, c, t2),
                box walk(env, c, t3),
            ),
        }
    }

    let env = Env { j: j, s: s.clone() };

    walk(&env, 0, t)
}

fn term_subst_top(s: &Term, t: &Term) -> Term {
    term_shift(-1, &term_subst(0, &term_shift(1, s), t))
}

fn is_val(_ctx: &Context, t: &Term) -> bool {
    match t {
        Abs(_, _, _) => true,
        _ => false,
    }
}

fn eval1(ctx: &Context, t: &Term) -> Term {
    match t {
        App(box Abs(_x, _, t12), box v2) if is_val(ctx, v2) => term_subst_top(v2, t12),
        App(box v1, box t2) if is_val(ctx, v1) => App(box v1.clone(), box eval1(ctx, t2)),
        App(box t1, box t2) => App(box eval1(ctx, t1), box t2.clone()),
        _ => panic!("NoRuleApplies"),
    }
}

fn eval(ctx: &Context, t: &Term) -> Term {
    let mut u = t.clone();
    while !is_val(ctx, &u) {
        u = eval1(ctx, &u);
    }
    u
}

fn main() {
    let ctx = vec![];
    let tru = Abs(
        "t".to_string(),
        Bool,
        box Abs("f".to_string(), Bool, box Var(0, 2)),
    );
    let fls = Abs(
        "t".to_string(),
        Bool,
        box Abs("f".to_string(), Bool, box Var(1, 2)),
    );
    print_term(&ctx, &tru);
    print_term(&ctx, &fls);
    let and = App(
        box Abs(
            "b".to_string(),
            Bool,
            box Abs("c".to_string(), Bool, box App(box Var(0, 2), box Var(1, 2))),
        ),
        box fls.clone(),
    );
    print_term(&ctx, &and);
    let t = App(box App(box and.clone(), box tru.clone()), box tru.clone());
    print_term(&ctx, &eval(&ctx, &t));
    let t = App(box App(box and.clone(), box tru.clone()), box fls.clone());
    print_term(&ctx, &eval(&ctx, &t));
}
