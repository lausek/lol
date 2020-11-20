#![cfg(test)]

use lol::interpreter::Interpreter;
use lol::transpiler::Transpiler;
use lovm2::prelude::*;

macro_rules! add_module {
    ($name:expr, $src:expr) => {{
        let mut int = Interpreter::new();
        let mut trans = Transpiler::new();
        let meta: ModuleMeta = $name.to_string().into();
        let module = trans.build(meta, $src).unwrap();

        println!("{}", module);

        int.load(module).unwrap();
        int
    }};
}

#[test]
fn arithmetic() {
    let mut int = add_module!(
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
        "
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
    let mut int = add_module!(
        "main",
        "
        (def fac (x) 
            (if (not (eq x 0))
                (ret (* x (fac (- x 1))))
                (ret 1)))
        "
    );

    assert_eq!(Value::from(1), int.call("fac", &[1]).unwrap());
    assert_eq!(Value::from(2), int.call("fac", &[2]).unwrap());
    assert_eq!(Value::from(6), int.call("fac", &[3]).unwrap());
    assert_eq!(Value::from(5040), int.call("fac", &[7]).unwrap());
}

#[test]
fn looping() {
    let mut int = add_module!(
        "loops",
        "
        (def looping (n)
            (let r 1)
            (let i 0)
            (loop
                (print r \"\n\")
                (if (eq i n)
                    (break))
                (if (eq (% i 2) 0)
                    (let i (+ i 1))
                    (continue))
                (let r (* r i))
                (let i (+ i 1)))
            (ret r))
        "
    );

    assert_eq!(Value::from(1), int.call("looping", &[1]).unwrap());
    assert_eq!(Value::from(1), int.call("looping", &[2]).unwrap());
    assert_eq!(Value::from(1), int.call("looping", &[5]).unwrap());
}
