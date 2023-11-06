use super::super::token::*;
use super::{
    path::{Path, PathElement},
    ExcludeOperation, IncludeOperation,
};
use crate::*;
use regex::Regex;

pub struct Lexer {
    tokenizer: Tokenizer,
    last_token: Token,
}

#[derive(Debug, PartialEq)]
pub struct ParseResult {
    pub include: Vec<IncludeOperation>,
    pub exclude: Vec<ExcludeOperation>,
}
impl Lexer {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Self {
            tokenizer,
            last_token: Token::EndOfLine,
        }
    }

    pub fn parse(&mut self) -> Result<ParseResult> {
        let mut include: Vec<IncludeOperation> = Vec::new();
        let mut exclude: Vec<ExcludeOperation> = Vec::new();

        loop {
            match self.last_token {
                Token::Exclude => exclude.push(self.parse_exclude_operation()?),
                Token::Comment(_) | Token::EndOfLine => self.next_token(),
                Token::EndOfFile => break Ok(ParseResult { include, exclude }),
                _ => include.push(self.parse_include_operation()?),
            }
        }
    }

    fn last_token(&self) -> Token {
        self.last_token.clone()
    }

    fn next_token(&mut self) {
        self.last_token = self.tokenizer.next();
    }

    fn parse_exclude_operation(&mut self) -> Result<ExcludeOperation> {
        self.next_token();
        match self.last_token {
            Token::Word(_)
            | Token::PathSeparator
            | Token::EndOfLine
            | Token::EndOfFile
            | Token::CurrentDirectory
            | Token::ParentDirectory => Ok(ExcludeOperation::new(self.parse_path(true)?)),
            _ => Err(anyhow!(
                "parse_exclude_operation: Unexpected token {:?}",
                &self.last_token
            )),
        }
    }

    fn parse_include_operation(&mut self) -> Result<IncludeOperation> {
        let src_path = match self.last_token {
            Token::Word(_)
            | Token::Asterisk
            | Token::DoubleAsterisk
            | Token::PathSeparator
            | Token::SrcDestSeparator
            | Token::CurrentDirectory
            | Token::ParentDirectory => self.parse_path(true)?,
            _ => {
                return Err(anyhow!(
                    "parse_include_operation: Unexpected token {:?}",
                    &self.last_token
                ))
            }
        };

        match self.last_token {
            Token::SrcDestSeparator => self.next_token(),
            Token::EndOfLine | Token::EndOfFile => {
                return Ok(IncludeOperation::new(
                    src_path,
                    Path {
                        elements: Vec::new(),
                    },
                ));
            }
            _ => {
                return Err(anyhow!(
                    "parse_include_operation: Unexpected token {:?}",
                    &self.last_token
                ))
            }
        }

        let dest_path = match self.last_token {
            Token::Word(_)
            | Token::Asterisk
            | Token::DoubleAsterisk
            | Token::PathSeparator
            | Token::EndOfLine
            | Token::EndOfFile => self.parse_path(false)?,
            _ => {
                return Err(anyhow!(
                    "parse_include_operation: Unexpected token {:?}",
                    &self.last_token
                ))
            }
        };

        Ok(IncludeOperation::new(src_path, dest_path))
    }

    fn parse_path(&mut self, allow_wildcard: bool) -> Result<Path> {
        let mut elements = Vec::new();
        loop {
            match &self.last_token {
                Token::Word(_)
                | Token::Asterisk
                | Token::DoubleAsterisk
                | Token::CurrentDirectory
                | Token::ParentDirectory => elements.push(self.parse_path_element(allow_wildcard)?),
                Token::PathSeparator => {
                    self.next_token();
                    continue;
                }
                Token::Exclude | Token::Comment(_) => {
                    return Err(anyhow!(
                        "parse_dest_path: Unexpected token {:?}",
                        &self.last_token
                    ))
                }
                Token::SrcDestSeparator | Token::EndOfLine | Token::EndOfFile => {
                    return Ok(Path { elements })
                }
            }
        }
    }

    fn parse_path_element(&mut self, allow_wildcard: bool) -> Result<PathElement> {
        let mut element_name_regex = String::new();
        let mut element_name_raw_string = String::new();
        loop {
            match self.last_token() {
                Token::DoubleAsterisk => {
                    if !allow_wildcard {
                        return Err(anyhow!("parse_path_element: Wildcard not allowed"));
                    }
                    self.next_token();
                    return Ok(PathElement::DoubleAsterisk);
                }
                Token::Word(word) => {
                    self.next_token();
                    element_name_regex += &regex::escape(&word);
                    element_name_raw_string += &word;
                }
                Token::Asterisk => {
                    if !allow_wildcard {
                        return Err(anyhow!("parse_path_element: Wildcard not allowed"));
                    }
                    self.next_token();
                    element_name_regex.push('*');
                    element_name_raw_string.push('*');
                }
                Token::CurrentDirectory => {
                    self.next_token();
                    break Ok(PathElement::CurrentDirectory);
                }
                Token::ParentDirectory => {
                    self.next_token();
                    break Ok(PathElement::ParentDirectory);
                }
                Token::PathSeparator
                | Token::SrcDestSeparator
                | Token::EndOfLine
                | Token::EndOfFile => {
                    break Ok(PathElement::FileOrDir {
                        raw_string: element_name_raw_string,
                        regex: Regex::new(&format!("^{}$", element_name_regex))
                            .map_err(|error| anyhow!("parse_path_element: {}", error))?,
                    })
                }
                _ => {
                    return Err(anyhow!(
                        "parse_path_element: Unexpected token {:?}",
                        &self.last_token
                    ))
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use regex::Regex;

    #[test]
    fn parse_lexicon() {
        let input = "
        # comment

        path/.././to/some*/**/where/\\*\\::
        !over/the /rainbow
        "
        .to_string();

        let tokenizer = Tokenizer::new(input);
        let mut lexer = Lexer::new(tokenizer);
        let result = lexer.parse().unwrap();

        let include_src_path = Path {
            elements: vec![
                PathElement::FileOrDir {
                    raw_string: "path".to_string(),
                    regex: Regex::new("^path$").unwrap(),
                },
                PathElement::ParentDirectory,
                PathElement::CurrentDirectory,
                PathElement::FileOrDir {
                    raw_string: "to".to_string(),
                    regex: Regex::new("^to$").unwrap(),
                },
                PathElement::FileOrDir {
                    raw_string: "some*".to_string(),
                    regex: Regex::new("^some*$").unwrap(),
                },
                PathElement::DoubleAsterisk,
                PathElement::FileOrDir {
                    raw_string: "where".to_string(),
                    regex: Regex::new("^where$").unwrap(),
                },
                PathElement::FileOrDir {
                    raw_string: "*:".to_string(),
                    regex: Regex::new("^\\*:$").unwrap(),
                },
            ],
        };
        let include_dest_path = Path { elements: vec![] };
        let exclude_path = Path {
            elements: vec![
                PathElement::FileOrDir {
                    raw_string: "over".to_string(),
                    regex: Regex::new("^over$").unwrap(),
                },
                PathElement::FileOrDir {
                    raw_string: "the ".to_string(),
                    regex: Regex::new("^the $").unwrap(),
                },
                PathElement::FileOrDir {
                    raw_string: "rainbow".to_string(),
                    regex: Regex::new("^rainbow$").unwrap(),
                },
            ],
        };

        let expected = ParseResult {
            include: vec![IncludeOperation::new(include_src_path, include_dest_path)],
            exclude: vec![ExcludeOperation::new(exclude_path)],
        };
        assert_eq!(expected, result);
    }
}
