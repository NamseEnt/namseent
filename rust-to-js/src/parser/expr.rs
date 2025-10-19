use crate::ast::Expr;
use crate::parser::primitives::*;
use chumsky::prelude::*;

/// Parse an expression
pub fn expr_parser<'a>() -> impl Parser<'a, &'a str, Expr, extra::Err<Rich<'a, char>>> {
    recursive(|expr| {
        // Parse "const" keyword followed by a path or string literal
        let const_expr = keyword("const")
            .padded_by(ws0())
            .ignore_then(
                string_literal()
                    .map(|s| format!("\"{}\"", s))
                    .or(none_of(",;(){}=")
                        .repeated()
                        .at_least(1)
                        .to_slice()
                        .map(|s: &str| s.trim().to_string()))
            )
            .map(|s: String| Expr::Const(s));

        // Parse "copy" keyword
        let copy_expr = keyword("copy")
            .padded_by(ws0())
            .ignore_then(ident())
            .map(Expr::Copy);

        // Parse "move" keyword
        let move_expr = keyword("move")
            .padded_by(ws0())
            .ignore_then(ident())
            .map(Expr::Move);

        // Parse array literal: [expr, expr, ...]
        let array_expr = just('[')
            .padded_by(ws0())
            .ignore_then(
                expr.clone()
                    .separated_by(just(',').padded_by(ws0()))
                    .allow_trailing()
                    .collect::<Vec<_>>()
            )
            .then_ignore(ws0())
            .then_ignore(just(']'))
            .map(Expr::Array);

        // Parse reference: &expr
        let ref_expr = just('&')
            .padded_by(ws0())
            .ignore_then(expr.clone())
            .map(|e| Expr::Ref(Box::new(e)));

        // Parse path (function name or variable)
        // This handles complex paths like core::fmt::rt::<impl Arguments<'_>>::new_const::<1>
        // We need to handle nested < > brackets
        let path_expr = any()
            .and_is(one_of(",;(){}=").not())
            .repeated()
            .at_least(1)
            .to_slice()
            .map(|s: &str| Expr::Path(s.trim().to_string()));

        // Primary expression
        let primary = choice((
            const_expr,
            copy_expr,
            move_expr,
            array_expr,
            ref_expr,
            path_expr,
        ));

        // Function call: expr(args)
        let call = primary.clone()
            .then(
                just('(')
                    .padded_by(ws0())
                    .ignore_then(
                        expr.clone()
                            .separated_by(just(',').padded_by(ws0()))
                            .allow_trailing()
                            .collect::<Vec<_>>()
                    )
                    .then_ignore(ws0())
                    .then_ignore(just(')'))
                    .or_not()
            )
            .then(
                // Parse optional "-> [return: bb, unwind continue]"
                ws0()
                    .ignore_then(just("->"))
                    .padded_by(ws0())
                    .ignore_then(just('['))
                    .ignore_then(
                        any()
                            .and_is(just(']').not())
                            .repeated()
                    )
                    .then_ignore(just(']'))
                    .or_not()
                    .map(|_| None::<String>) // Ignore for now, just parse it
            )
            .map(|((func, args), _target)| {
                if let Some(args) = args {
                    Expr::Call {
                        function: Box::new(func),
                        args,
                        target: None,
                    }
                } else {
                    func
                }
            });

        call
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const_expr() {
        let result = expr_parser().parse("const main::promoted[0]").into_result();
        assert_eq!(result, Ok(Expr::Const("main::promoted[0]".to_string())));
    }

    #[test]
    fn test_copy_expr() {
        let result = expr_parser().parse("copy _3").into_result();
        assert_eq!(result, Ok(Expr::Copy("_3".to_string())));
    }

    #[test]
    fn test_move_expr() {
        let result = expr_parser().parse("move _2").into_result();
        assert_eq!(result, Ok(Expr::Move("_2".to_string())));
    }

    #[test]
    fn test_array_expr() {
        let result = expr_parser().parse(r#"[const "hello world\n"]"#).into_result();
        if let Err(errs) = &result {
            for err in errs {
                eprintln!("Parse error: {}", err);
            }
        }
        assert!(result.is_ok());
    }

    #[test]
    fn test_ref_expr() {
        let result = expr_parser().parse("&_1").into_result();
        assert_eq!(result, Ok(Expr::Ref(Box::new(Expr::Path("_1".to_string())))));
    }

    #[test]
    fn test_call_expr() {
        let result = expr_parser().parse("_print(move _2)").into_result();
        assert!(result.is_ok());
        if let Ok(Expr::Call { function, args, .. }) = result {
            assert_eq!(*function, Expr::Path("_print".to_string()));
            assert_eq!(args.len(), 1);
        }
    }
}
