use libranoc::syntax;

fn main() {
    let src = r#"
        fn main {
            show "Hello, world!";
        }
    "#;

    let mut lexer = syntax::Tokenizer(src);
    while let Some(token) = lexer.next() {
        dbg!(&token);
        dbg!(&lexer.span());
        dbg!(&lexer.extras);
    }
}
