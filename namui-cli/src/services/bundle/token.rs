use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Word(String),
    Asterisk,
    DoubleAsterisk,
    CurrentDirectory,
    ParentDirectory,
    PathSeparator,
    SrcDestSeparator,
    Exclude,
    Comment(String),
    EndOfLine,
    EndOfFile,
}

pub struct Tokenizer {
    chars: VecDeque<char>,
    last_char: Option<char>,
}

impl Tokenizer {
    pub fn new(file_string: String) -> Self {
        let chars = file_string.chars().collect();
        Self {
            chars,
            last_char: Some(' '),
        }
    }

    pub fn next(self: &mut Self) -> Token {
        loop {
            match self.last_char {
                Some(char) => match char {
                    ' ' => {
                        self.next_char();
                        continue;
                    }
                    '*' => match self.consume_if_next_char_is_asterisk() {
                        true => break Token::DoubleAsterisk,
                        false => break Token::Asterisk,
                    },
                    '/' => {
                        self.next_char();
                        break Token::PathSeparator;
                    }
                    ':' => {
                        self.next_char();
                        break Token::SrcDestSeparator;
                    }
                    '!' => {
                        self.next_char();
                        break Token::Exclude;
                    }
                    '\n' => {
                        self.next_char();
                        break Token::EndOfLine;
                    }
                    '#' => {
                        let content = self.consume_any_char_until_end_of_line();
                        break Token::Comment(content);
                    }
                    _ => {
                        let content = self.consume_word();
                        break match content.as_str() {
                            "." => Token::CurrentDirectory,
                            ".." => Token::ParentDirectory,
                            _ => Token::Word(content),
                        };
                    }
                },
                None => break Token::EndOfFile,
            };
        }
    }

    fn next_char(self: &mut Self) -> Option<char> {
        self.last_char = self.chars.pop_front();
        self.last_char
    }

    fn consume_if_next_char_is_asterisk(self: &mut Self) -> bool {
        match self.next_char() {
            Some(next_char) => match next_char {
                '*' => {
                    self.next_char();
                    true
                }
                _ => false,
            },
            None => false,
        }
    }

    fn consume_any_char_until_end_of_line(self: &mut Self) -> String {
        let mut content = String::new();
        loop {
            match self.last_char {
                Some(char) => match char {
                    '\n' => break,
                    _ => {
                        content.push(char);
                        self.next_char();
                    }
                },
                None => break,
            }
        }
        content
    }

    fn consume_word(self: &mut Self) -> String {
        let mut content = String::new();
        loop {
            match self.last_char {
                Some(char) => match char {
                    '/' | ':' | '*' | '\n' => break,
                    '\\' => match self.consume_if_next_char_is_escapable_char() {
                        Some(escaped_char) => content.push(escaped_char),
                        None => content.push(char),
                    },
                    _ => {
                        content.push(char);
                        self.next_char();
                    }
                },
                None => break,
            }
        }
        content
    }

    fn consume_if_next_char_is_escapable_char(self: &mut Self) -> Option<char> {
        match self.next_char() {
            Some(next_char) => match next_char {
                ':' | '*' => {
                    self.next_char();
                    Some(next_char)
                }
                _ => None,
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenize() {
        let input = "
        # comment

        path/.././to/some*/**/where/\\*\\::
        !over/the /rainbow
        "
        .to_string();
        let expected_tokens = vec![
            Token::EndOfLine,
            Token::Comment("# comment".to_string()),
            Token::EndOfLine,
            Token::EndOfLine,
            Token::Word("path".to_string()),
            Token::PathSeparator,
            Token::ParentDirectory,
            Token::PathSeparator,
            Token::CurrentDirectory,
            Token::PathSeparator,
            Token::Word("to".to_string()),
            Token::PathSeparator,
            Token::Word("some".to_string()),
            Token::Asterisk,
            Token::PathSeparator,
            Token::DoubleAsterisk,
            Token::PathSeparator,
            Token::Word("where".to_string()),
            Token::PathSeparator,
            Token::Word("*:".to_string()),
            Token::SrcDestSeparator,
            Token::EndOfLine,
            Token::Exclude,
            Token::Word("over".to_string()),
            Token::PathSeparator,
            Token::Word("the ".to_string()),
            Token::PathSeparator,
            Token::Word("rainbow".to_string()),
            Token::EndOfLine,
            Token::EndOfFile,
        ];

        let mut tokenizer = Tokenizer::new(input);
        for expected_token in expected_tokens {
            assert_eq!(tokenizer.next(), expected_token);
        }
    }
}
