use rquickjs::{Context, Runtime};

#[test]
fn test_hello_world() {
    let output = test_wrapper(
        "src/tests/_hello_world/main.rs",
        "function main() {
let _0;
let _1;
let _2;
let _3;
function bb0() {
_3 = main__promoted_0;
_2 = core__fmt__rt__Arguments__new_const(_3, );
return bb1();
}
function bb1() {
_1 = std__io__print(_2, );
return bb2();
}
function bb2() {
return;
}
bb0();
return _0;
}
const main__promoted_0 = (() => {
let _0;
let _1;
function bb0() {
_1 = [\"hello world!\\n\",];
_0 = _1;
return;
}
bb0();
return _0;
})();
main();",
    );

    assert_eq!(output, "hello world!\n");
}

#[test]
fn test_print_variables() {
    let output = test_wrapper(
        "src/tests/_print_variables/main.rs",
        r#"function main() {
let _0;
let _1;
let _2;
let _3;
let _4;
let _5;
let _6;
let _7;
let _8;
let _9;
let _10;
let _11;
let _12;
let _13;
let _14;
let _15;
let _16;
let _17;
let _18;
function bb0() {
_1 = 5;
_2 = "abc";
_3 = true;
_7 = _1;
_8 = _2;
_9 = _3;
_6 = [_7,_8,_9,];
_16 = _6[0];
_11 = core__fmt__rt__Argument__new_display(_16, );
return bb1();
}
function bb1() {
_17 = _6[1];
_12 = core__fmt__rt__Argument__new_display(_17, );
return bb2();
}
function bb2() {
_18 = _6[2];
_13 = core__fmt__rt__Argument__new_display(_18, );
return bb3();
}
function bb3() {
_10 = [_11,_12,_13,];
_14 = main__promoted_0;
_15 = _10;
_5 = core__fmt__rt__Arguments__new_v1(_14, _15, );
return bb4();
}
function bb4() {
_4 = std__io__print(_5, );
return bb5();
}
function bb5() {
return;
}
bb0();
return _0;
}
const main__promoted_0 = (() => {
let _0;
let _1;
function bb0() {
_1 = ["a: "," b: "," c: ","\n",];
_0 = _1;
return;
}
bb0();
return _0;
})();
main();"#,
    );

    assert_eq!(output, "a: 5 b: abc c: true\n");
}

fn test_wrapper(path: &str, expected_js_code: &str) -> String {
    let js_code = crate::run(path);

    let js_lines = js_code.lines().collect::<Vec<_>>();
    let expected_lines = expected_js_code.lines().collect::<Vec<_>>();

    println!("js_lines: {}", js_lines.join("\n"));
    println!("expected_lines: {}", expected_lines.join("\n"));

    assert_eq!(js_lines.len(), expected_lines.len(),);

    for (js_line, expected_line) in js_lines.into_iter().zip(expected_lines.into_iter()) {
        assert_eq!(js_line, expected_line);
    }

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
        "#;

        ctx.eval::<(), _>(glue).unwrap();

        ctx.eval::<(), _>(js_code.as_str()).unwrap();

        ctx.eval("output").unwrap()
    })
}
