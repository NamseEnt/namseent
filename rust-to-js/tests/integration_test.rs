use rquickjs::{Context, Runtime};

#[test]
fn test_hello_world() {
    let mir_content = include_str!("test_hello.mir");

    let js_code = rust_to_js::transpile(mir_content);

    println!("Generated JavaScript:\n{}", js_code);

    let runtime = Runtime::new().unwrap();
    let context = Context::full(&runtime).unwrap();

    context.with(|ctx| {
        let glue = r#"
            let output = "";
            function _print(arg) {
                output += arg;
            }
        "#;

        ctx.eval::<(), _>(glue).unwrap();

        ctx.eval::<(), _>(js_code.as_str()).unwrap();

        let output: String = ctx.eval("output").unwrap();
        println!("QuickJS output: {:?}", output);

        assert_eq!(output, "hello world\n");
    });
}
