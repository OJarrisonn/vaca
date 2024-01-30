#[cfg(test)]
mod tests {
    use chifre::library;

    #[test]
    fn foo_library() {
        let source = r#"
        <{net stl/net/tcp -> receive send}>
        <{fs stl/fs -> read-file}>
        @(do-foo a)
        #(addr (read-file "addr.txt")
          do-foo <(addr : -> { (send addr "Foo" ) (receive addr) }))
        #(a 1000)
        "#;

        let library = library::lex_library(source);
        dbg!(&library);

        assert!(library.is_ok())
    }
}