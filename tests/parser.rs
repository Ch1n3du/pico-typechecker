use chumsky::Parser;
use pico_typechecker::{
    ast::{Expr, Function},
    lexer::lexer,
    parser,
    tipo::Tipo,
    token::Token,
};

fn try_parsing(src: &str) -> Expr {
    let toks: Vec<Token> = lexer()
        .parse(src)
        .unwrap()
        .into_iter()
        .map(|p| p.0)
        .collect();

    parser::expr_parser().parse(toks).unwrap()
}

#[test]
fn basic() {
    let exprs = try_parsing("let x = if true { 12 } else { 34 }; true");
    println!("{exprs:?}")
}

#[test]
fn can_parse_funk_expr() {
    let src = "funk fib(n: int) -> int { n } 43";
    // let src = "let x = let y = 3;";
    let ast = try_parsing(src);
    let fn_ = Function {
        params: vec![("n".to_string(), Tipo::int_type())],
        ret: Tipo::int_type(),
        body: Box::new(Expr::Block(Box::new(Expr::Identifier("n".to_string())))),
    };

    let expected = Expr::Funk {
        name: "fib".to_string(),
        fn_,
        // then: Box::new(Expr::Value(Value::Num(43))),
    };
    assert_eq!(ast, expected);
    // panic!("{ast:?}")
}

#[test]
fn can_parse_anon_fns() {
    let src = "let add = fn (a: int, b: int) -> int { a + b }; 4";
    let ast = try_parsing(src);
    println!("{ast:?}")
}

#[test]
fn can_parse_bigboy() {
    let src = r#"
    funk main() -> int {
        let add = fn (a: int, b: int) -> int { a + b };
        32
    }
    "#;
    let ast = try_parsing(src);
    println!("{ast:?}");
    panic!()
}

#[test]
fn can_parse_goal() {
    let src = r#"
    funk fib(n: int) -> int {
        if n < 2 {
            1
        } else {
            fib(n-1) + fib(n-2)
        }
    }
    funk main() {
        let x = 45 * 23;
        let y: int = 1322;
        let add = fn(a: int, b: int) -> int { a + b };

        add(x, y)
    }
    "#;

    let ast = try_parsing(src);
    panic!("{ast:?}")
}