use eopl::{Value, parse_and_eval};

const KOTLIN_SAMPLE: &str = include_str!("main.let");

#[test]
fn evaluates_the_ported_kotlin_sample() {
    let value = parse_and_eval(KOTLIN_SAMPLE).expect("sample program should evaluate");
    assert_eq!(value, Value::Number(25));
}

#[test]
fn evaluates_recursive_factorial_with_letrec() {
    let source = r#"
        letrec fact(n) =
            if zero?(n)
            then 1
            else *(n,(fact -(n,1)))
        in (fact 5)
    "#;

    let value = parse_and_eval(source).expect("letrec program should evaluate");
    assert_eq!(value, Value::Number(120));
}
