use crate::ast::{Item, Function, Const};
use crate::parser::primitives::*;
use crate::parser::types::type_parser;
use crate::parser::stmt::local_parser;
use crate::parser::block::basic_block_parser;
use chumsky::prelude::*;

/// Parse a function name (e.g., "main" or path like "foo::bar")
fn function_name<'a>() -> impl Parser<'a, &'a str, String, extra::Err<Rich<'a, char>>> {
    none_of(" \t\n\r():")
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map(|s| s.trim().to_string())
}

/// Parse a function definition
pub fn function_parser<'a>() -> impl Parser<'a, &'a str, Function, extra::Err<Rich<'a, char>>> {
    keyword("fn")
        .padded_by(ws0())
        .ignore_then(function_name())
        .then_ignore(ws0())
        .then_ignore(just("()"))
        .then_ignore(ws0())
        .then_ignore(just("->"))
        .then_ignore(ws0())
        .then(type_parser())
        .then_ignore(ws0())
        .then_ignore(just('{'))
        .then_ignore(ws0())
        .then(
            local_parser()
                .padded_by(ws0())
                .repeated()
                .collect::<Vec<_>>()
        )
        .then_ignore(ws0())
        .then(
            basic_block_parser()
                .padded_by(ws0())
                .repeated()
                .collect::<Vec<_>>()
        )
        .then_ignore(ws0())
        .then_ignore(just('}'))
        .map(|(((name, return_type), locals), blocks)| Function {
            name,
            return_type,
            locals,
            blocks,
        })
}

/// Parse a const name (e.g., "main::promoted[0]")
fn const_name<'a>() -> impl Parser<'a, &'a str, String, extra::Err<Rich<'a, char>>> {
    // Parse up to ": " (colon followed by whitespace or specific type characters)
    any()
        .and_is(just(':').then(one_of(" \t\n\r&[(")).not())
        .repeated()
        .at_least(1)
        .to_slice()
        .map(|s: &str| s.trim().to_string())
}

/// Parse a const definition
pub fn const_parser<'a>() -> impl Parser<'a, &'a str, Const, extra::Err<Rich<'a, char>>> {
    keyword("const")
        .padded_by(ws0())
        .ignore_then(const_name())
        .then_ignore(just(':'))
        .then_ignore(ws0())
        .then(type_parser())
        .then_ignore(ws0())
        .then_ignore(just('='))
        .then_ignore(ws0())
        .then_ignore(just('{'))
        .then_ignore(ws0())
        .then(
            local_parser()
                .padded_by(ws0())
                .repeated()
                .collect::<Vec<_>>()
        )
        .then_ignore(ws0())
        .then(
            basic_block_parser()
                .padded_by(ws0())
                .repeated()
                .collect::<Vec<_>>()
        )
        .then_ignore(ws0())
        .then_ignore(just('}'))
        .map(|(((name, ty), locals), blocks)| Const {
            name,
            ty,
            locals,
            blocks,
        })
}

/// Parse an item (function or const)
pub fn item_parser<'a>() -> impl Parser<'a, &'a str, Item, extra::Err<Rich<'a, char>>> {
    choice((
        function_parser().map(Item::Function),
        const_parser().map(Item::Const),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_function() {
        let input = r#"fn main() -> () {
            let mut _0: ();

            bb0: {
                return;
            }
        }"#;
        let result = function_parser().parse(input).into_result();
        assert!(result.is_ok());
        let func = result.unwrap();
        assert_eq!(func.name, "main");
        assert_eq!(func.locals.len(), 1);
        assert_eq!(func.blocks.len(), 1);
    }

    #[test]
    fn test_const_definition() {
        let input = r#"const main::promoted[0]: &[&str; 1] = {
            let mut _0: &[&str; 1];

            bb0: {
                return;
            }
        }"#;
        let result = const_parser().parse(input).into_result();
        if let Err(errs) = &result {
            for err in errs {
                eprintln!("Parse error: {}", err);
            }
        }
        assert!(result.is_ok());
        let const_item = result.unwrap();
        assert_eq!(const_item.name, "main::promoted[0]");
    }
}
