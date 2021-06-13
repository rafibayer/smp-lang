use smp;

#[test]
fn test_simple() {
    let program = String::from("
    def main() {
        return 1;
    }
    ");
    let mut s = smp::scanner::Scanner::new(program).unwrap();
    let program = smp::ast::generate_ast(&mut s).unwrap();
    let interpreter = smp::interpreter::Interpreter::new(program);
    assert_eq!(interpreter.execute().unwrap(), Some(1f64));
}

#[test]
fn test_recursive() {
    let program = String::from("
    def fact(n) {
        if (n == 1) {
            return 1;
        }

        return n * fact(n - 1);
    }
    def main() {
        return fact(10);
    }
    ");
    let mut s = smp::scanner::Scanner::new(program).unwrap();
    let program = smp::ast::generate_ast(&mut s).unwrap();
    let interpreter = smp::interpreter::Interpreter::new(program);
    assert_eq!(interpreter.execute().unwrap(), Some(3628800f64));
}

#[test]
fn test_multiarg() {
    let program = String::from("
    def func(a, b, c, d, e) {
        return a*b*c*d*e;

    }

    def main() {
        return func(1, 2, 3, 4, 5);
    }
    ");
    let mut s = smp::scanner::Scanner::new(program).unwrap();
    let program = smp::ast::generate_ast(&mut s).unwrap();
    let interpreter = smp::interpreter::Interpreter::new(program);
    assert_eq!(interpreter.execute().unwrap(), Some(120f64));
}

#[test]
fn test_nested_parens() {
    let program = String::from("

    def main() {
        return ((1));
    }
    ");
    let mut s = smp::scanner::Scanner::new(program).unwrap();
    let program = smp::ast::generate_ast(&mut s).unwrap();
    let interpreter = smp::interpreter::Interpreter::new(program);
    assert_eq!(interpreter.execute().unwrap(), Some(1f64));
}