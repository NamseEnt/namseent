use chumsky::prelude::*;

/// Parse whitespace and comments
pub fn ws<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> {
    let comment = just("//")
        .then(any().and_is(just('\n').not()).repeated())
        .padded();

    choice((
        comment.to(()),
        text::whitespace().to(()),
    ))
    .repeated()
    .at_least(1)
    .to(())
}

/// Optional whitespace
pub fn ws0<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> + Clone {
    choice((
        text::inline_whitespace().at_least(1).to(()),
        text::newline().to(()),
        just("//").then(any().and_is(just('\n').not()).repeated()).to(()),
    ))
    .repeated()
    .to(())
}

/// Parse an identifier (e.g., _0, _1, main, promoted)
pub fn ident<'a>() -> impl Parser<'a, &'a str, String, extra::Err<Rich<'a, char>>> + Clone {
    text::ident()
        .map(|s: &str| s.to_string())
        .or(just('_')
            .then(text::int(10))
            .to_slice()
            .map(|s: &str| s.to_string()))
}

/// Parse a keyword
pub fn keyword<'a>(kw: &'static str) -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> + Clone {
    just(kw)
        .then_ignore(text::ident().not().rewind())
        .to(())
}

/// Parse a string literal
pub fn string_literal<'a>() -> impl Parser<'a, &'a str, String, extra::Err<Rich<'a, char>>> + Clone {
    just('"')
        .ignore_then(
            none_of('"')
                .or(just('\\').ignore_then(any()))
                .repeated()
                .collect::<String>()
        )
        .then_ignore(just('"'))
}

/// Parse a number
pub fn number<'a>() -> impl Parser<'a, &'a str, usize, extra::Err<Rich<'a, char>>> + Clone {
    text::int(10).from_str().unwrapped()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ident() {
        assert_eq!(ident().parse("_0").into_result(), Ok("_0".to_string()));
        assert_eq!(ident().parse("_1").into_result(), Ok("_1".to_string()));
        assert_eq!(ident().parse("main").into_result(), Ok("main".to_string()));
        assert_eq!(ident().parse("promoted").into_result(), Ok("promoted".to_string()));
    }

    #[test]
    fn test_keyword() {
        assert!(keyword("fn").parse("fn").into_result().is_ok());
        assert!(keyword("let").parse("let").into_result().is_ok());
        assert!(keyword("mut").parse("mut").into_result().is_ok());
        assert!(keyword("const").parse("const").into_result().is_ok());
    }

    #[test]
    fn test_string_literal() {
        assert_eq!(
            string_literal().parse(r#""hello world\n""#).into_result(),
            Ok("hello world\\n".to_string())
        );
    }

    #[test]
    fn test_number() {
        assert_eq!(number().parse("123").into_result(), Ok(123));
        assert_eq!(number().parse("0").into_result(), Ok(0));
    }
}
