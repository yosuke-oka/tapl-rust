//#![feature(box_syntax)]

#[derive(Clone, Debug)]
enum Term {
    True,
    False,
    If(Box<Term>, Box<Term>, Box<Term>),
    Zero,
    Succ(Box<Term>),
    Pred(Box<Term>),
    IsZero(Box<Term>),
}

fn isNumericVal(t: &Term) -> bool{
    match t {
        Term::Zero => true,
        Term::Succ(ref t1) => isNumericVal(t1),
        _ => false
    }
}


use Term::*;

fn main() {
    let t = Zero;
    println!("{:?}", isNumericVal(&t));

    let t = Box::new(Box::new(Succ(Box::new(Succ(Box::new(Zero))))));
    println!("{:?}", isNumericVal(&t));
}
