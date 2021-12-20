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

fn is_val(t: &Term) -> bool {
    match t {
        Term::True | Term::False => true,
        t1 if is_numeric_val(t1) => true,
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

        let t = box Pred(box Zero);
        assert!(!is_numeric_val(&t));
    }

    #[test]
    fn is_val_test() {
        let t = True;
        assert!(is_val(&t));

        let t = False;
        assert!(is_val(&t));

        let t = box Succ(box Succ(box Zero));
        assert!(is_val(&t));

        let t = box If(box True, box True, box True);
        assert!(!is_val(&t));
    }
}
