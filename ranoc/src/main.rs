use libranoc::parse;

fn main() {
    let src = r#"
        fn main {
            show "Hello, world!";
        }
    "#;

    let mut lexer = parse::Tokenizer(src);
    while let Some(token) = lexer.next() {
        dbg!(&token);
        dbg!(&lexer.span());
        dbg!(&lexer.extras);
    }
}
