use libranoc::syntax;

fn main() {
    let src = r#"
        extern fn println(o: impl Show);
        fn main {
            -1 + 3 * 1[1+2] && 2;
        }
    "#;

    let tokens = syntax::tokenize(src);
    let ast = syntax::parse(&tokens);
    match ast {
        Ok(ast) => {
            dbg!(&ast);
        }
        Err(err) => {
            eprintln!("{:?}", err);
        }
    }
}
