#![cfg(test)]

use lol::{create_lol_module, create_lol_runtime, Interpreter};
use lovm2::prelude::*;

#[test]
fn arithmetic() {
    let mut int = create_lol_runtime(
        "main",
        "
        (def add (a b)
            (ret (+ a b)))
        (def sub (a b)
            (ret (- a b)))
        (def mul (a b)
            (ret (* a b)))
        (def div (a b)
            (ret (/ a b)))
        (def rem (a b)
            (ret (% a b)))
        ",
    );

    let add = int.call("add", &[1, 2]).unwrap();
    let sub = int.call("sub", &[1, 2]).unwrap();
    let mul = int.call("mul", &[1, 2]).unwrap();
    let div = int.call("div", &[1, 2]).unwrap();
    let rem = int.call("rem", &[1, 2]).unwrap();
    assert_eq!(Value::from(3), add);
    assert_eq!(Value::from(-1), sub);
    assert_eq!(Value::from(2), mul);
    assert_eq!(Value::from(0.5), div);
    assert_eq!(Value::from(1), rem);
}

#[test]
fn recursive_faculty() {
    let mut int = create_lol_runtime(
        "main",
        "
        (def fac (x) 
            (if (not (eq x 0))
                (ret (* x (fac (- x 1))))
                (ret 1)))
        ",
    );

    assert_eq!(Value::from(1), int.call("fac", &[1]).unwrap());
    assert_eq!(Value::from(2), int.call("fac", &[2]).unwrap());
    assert_eq!(Value::from(6), int.call("fac", &[3]).unwrap());
    assert_eq!(Value::from(5040), int.call("fac", &[7]).unwrap());
}

#[test]
fn looping() {
    let mut int = create_lol_runtime(
        "loops",
        "
        (def looping (n)
            (let r 1)
            (let i 0)
            (loop
                (if (eq i n)
                    (break))
                (if (eq (% i 2) 0)
                    (do
                        (let i (+ i 1))
                        (continue)))
                (let r (* r i))
                (let i (+ i 1)))
            (ret r))
        ",
    );

    assert_eq!(Value::from(1), int.call("looping", &[1]).unwrap());
    assert_eq!(Value::from(1), int.call("looping", &[2]).unwrap());
    assert_eq!(Value::from(3), int.call("looping", &[5]).unwrap());
}

#[test]
fn import_vice_versa() {
    let mut int = Interpreter::new();
    let a = create_lol_module(
        "a",
        "
    (import b)
    (def main (x)
        (ret (b-inb)))
        ",
    )
    .unwrap();
    let b = create_lol_module(
        "b",
        "
    (def inb ()
        (ret 1))
        ",
    )
    .unwrap();

    int.load(b).unwrap();
    int.load(a).unwrap();

    assert!(int.call("a-main", &[0]).is_ok());
}

#[test]
fn create_complex_types() {
    let mut int = Interpreter::new();
    let main = create_lol_module(
        "main",
        r#"
    (def create-list (n)
        (ret (list n 2 "abc")))

    (def create-dict (key val)
        (ret (dict (key val) ("a" 1))))
        "#,
    )
    .unwrap();

    int.load_global(main).unwrap();

    let ls = int.call("create-list", &[true]).unwrap();

    assert_eq!(Value::from(true), ls.get(&0.into()).unwrap());
    assert_eq!(Value::from(2), ls.get(&1.into()).unwrap());
    assert_eq!(Value::from("abc"), ls.get(&2.into()).unwrap());

    let dict = int.call("create-dict", &[1, 2]).unwrap();

    assert_eq!(Value::from(2), dict.get(&1.into()).unwrap());
    assert_eq!(Value::from(1), dict.get(&"a".into()).unwrap());
}

#[test]
fn foreach() {
    let mut int = Interpreter::new();
    let main = create_lol_module(
        "main",
        r#"
    (def sum (n)
        (let res 0)
        (foreach ((range 1 (+ n 1)) i)
            (let res (+ res i)))
        (ret res))
        "#,
    )
    .unwrap();

    int.load_global(main).unwrap();

    assert_eq!(Value::from(6), int.call("sum", &[3]).unwrap());
    assert_eq!(Value::from(10), int.call("sum", &[4]).unwrap());
    assert_eq!(Value::from(15), int.call("sum", &[5]).unwrap());
}
