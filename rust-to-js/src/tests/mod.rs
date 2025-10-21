use rquickjs::{Context, Runtime};

#[test]
fn test_hello_world() {
    let output = test_wrapper(
        "src/tests/_hello_world/main.rs",
        include_str!("_hello_world/main.js"),
    );

    assert_eq!(output, "hello world!\n");
}

#[test]
fn test_print_variables() {
    let output = test_wrapper(
        "src/tests/_print_variables/main.rs",
        include_str!("_print_variables/main.js"),
    );

    assert_eq!(output, "a: 5 b: abc c: true\n");
}

#[test]
fn test_struct() {
    let output = test_wrapper("src/tests/_struct/main.rs", include_str!("_struct/main.js"));

    assert_eq!(output, "1 hello 2.3\n");
}

#[test]
fn test_enum() {
    let output = test_wrapper("src/tests/_enum/main.rs", include_str!("_enum/main.js"));

    assert_eq!(output, "MyB MyC: hello MyD: 1.2\n");
}

#[test]
fn test_iter() {
    let output = test_wrapper("src/tests/_iter/main.rs", include_str!("_iter/main.js"));

    assert_eq!(output, "1\n2\n3\n1\n2\n3\n");
}

fn test_wrapper(path: &str, expected_js_code: &str) -> String {
    let js_code = crate::run(path);

    let js_lines = js_code.trim().lines().collect::<Vec<_>>();
    let expected_lines = expected_js_code.trim().lines().collect::<Vec<_>>();

    println!("js_lines: {}", js_lines.join("\n"));
    println!("expected_lines: {}", expected_lines.join("\n"));

    // assert_eq!(js_lines.len(), expected_lines.len());

    // for (js_line, expected_line) in js_lines.into_iter().zip(expected_lines.into_iter()) {
    //     assert_eq!(js_line.trim(), expected_line.trim());
    // }

    let runtime = Runtime::new().unwrap();
    let context = Context::full(&runtime).unwrap();

    context.with(|ctx| {
        let glue = r#"
            let output = "";
            function core__fmt__rt__Argument__new_display(arg) {
                return arg.toString();
            }
            function core__fmt__rt__Arguments__new_v1(format_strings, args) {
                return { format_strings, args };
            }
            function core__fmt__rt__Arguments__new_const(format_strings) {
                return { format_strings, args: [] };
            }
            function std__io__print(args) {
                for (let i = 0; i < args.format_strings.length; i++) {
                    output += args.format_strings[i];
                    if (i < args.args.length) {
                        output += args.args[i];
                    }
                }
            }
            function std__iter__IntoIterator__into_iter(iter) {
                return {
                    iter,
                    next() {
                        return iter.next();
                    }
                };
            }
        "#;

        ctx.eval::<(), _>(glue).unwrap();

        ctx.eval::<(), _>(js_code.as_str()).unwrap();

        ctx.eval("output").unwrap()
    })
}
