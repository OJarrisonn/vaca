#[cfg(test)]
mod tests {
    use chifre::form;
    #[test]
    fn simple_hello_world() {
        let source = r#"(println "Hello World")"#;
        let repl = form::lex_form(source);
        //let repl = dbg!(repl);
        assert!(repl.is_ok())
    }

    #[test]
    fn mismatching_paren() {
        let source = r#"#(a <(x -> (* 2 x))"#;
        let repl = form::lex_form(source);
        //let repl = dbg!(repl);
        assert!(repl.is_err())
    }

    #[test]
    fn literals() {
        let source = r#"{a'}"#;

        let repl = form::lex_form(source);
       // let repl = dbg!(repl);
        assert!(repl.is_ok())
    }
}