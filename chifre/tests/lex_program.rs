#[cfg(test)]
mod tests {
    use chifre::program;

    #[test]
    fn echo_program() {
        let source = r#"
        <{ fs stl/fs -> read-file write-file }>
        
        #(a (read-file))
        (write-file a)
        "#;

        let program = program::lex_program(source);
        dbg!(&program);

        assert!(program.is_ok())
    }
}