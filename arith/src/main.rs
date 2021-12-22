#![feature(box_syntax, box_patterns)]

#[derive(Clone, Debug, PartialEq)]
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

fn eval1(t: &Term) -> Term {
    match t {
        If(box True, t2, _t3) => *t2.clone(),
        If(box False, _t2, t3) => *t3.clone(),
        If(box t1, t2, t3) => If(box eval1(&t1), t2.clone(), t3.clone()),
        Succ(box t1) => Succ(box eval1(&t1)),
        Pred(box Zero) => Zero,
        Pred(box Succ(nv1)) if is_numeric_val(nv1) => *nv1.clone(),
        Pred(box t1) => Pred(box eval1(&t1)),
        IsZero(box Zero) => True,
        IsZero(box Succ(nv1)) if is_numeric_val(nv1) => False,
        IsZero(box t1) => IsZero(box eval1(&t1)),
        _ => panic!("NoRuleApplies"),
    }
}

fn eval(t: &Term) -> Term {
    let mut u = t.clone();
    while !is_val(&u) {
        u = eval1(&u);
    }
    u
}

fn big_step_eval(t: &Term) -> Term {
    match t {
        t if is_val(&t) => t.clone(),
        If(t1, t2, _t3) if big_step_eval(t1) == True => big_step_eval(t2),
        If(t1, _t2, t3) if big_step_eval(t1) == False => big_step_eval(t3),
        Succ(nv1) if is_numeric_val(&big_step_eval(nv1)) => Succ(box big_step_eval(nv1)),
        Pred(t1) if big_step_eval(t1) == Zero => Zero,
        Pred(t1) => {
            if let Succ(nv1) = big_step_eval(t1) {
                *nv1.clone()
            } else {
                panic!("NoRuleApplies")
            }
        }
        IsZero(t1) if big_step_eval(t1) == Zero => True,
        IsZero(t1) => {
            if let Succ(_) = big_step_eval(t1) {
                False
            } else {
                panic!("NoRuleApplies")
            }
        }
        _ => panic!("NoRuleApplies"),
    }
}

use Term::*;

fn main() {
    let t = box If(box True, box True, box True);
    println!("{:?}", eval(&t));
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

    #[test]
    fn eval_test() {
        let t = box If(
            box IsZero(box Zero),
            box If(box False, box True, box Succ(box Zero)),
            box False,
        );
        assert_eq!(eval(&t), Succ(box Zero));
    }

    #[test]
    fn big_step_eval_test() {
        let t = box If(
            box IsZero(box Zero),
            box If(box False, box True, box Succ(box Zero)),
            box False,
        );
        assert_eq!(big_step_eval(&t), Succ(box Zero));
    }
}
