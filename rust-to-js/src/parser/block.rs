use crate::ast::BasicBlock;
use crate::parser::primitives::*;
use crate::parser::stmt::statement_parser;
use chumsky::prelude::*;

/// Parse a basic block (e.g., bb0: { statements })
pub fn basic_block_parser<'a>() -> impl Parser<'a, &'a str, BasicBlock, extra::Err<Rich<'a, char>>> {
    ident()
        .then_ignore(ws0())
        .then_ignore(just(':'))
        .then_ignore(ws0())
        .then_ignore(just('{'))
        .then_ignore(ws0())
        .then(
            statement_parser()
                .padded_by(ws0())
                .repeated()
                .collect::<Vec<_>>()
        )
        .then_ignore(ws0())
        .then_ignore(just('}'))
        .map(|(label, statements)| BasicBlock { label, statements })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Statement;

    #[test]
    fn test_basic_block() {
        let input = r#"bb0: {
            _3 = const main::promoted[0];
            return;
        }"#;
        let result = basic_block_parser().parse(input).into_result();
        assert!(result.is_ok());
        let block = result.unwrap();
        assert_eq!(block.label, "bb0");
        assert_eq!(block.statements.len(), 2);
    }

    #[test]
    fn test_empty_block() {
        let input = "bb2: { return; }";
        let result = basic_block_parser().parse(input).into_result();
        assert!(result.is_ok());
        let block = result.unwrap();
        assert_eq!(block.label, "bb2");
        assert_eq!(block.statements.len(), 1);
        assert_eq!(block.statements[0], Statement::Return);
    }
}
