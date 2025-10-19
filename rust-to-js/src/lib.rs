pub mod ast;
pub mod parser;
pub mod codegen;

/// Transpile MIR code to JavaScript
pub fn transpile(mir_content: &str) -> String {
    match parser::parse(mir_content) {
        Ok(program) => codegen::generate(&program),
        Err(errors) => {
            eprintln!("Parse errors:");
            for error in &errors {
                eprintln!("{}", error);
            }
            panic!("Failed to parse MIR");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpile_simple() {
        let mir = r#"
            fn main() -> () {
                let mut _0: ();

                bb0: {
                    return;
                }
            }
        "#;

        let js = transpile(mir);
        assert!(js.contains("function main()"));
        assert!(js.contains("return"));
    }
}
