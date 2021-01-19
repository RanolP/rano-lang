use libranoc::syntax;

fn main() {
    let src = r#"
        extern fn println(o: impl Show)
        fn main
    "#;

    let tokens = syntax::tokenize(src);
    let ast = syntax::parse(&tokens);
    if let Ok(ast) = ast {
        dbg!(&ast);
    }
}
