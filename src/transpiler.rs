use ess::Sexp;
use lovm2::hir::block::Block;
use lovm2::hir::prelude::*;
use lovm2::module::Module;
use std::collections::HashMap;

macro_rules! take_as {
    ($expr:expr, $ty:path) => {
        match $expr {
            $ty(inner, _loc) => Ok(inner),
            _ => Err(format!("expected {}, got {:?}", stringify!($ty), $expr)),
        }
    };
}

pub struct Transpiler {
    operators: HashMap<&'static str, Operator2>,
}

impl Transpiler {
    pub fn new() -> Self {
        let mut trans = Self {
            operators: HashMap::new(),
        };
        trans.operators.insert("+", Operator2::Add);
        trans.operators.insert("-", Operator2::Sub);
        trans.operators.insert("*", Operator2::Mul);
        trans.operators.insert("/", Operator2::Div);
        trans.operators.insert("%", Operator2::Rem);
        trans.operators.insert("eq", Operator2::Equal);
        trans.operators.insert("ne", Operator2::NotEqual);
        trans.operators.insert("ge", Operator2::GreaterEqual);
        trans.operators.insert("gt", Operator2::GreaterThan);
        trans.operators.insert("le", Operator2::LessEqual);
        trans.operators.insert("lt", Operator2::LessThan);
        trans.operators.insert("and", Operator2::And);
        trans.operators.insert("or", Operator2::Or);
        trans
    }

    pub fn translate(&mut self, sexprs: &[Sexp]) -> Result<Module, String> {
        let mut builder = ModuleBuilder::new();

        for sexpr in sexprs.iter() {
            match sexpr {
                Sexp::List(list, _) => {
                    if let Some(Sexp::Sym(_, _)) = list.get(0) {
                        self.translate_define(&mut builder, &list)?;
                    // translate_macro(&list)?;
                    } else {
                        unimplemented!()
                    }
                }
                _ => panic!("not expected at top-level: {:?}", sexpr),
            }
        }

        builder.build()
    }

    fn translate_define(&self, module: &mut ModuleBuilder, list: &[Sexp]) -> Result<(), String> {
        let name = take_as!(&list[1], Sexp::Sym)?;
        let arguments = take_as!(&list[2], Sexp::List)?
            .iter()
            .map(|item| take_as!(item, Sexp::Sym).unwrap())
            .map(|item| Variable::from(item.to_string()))
            .collect();

        // TODO: avoid index errors here
        let body = &list[3..];
        let mut hir = HIR::with_args(arguments);

        for stmt in body.iter() {
            self.translate_macro(&mut hir.code, &stmt)?;
        }

        module.add(name.to_string()).hir(hir);

        Ok(())
    }

    fn translate_macro(&self, block: &mut Block, ast: &Sexp) -> Result<(), String> {
        let list = take_as!(&ast, Sexp::List)?;
        let name = take_as!(&list[0], Sexp::Sym)?;
        // TODO: avoid index errors here
        let rest = &list[1..];

        match name.as_ref() {
            "break" => block.push(Break::new()),
            "continue" => block.push(Continue::new()),
            "if" => {
                let condition = self.translate_expr(&rest[0])?;
                let branch = block.branch();
                self.translate_macro(branch.add_condition(condition), &rest[1])?;
                if rest.len() == 3 {
                    self.translate_macro(branch.default_condition(), &rest[2])?;
                }
            }
            "let" => {
                assert_eq!(2, rest.len());
                let name = take_as!(&rest[0], Sexp::Sym)?;
                let name = Variable::from(name.to_string());
                let val = self.translate_expr(&rest[1])?;
                block.push(Assign::local(name, val));
            }
            "loop" => {
                let repeat = block.repeat(None);
                for item in rest.iter() {
                    self.translate_macro(&mut repeat.block, item)?;
                }
            }
            "ret" => {
                assert!(rest.len() <= 1);
                let inx = if rest.is_empty() {
                    Return::nil()
                } else {
                    let val = self.translate_expr(&rest[0])?;
                    Return::value(val)
                };
                block.push(inx);
            }
            _ => {
                // TODO: use Call::with_args
                let mut call = Call::new(name.as_ref());

                for param in rest.iter() {
                    let expr = self.translate_expr(param)?;
                    call = call.arg(expr);
                }

                block.push(call);
            }
        }

        Ok(())
    }

    fn translate_expr(&self, sexp: &Sexp) -> Result<Expr, String> {
        match sexp {
            Sexp::Sym(name, _) => Ok(Expr::from(Variable::from(name.to_string()))),
            Sexp::Str(s, _) => Ok(Expr::from(s.as_ref())),
            Sexp::Char(c, _) => Ok(Expr::from(format!("{}", c).as_ref())),
            Sexp::Int(n, _) => Ok(Expr::from(*n)),
            Sexp::Float(n, _) => Ok(Expr::from(*n)),
            Sexp::List(list, _) => self.translate_expr_macro(list),
        }
    }

    fn translate_expr_macro(&self, list: &[Sexp]) -> Result<Expr, String> {
        let name = take_as!(&list[0], Sexp::Sym)?;
        // TODO: avoid index errors here
        let mut rest = vec![];
        for item in list.iter().skip(1) {
            rest.push(self.translate_expr(item)?);
        }

        if name.as_ref() == "not" {
            assert_eq!(1, rest.len());
            return Ok(Expr::not(rest[0].clone()));
        }

        if let Some(op) = self.operators.get(name.as_ref()) {
            Ok(Expr::from_opn(op.clone(), rest))
        } else {
            let call = Call::with_args(name.as_ref(), rest);
            Ok(Expr::from(call))
        }
    }
}
