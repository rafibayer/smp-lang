use smp;
use smp::interpreter::environment::Value;

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
    assert_eq!(interpreter.execute().unwrap(), Some(Value::from(1f64)));
}

#[test]
fn test_recursive() {
    let program = String::from("
    def fact(n) {
        if (n == 1) {
            return 1;
        }

        # n! = n * (n-1)!
        return n * fact(n - 1);
    }
    def main() {
        return fact(10);
    }
    ");
    let mut s = smp::scanner::Scanner::new(program).unwrap();
    let program = smp::ast::generate_ast(&mut s).unwrap();
    let interpreter = smp::interpreter::Interpreter::new(program);
    assert_eq!(interpreter.execute().unwrap(), Some(Value::from(3628800f64)));
}

#[test]
fn test_multiarg() {
    let program = String::from("
    # returns the product of the parameters 
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
    assert_eq!(interpreter.execute().unwrap(), Some(Value::from(120f64)));
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
    assert_eq!(interpreter.execute().unwrap(), Some(Value::from(1f64)));
}

#[test]
fn test_arr() {
    let program = String::from("

    def main() {
        arr := [5];
        i := 0;
        while (i < 5) {
            arr[i] := i;
            i := i + 1;
        }
        arr[4]; # outputs the 5th element of arr
        return arr; # returns the entire array
    }
    ");
    let mut s = smp::scanner::Scanner::new(program).unwrap();
    let program = smp::ast::generate_ast(&mut s).unwrap();
    let interpreter = smp::interpreter::Interpreter::new(program);
    assert_eq!(interpreter.execute().unwrap(), Some(Value::from(vec![0f64, 1f64, 2f64, 3f64, 4f64])));
}

#[test]
fn test_reassign() {
    let program = String::from("

    def main() {
        a := [10]; # assign a to an array
        a := 5; # re-assign a to a num
        return a; # we should get back the num
    }
    ");
    let mut s = smp::scanner::Scanner::new(program).unwrap();
    let program = smp::ast::generate_ast(&mut s).unwrap();
    let interpreter = smp::interpreter::Interpreter::new(program);
    assert_eq!(interpreter.execute().unwrap(), Some(Value::from(5f64)));
}

#[test]
fn test_multicall() {
    let program = String::from("

    # returns a
    def reta(a) {
        return a;
    } 

    def main() {
        first := reta(1);
        second := reta(2);
        return first + second;
    }
    ");
    let mut s = smp::scanner::Scanner::new(program).unwrap();
    let program = smp::ast::generate_ast(&mut s).unwrap();
    let interpreter = smp::interpreter::Interpreter::new(program);
    assert_eq!(interpreter.execute().unwrap(), Some(Value::from(3f64)));
}

#[test]
fn test_comments() {
    let program = String::from("

    # comment
    def main() { # the main function is called main() {}
        # returns 1;
        return 1; # we will now return !!
        # 1 was returned
    }
    # the program is over
    # comments without newline");
    let mut s = smp::scanner::Scanner::new(program).unwrap();
    let program = smp::ast::generate_ast(&mut s).unwrap();
    let interpreter = smp::interpreter::Interpreter::new(program);
    assert_eq!(interpreter.execute().unwrap(), Some(Value::from(1f64)));
}