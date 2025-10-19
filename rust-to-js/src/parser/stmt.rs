use crate::ast::{Local, Statement};
use crate::parser::primitives::*;
use crate::parser::types::type_parser;
use crate::parser::expr::expr_parser;
use chumsky::prelude::*;

/// Parse a local variable declaration
pub fn local_parser<'a>() -> impl Parser<'a, &'a str, Local, extra::Err<Rich<'a, char>>> {
    keyword("let")
        .padded_by(ws0())
        .ignore_then(keyword("mut").padded_by(ws0()).or_not())
        .then(ident())
        .then_ignore(ws0())
        .then_ignore(just(':'))
        .then_ignore(ws0())
        .then(type_parser())
        .then_ignore(ws0())
        .then_ignore(just(';'))
        .map(|((is_mut, name), ty)| Local {
            mutable: is_mut.is_some(),
            name,
            ty,
        })
}

/// Parse a statement
pub fn statement_parser<'a>() -> impl Parser<'a, &'a str, Statement, extra::Err<Rich<'a, char>>> {
    let return_stmt = keyword("return")
        .padded_by(ws0())
        .then_ignore(just(';'))
        .to(Statement::Return);

    let assign_stmt = ident()
        .then_ignore(ws0())
        .then_ignore(just('='))
        .then_ignore(ws0())
        .then(expr_parser())
        .then_ignore(ws0())
        .then_ignore(just(';'))
        .map(|(target, value)| Statement::Assign { target, value });

    choice((return_stmt, assign_stmt))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Type, Expr};

    #[test]
    fn test_local_immutable() {
        let result = local_parser().parse("let _0: ();").into_result();
        assert_eq!(
            result,
            Ok(Local {
                mutable: false,
                name: "_0".to_string(),
                ty: Type::Unit,
            })
        );
    }

    #[test]
    fn test_local_mutable() {
        let result = local_parser().parse("let mut _1: ();").into_result();
        assert_eq!(
            result,
            Ok(Local {
                mutable: true,
                name: "_1".to_string(),
                ty: Type::Unit,
            })
        );
    }

    #[test]
    fn test_return_stmt() {
        let result = statement_parser().parse("return;").into_result();
        assert_eq!(result, Ok(Statement::Return));
    }

    #[test]
    fn test_assign_stmt() {
        let result = statement_parser().parse("_3 = const main::promoted[0];").into_result();
        assert_eq!(
            result,
            Ok(Statement::Assign {
                target: "_3".to_string(),
                value: Expr::Const("main::promoted[0]".to_string()),
            })
        );
    }
}
