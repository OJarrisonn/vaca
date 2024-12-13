pub fn lex(source: &str) {
    let value = edn_format::parse_str(source);
    println!("{:#?}", value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        lex(include_str!("samples/hello_world.vaca"));
    }
}