#![feature(box_syntax, box_patterns)]

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

fn is_numeric_val(t: &Term) -> bool {
    match t {
        Term::Zero => true,
        Term::Succ(ref t1) => is_numeric_val(t1),
        _ => false,
    }
}

use Term::*;

fn main() {
    let t = Zero;
    println!("{:?}", is_numeric_val(&t));

    let t = box Succ(box Succ(box Zero));
    println!("{:?}", is_numeric_val(&t));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_numeric_val_test() {
        let t = Zero;
        assert!(is_numeric_val(&t));

        let t = box Succ(box Succ(box Zero));
        assert!(is_numeric_val(&t));
    }
}
