pub mod primitives;
pub mod types;
pub mod expr;
pub mod stmt;
pub mod block;
pub mod item;

use crate::ast::Program;
use crate::parser::primitives::ws0;
use crate::parser::item::item_parser;
use chumsky::prelude::*;

/// Parse a complete MIR program
pub fn program_parser<'a>() -> impl Parser<'a, &'a str, Program, extra::Err<Rich<'a, char>>> {
    ws0()
        .ignore_then(
            item_parser()
                .padded_by(ws0())
                .repeated()
                .at_least(1)
                .collect::<Vec<_>>()
        )
        .then_ignore(ws0())
        .map(|items| Program { items })
}

/// Parse MIR text into a Program AST
pub fn parse(input: &str) -> Result<Program, Vec<Rich<char>>> {
    program_parser()
        .parse(input)
        .into_result()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_program() {
        let input = r#"
            fn main() -> () {
                let mut _0: ();

                bb0: {
                    return;
                }
            }
        "#;
        let result = parse(input);
        assert!(result.is_ok());
        let program = result.unwrap();
        assert_eq!(program.items.len(), 1);
    }

    #[test]
    fn test_parse_program_with_const() {
        let input = r#"
            fn main() -> () {
                let mut _0: ();

                bb0: {
                    return;
                }
            }

            const main::promoted[0]: &[&str; 1] = {
                let mut _0: &[&str; 1];

                bb0: {
                    return;
                }
            }
        "#;
        let result = parse(input);
        assert!(result.is_ok());
        let program = result.unwrap();
        assert_eq!(program.items.len(), 2);
    }
}
