#[cfg(test)]
mod tests {
    use chifre::program::lex_program;
    use pasto::{build::validate_program, program::parse_program, BuildResult};

    fn lex_parse_validate(source: &str) -> BuildResult<()> {
        let tokens = lex_program(source);
        assert!(tokens.is_ok());
        let program = parse_program(tokens.unwrap());
        assert!(program.is_ok());
        let program = program.unwrap();
        
        validate_program(&program)
    }

    #[test]
    fn valid_program() {
        let source = r#"
        #(a 10)
        #(b 5)
        (println (+ a b))
        "#;
        
        assert!(lex_parse_validate(source).is_ok());
    }

    #[test]
    fn reassign_immutable() {
        let source = r#"
        #(a "Text")
        (println a)
        #(a "Another text") ; Invalid code
        "#;
        let res = lex_parse_validate(source);
        assert!(res.is_err());
        eprintln!("{}", res.unwrap_err());
    }

    #[test]
    fn access_mutable_from_inner_scope() {
        let source = r#"
        #'(a' "Text")
        (println a')
        {
            (println a') ; Invalid code
        }
        "#;
        let res = lex_parse_validate(source);
        assert!(res.is_err());
        eprintln!("{}", res.unwrap_err());
    }
}
