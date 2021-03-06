#![feature(box_syntax, box_patterns)]
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Term
{
    TString(String),
    TBool(bool),
    TConcat(Box<Term>, Box<Term>),
    TCon(Box<Term>, Box<Term>, Box<Term>),
    TWhen(Box<Term>, Box<Term>),
    TUnless(Box<Term>, Box<Term>),
    TPrintLn(Box<Term>),
    TUnit,
    TSeq(Box<Term>, Box<Term>),
    TShow(Box<Term>),
    TAnd(Box<Term>, Box<Term>),
    TOr(Box<Term>, Box<Term>),
    TNot(Box<Term>),
    TWhile(Box<Term>, Box<Term>),
    TS,
    TK,
    TI,
    TApp(Box<Term>, Box<Term>),
    TNil,
    TCons(Box<Term>, Box<Term>),
    TIsNil(Box<Term>),
    TFirst(Box<Term>),
    TSecond(Box<Term>),
    TZero,
    TInc(Box<Term>),
    TIsZero(Box<Term>),
    TPred(Box<Term>),
    TFromi8(i8)
}
use Term::*;
pub trait VM {
    fn println(&mut self, &String);
}

pub fn is_value(tr : & Term) -> bool
{
    match tr
    {
        &TS | &TK | &TI | &TString(_) | &TBool(_) | &TUnit => true,
        &TApp(box TK, box ref x) => is_value(x),
        &TApp(box TS, box ref x) => is_value(x),
        &TApp(box TApp(box TS, box ref l), box ref r) =>
            is_value(l) && is_value(r),
        &TNil => true,
        &TCons(box ref l, box ref r) => is_value(l) && is_value(r),
        &TZero => true,
        &TInc(box ref x) => is_value(x),
        _ => false
    }
}

pub fn eval<ENV : VM>(env : &mut ENV, tr : & Term) -> Term
{
    match tr
    {
        _ if is_value(tr) => tr.clone(),
        &TApp(box ref l, box ref r) =>
            match (eval(env, &l), eval(env, &r)) {
                (TApp(box TS, box TApp(x, y)), z) =>
                    eval(env,
                         &TApp(box TApp(x, box z.clone()), box TApp(y, box z))),
                (TApp(box TK, box x), _) => x,
                (TI, x) => x,
                (x, y) => {
                    debug_assert!(is_value(&x) && is_value(&y));
                    TApp(box x, box y) },
            },
        &TCon(box ref i, box ref t, box ref e) =>
            match eval(env, &i) {
                TBool(true) => eval(env, t),
                TBool(false) => eval(env, e),
                _ => unreachable!()
            },
        &TPrintLn(box ref s) =>
            match eval(env, &s) {
                TString(x) => { env.println(&x); TUnit },
                _ => unreachable!()
            },
        &TConcat(box ref l, box ref r) =>
            match (eval(env, l), eval(env, r)) {
                (TString(ls), TString(rs)) => TString(ls + &rs),
                _ => unreachable!()
            },
        &TSeq(box ref l, box ref r) => { eval(env, l); eval(env, r) },
        &TShow(box ref x) =>
            match eval(env, x) {
                TString(x) => TString(x),
                TBool(true) => TString(String::from("true")),
                TBool(false) => TString(String::from("false")),
                TUnit => TString(String::from("unit")),
                TZero => TString(String::from("O")),
                TInc(box x) =>
                    match eval(env, &TShow(box x)) {
                        TString(str) => TString(String::from("(S ") + &str + ")"),
                        _ => unreachable!()
                    },
                _ => unreachable!()
            },
        &TWhen(box ref c, box ref act) =>
            eval(env, &TCon(box c.clone(), box act.clone(), box TUnit)),
        &TUnless(box ref c, box ref act) =>
            eval(env, &TCon(box c.clone(), box TUnit, box act.clone())),
        &TAnd(box ref l, box ref r) =>
            eval(env, &TCon(box l.clone(), box r.clone(), box TBool(false))),
        &TOr(box ref l, box ref r) =>
            eval(env, &TCon(box l.clone(), box TBool(true), box r.clone())),
        &TNot(box ref c) =>
            eval(env, &TCon(box c.clone(), box TBool(false), box TBool(true))),
        &TWhile(box ref c, box ref act) =>
            eval(env,
                 &TWhen(box c.clone(),
                        box TSeq(box act.clone(),
                                 box TWhile(box c.clone(), box act.clone())))),
        &TIsNil(box ref l) =>
            match eval(env, &l) {
                TNil => TBool(true),
                _ => TBool(false)
            },
        &TFirst(box ref x) =>
            match eval(env, &x) {
                TCons(box ref l, _) => l.clone(),
                _ => unreachable!()
            },
        &TSecond(box ref x) =>
            match eval(env, &x) {
                TCons(_, box ref r) => r.clone(),
                _ => unreachable!()
            },
        &TIsZero(box ref x) =>
            match eval(env, &x) {
                TZero => TBool(true),
                _ => TBool(false)
            },
        &TPred(box ref x) =>
            match eval(env, &x) {
                TInc(box y) => y.clone(),
                _ => unreachable!()
            },
        &TFromi8(x) =>
            match x {
                0 => TZero,
                _ => TInc(box eval(env, &TFromi8(x-1)))
            },
        _ => unreachable!()
    }
}

use std::collections::LinkedList;
pub struct TestVM {
    output : LinkedList<String>
}

impl TestVM {
    pub fn new() -> TestVM {
        TestVM { output : LinkedList::new() }
    }
}

impl VM for TestVM {
    fn println(&mut self, x : &String) {
        self.output.push_back(x.clone())
    }
}

pub struct DefaultVM;

impl VM for DefaultVM {
    fn println(&mut self, x : &String) {
        println!("{}", x)
    }
}

#[test]
pub fn test_conditional(){
    assert_eq!(
        eval(&mut TestVM::new(),
             &TCon(box TBool(true),
                   box TString(String::from("Hello")),
                   box TString(String::from("World")))),
        TString(String::from("Hello")));
}

#[test]
pub fn test_concat(){
    assert_eq!(
        eval(&mut TestVM::new(),
             &TConcat(box TString(String::from("Hello ")),
                      box TString(String::from("World")))),
        TString(String::from("Hello World")));
}

#[test]
pub fn test_isnil_nil(){
    assert_eq!(
        eval(&mut TestVM::new(),
             &TIsNil(box TNil)),
        TBool(true));
}

#[test]
pub fn test_not_isnil_nonnil(){
    assert_eq!(
        eval(&mut TestVM::new(),
             &TNot(box TIsNil(box TCons(box TNil, box TNil)))),
        TBool(true));
}

#[test]
pub fn test_or_is_zero_pred_inc(){
    assert_eq!(
        eval(&mut TestVM::new(),
             &TOr(box TBool(false), box TIsZero(box TPred(box TInc(box TZero))))),
        TBool(true));
}

#[test]
pub fn test_unless_and_println_show_fromi(){
    let mut x = TestVM::new();
    eval(&mut x, &TUnless(box TAnd(box TBool(true), box TBool(false)),
                          box TPrintLn(box TShow(box TFromi8(9)))));
    let mut y = LinkedList::<String>::new();
    y.push_back(String::from("(S (S (S (S (S (S (S (S (S O)))))))))"));
    assert_eq!(x.output, y);
}

#[test]
pub fn test_app_SKI(){
    assert_eq!(eval(&mut TestVM::new(),
                    &TApp(box TApp(box TK,
                                   box TApp(box TI, box TApp(box TS, box TK))),
                          box TS)),
               TApp(box TS, box TK));
}
