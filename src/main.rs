use mini_lisp::interpreter;
use mini_lisp::parser;

fn main() {
    let path = std::env::args().nth(1).expect("No file path provided");
    let unparsed = std::fs::read_to_string(path).expect("Could not read file");
    let writer = std::io::stdout();
    match parser::parse(&unparsed) {
        Ok(program) => {
            if let Err(err) = interpreter::run(program, &mut writer.lock()) {
                eprintln!("{}", err);
            }
        }
        Err(err) => eprintln!("{}", err),
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use super::*;

    #[test]
    fn test_syntax_error() {
        let unparsed = "(+)";
        let result = parser::parse(&unparsed);
        assert!(result.is_err());

        let unparsed = "(+ (* 5 2) -)";
        let result = parser::parse(&unparsed);
        assert!(result.is_err());
    }

    #[test]
    fn test_print_num() {
        let unparsed = r"(print-num 1)
            (print-num 2)
            (print-num 3)
            (print-num 4)";
        let program = parser::parse(&unparsed).unwrap();
        let mut writer = Vec::new();
        interpreter::run(program, &mut writer).unwrap();
        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "1\n2\n3\n4\n");

        let unparsed = r"(print-num 0)
            (print-num -123)
            (print-num 456)";
        let program = parser::parse(&unparsed).unwrap();
        let mut writer = Vec::new();
        interpreter::run(program, &mut writer).unwrap();
        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "0\n-123\n456\n");
    }

    #[test]
    fn test_num_exp() {
        let unparsed = r"(+ 1 2 3)
            (* 4 5 6)
            (print-num (+ 1 (+ 2 3 4) (* 4 5 6) (/ 8 3) (mod 10 3)))
            (print-num (mod 10 4))
            (print-num (- (+ 1 2) 4))
            (print-num -256)";
        let program = parser::parse(&unparsed).unwrap();
        let mut writer = Vec::new();
        interpreter::run(program, &mut writer).unwrap();
        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "133\n2\n-1\n-256\n");

        let unparsed = r"(print-num (mod 10 (+ 1 2)))
            (print-num (* (/ 1 2) 4))
            (print-num (- (+ 1 2 3 (- 4 5) 6 (/ 7 8) (mod 9 10)) 11))";
        let program = parser::parse(&unparsed).unwrap();
        let mut writer = Vec::new();
        interpreter::run(program, &mut writer).unwrap();
        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "1\n0\n9\n");
    }

    #[test]
    fn test_bool() {
        let unparsed = r"(print-bool #t)
            (print-bool #f)
            (print-bool (and #t #f))
            (print-bool (and #t #t))
            (print-bool (or #t #f))
            (print-bool (or #f #f))
            (print-bool (not #t))
            (print-bool (not #f))";
        let program = parser::parse(&unparsed).unwrap();
        let mut writer = Vec::new();
        interpreter::run(program, &mut writer).unwrap();
        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "#t\n#f\n#f\n#t\n#t\n#f\n#f\n#t\n");

        let unparsed = r"(print-bool (or #t #t #f))
            (print-bool (or #f (and #f #t) (not #f)))
            (print-bool (and #t (not #f) (or #f #t) (and #t (not #t))))";
        let program = parser::parse(&unparsed).unwrap();
        let mut writer = Vec::new();
        interpreter::run(program, &mut writer).unwrap();
        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "#t\n#t\n#f\n");
    }

    #[test]
    fn test_if_statement() {
        let unparsed = r"(print-num (if #t 1 2))
            (print-num (if #f 1 2))";
        let program = parser::parse(&unparsed).unwrap();
        let mut writer = Vec::new();
        interpreter::run(program, &mut writer).unwrap();
        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "1\n2\n");

        let unparsed = r"(print-num (if (< 1 2) (+ 1 2 3) (* 1 2 3 4 5)))
            (print-num (if (= 9 (* 2 5))
              0
              (if #t 1 2)))";
        let program = parser::parse(&unparsed).unwrap();
        let mut writer = Vec::new();
        interpreter::run(program, &mut writer).unwrap();
        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "6\n1\n");
    }

    #[test]
    fn test_variable_definition() {
        let unparsed = r"(define x 1)
            (print-num x)
            (define y (+ 1 2 3))
            (print-num y)";
        let program = parser::parse(&unparsed).unwrap();
        let mut writer = Vec::new();
        interpreter::run(program, &mut writer).unwrap();
        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "1\n6\n");

        let unparsed = r"(define a (* 1 2 3 4))
            (define b (+ 10 -5 -2 -1))
            (print-num (+ a b))";
        let program = parser::parse(&unparsed).unwrap();
        let mut writer = Vec::new();
        interpreter::run(program, &mut writer).unwrap();
        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "26\n");
    }

    #[test]
    fn test_function() {
        let unparsed = r"(print-num
              ((fun (x) (+ x 1)) 3))
            (print-num
              ((fun (a b) (+ a b)) 4 5))";
        let program = parser::parse(&unparsed).unwrap();
        let mut writer = Vec::new();
        interpreter::run(program, &mut writer).unwrap();
        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "4\n9\n");

        let unparsed = r"(define x 0)
            (print-num
              ((fun (x y z) (+ x (* y z))) 10 20 30))
            (print-num x)";
        let program = parser::parse(&unparsed).unwrap();
        let mut writer = Vec::new();
        interpreter::run(program, &mut writer).unwrap();
        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "610\n0\n");
    }

    #[test]
    fn test_named_function() {
        let unparsed = r"(define foo
              (fun (a b c) (+ a b (* b c))))
            (print-num (foo 10 9 8))";
        let program = parser::parse(&unparsed).unwrap();
        let mut writer = Vec::new();
        interpreter::run(program, &mut writer).unwrap();
        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "91\n");

        let unparsed = r"(define bar (fun (x) (+ x 1)))
            (define bar-z (fun () 2))
            (print-num (bar (bar-z)))";
        let program = parser::parse(&unparsed).unwrap();
        let mut writer = Vec::new();
        interpreter::run(program, &mut writer).unwrap();
        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "3\n");
    }

    #[test]
    fn test_recursion() {
        let unparsed = r"(define fact
              (fun (n) (if (< n 3) n
                (* n (fact (- n 1))))))
            (print-num (fact 2))
            (print-num (fact 3))
            (print-num (fact 4))
            (print-num (fact 10))

            (define fib (fun (x)
              (if (< x 2) x (+
                (fib (- x 1))
                (fib (- x 2))))))

            (print-num (fib 1))
            (print-num (fib 3))
            (print-num (fib 5))
            (print-num (fib 10))
            (print-num (fib 20))";
        let program = parser::parse(&unparsed).unwrap();
        let mut writer = Vec::new();
        interpreter::run(program, &mut writer).unwrap();
        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "2\n6\n24\n3628800\n1\n2\n5\n55\n6765\n");

        let unparsed = r"(define min
              (fun (a b)
                  (if (< a b) a b)))

            (define max
              (fun (a b)
                (if (> a b) a b)))

            (define gcd
              (fun (a b)
                (if (= 0 (mod (max a b) (min a b)))
                  (min a b)
                  (gcd (min a b) (mod (max a b) (min a b))))))

            (print-num (gcd 100 88))

            (print-num (gcd 1234 5678))

            (print-num (gcd 81 54))";
        let program = parser::parse(&unparsed).unwrap();
        let mut writer = Vec::new();
        interpreter::run(program, &mut writer).unwrap();
        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "4\n2\n27\n");
    }

    #[test]
    fn test_type_checking() {
        let unparsed = r"(+ 1 2 3 (or #t #f))";
        let program = parser::parse(&unparsed).unwrap();
        let result = interpreter::run(program, &mut io::stdout());
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err, "Type Error: Expect 'number' but got 'boolean'");
        }

        let unparsed = r"(define f
              (fun (x)
                (if (> x 10) 10 (= x 5))))
            (print-num (* 2 (f 4)))";
        let program = parser::parse(&unparsed).unwrap();
        let result = interpreter::run(program, &mut io::stdout());
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err, "Type Error: Expect 'number' but got 'boolean'");
        }
    }

    #[test]
    fn test_nested_function() {
        let unparsed = r"(define dist-square
              (fun (x y)
                (define square (fun (x) (* x x)))
                (+ (square x) (square y))))
            (print-num (dist-square 3 4))";
        let program = parser::parse(&unparsed).unwrap();
        let mut writer = Vec::new();
        interpreter::run(program, &mut writer).unwrap();
        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "25\n");

        let unparsed = r"(define diff
              (fun (a b)
                  (define abs
                    (fun (a)
                      (if (< a 0) (- 0 a) a)))
                  (abs (- a b))))
            (print-num (diff 1 10))
            (print-num (diff 10 2))";
        let program = parser::parse(&unparsed).unwrap();
        let mut writer = Vec::new();
        interpreter::run(program, &mut writer).unwrap();
        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "9\n8\n");
    }

    #[test]
    fn test_first_class_function() {
        let unparsed = r"(define add-x
              (fun (x) (fun (y) (+ x y))))
            (define z (add-x 10))
            (print-num (z 1))";
        let program = parser::parse(&unparsed).unwrap();
        let mut writer = Vec::new();
        interpreter::run(program, &mut writer).unwrap();
        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "11\n");

        let unparsed = r"(define foo
              (fun (f x) (f x)))
            (print-num
              (foo (fun (x) (- x 1)) 10))";
        let program = parser::parse(&unparsed).unwrap();
        let mut writer = Vec::new();
        interpreter::run(program, &mut writer).unwrap();
        let output = String::from_utf8(writer).unwrap();
        assert_eq!(output, "9\n");
    }
}
