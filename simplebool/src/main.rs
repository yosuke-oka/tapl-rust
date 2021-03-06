#![feature(box_syntax, box_patterns)]
use std::fmt;

#[derive(Clone, Debug)]
enum Term {
    Var(usize, usize),
    Abs(String, Type, Box<Term>),
    App(Box<Term>, Box<Term>),
    True,
    False,
    If(Box<Term>, Box<Term>, Box<Term>),
}

#[derive(Clone, Debug, PartialEq)]
enum Type {
    Arrow(Box<Type>, Box<Type>),
    Bool,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Arrow(t1, t2) => write!(f, "{} -> {}", t1, t2),
            Bool => write!(f, "Bool"),
        }
    }
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

fn type_of(ctx: &Context, t: &Term) -> Type {
    match t {
        Var(i, _) => get_type_from_context(ctx, *i),
        Abs(x, ty_t1, t2) => {
            let new_ctx = add_binding(ctx, x, VarBind(ty_t1.clone()));
            let ty_t2 = type_of(&new_ctx, t2);
            Arrow(box ty_t1.clone(), box ty_t2)
        }
        App(t1, t2) => {
            let ty_t1 = type_of(ctx, t1);
            let ty_t2 = type_of(ctx, t2);
            match ty_t1 {
                Arrow(ty_t11, ty_t12) => {
                    if ty_t2 == *ty_t11 {
                        *ty_t12
                    } else {
                        panic!("parameter type mismatch")
                    }
                }
                _ => panic!("arrow type expected"),
            }
        }
        True => Bool,
        False => Bool,
        If(t1, t2, t3) => {
            if type_of(ctx, t1) != Bool {
                panic!("guard of conditional not a boolean");
            }
            let ty_t2 = type_of(ctx, t2);
            if ty_t2 != type_of(ctx, t3) {
                panic!("arms of conditional have different type")
            }
            ty_t2
        }
    }
}

fn print_term(ctx: &Context, t: &Term) {
    fn inner(ctx: &Context, t: &Term) {
        match t {
            Var(index, context_length) => {
                if *context_length == ctx.len() {
                    print!("{}", ctx[*index].0);
                } else {
                    panic!("bad index: (context, term) = ({:?}, {:?})", ctx, t);
                }
            }
            Abs(x, t, t1) => {
                let (new_ctx, new_x) = pickup_freshname(ctx, x);
                print!("(?? {}:{:?}. ", new_x, t);
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
            True => print!("true"),
            False => print!("false"),
            If(t1, t2, t3) => {
                print!("(if ");
                inner(&ctx, t1);
                print!(" then ");
                inner(&ctx, t2);
                print!(" else ");
                inner(&ctx, t3);
                print!(")");
            }
        }
    }

    inner(ctx, t);
    println!();
}

// Context??????????????????????????????????????????????????????????????????????????????????????????????????????????????????
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
        True => true,
        False => true,
        _ => false,
    }
}

fn eval1(ctx: &Context, t: &Term) -> Term {
    match t {
        App(box Abs(_x, _, t12), box v2) if is_val(ctx, v2) => term_subst_top(v2, t12),
        App(box v1, box t2) if is_val(ctx, v1) => App(box v1.clone(), box eval1(ctx, t2)),
        App(box t1, box t2) => App(box eval1(ctx, t1), box t2.clone()),
        If(box True, t2, _t3) => *t2.clone(),
        If(box False, _t2, t3) => *t3.clone(),
        If(box t1, t2, t3) => If(box eval1(ctx, &t1), t2.clone(), t3.clone()),
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

    let f = Abs(
        "b".to_string(),
        Bool,
        box If(box Var(0, 1), box True, box False),
    );
    print_term(&ctx, &f);
    println!("{}", type_of(&ctx, &f));

    let t1 = App(box f.clone(), box If(box False, box True, box False));
    print_term(&ctx, &t1);
    println!("{}", type_of(&ctx, &t1));

    let ret = eval(&ctx, &t1);
    print_term(&ctx, &ret);
    println!("{}", type_of(&ctx, &ret));

    let t2 = Abs(
        "x".to_string(),
        Bool,
        box App(
            box Abs(
                "b".to_string(),
                Bool,
                box If(box Var(1, 2), box True, box False),
            ),
            box If(box Var(0, 1), box False, box Var(0, 1)),
        ),
    );
    print_term(&ctx, &t2);
    println!("{}", type_of(&ctx, &t2));

    let t = App(box t2.clone(), box False);
    let ret = eval(&ctx, &t);
    print_term(&ctx, &ret);
    println!("{}", type_of(&ctx, &ret));
}
