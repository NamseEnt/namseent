use crate::ast::Type;
use crate::parser::primitives::*;
use chumsky::prelude::*;

/// Parse a type
pub fn type_parser<'a>() -> impl Parser<'a, &'a str, Type, extra::Err<Rich<'a, char>>> + Clone {
    recursive(|ty| {
        let unit = just("()")
            .to(Type::Unit);

        let path = any()
            .and_is(one_of(",;()[]{}").not())
            .repeated()
            .at_least(1)
            .to_slice()
            .map(|s: &str| Type::Path(s.trim().to_string()));

        let array = just('[')
            .ignore_then(ty.clone())
            .then(
                just(';')
                    .padded_by(ws0())
                    .ignore_then(number())
                    .or_not()
            )
            .then_ignore(just(']'))
            .map(|(element, size)| Type::Array {
                element: Box::new(element),
                size,
            });

        let reference = just('&')
            .ignore_then(ty.clone())
            .map(|inner| Type::Ref(Box::new(inner)));

        choice((
            unit.padded_by(ws0()),
            reference.padded_by(ws0()),
            array.padded_by(ws0()),
            path.padded_by(ws0()),
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_type() {
        assert_eq!(
            type_parser().parse("()").into_result(),
            Ok(Type::Unit)
        );
    }

    #[test]
    fn test_array_type() {
        let result = type_parser().parse("[&str; 1]").into_result();
        assert_eq!(
            result,
            Ok(Type::Array {
                element: Box::new(Type::Ref(Box::new(Type::Path("str".to_string())))),
                size: Some(1),
            })
        );
    }

    #[test]
    fn test_ref_type() {
        let result = type_parser().parse("&[&str; 1]").into_result();
        assert_eq!(
            result,
            Ok(Type::Ref(Box::new(Type::Array {
                element: Box::new(Type::Ref(Box::new(Type::Path("str".to_string())))),
                size: Some(1),
            })))
        );
    }

    #[test]
    fn test_path_type() {
        let result = type_parser().parse("std::fmt::Arguments<'_>").into_result();
        assert_eq!(
            result,
            Ok(Type::Path("std::fmt::Arguments<'_>".to_string()))
        );
    }
}
