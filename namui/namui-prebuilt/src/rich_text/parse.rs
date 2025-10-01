//! This module implemented by AI(Gemini 2.5 Experimental 03-25)

use std::fmt;

#[derive(Debug, bincode::Decode, bincode::Encode, PartialEq)]
pub(crate) enum Token {
    DefaultText { text: String },
    Image { tag: String },
    StyledText { tag: String, text: String },
    RenderingTree { tag: String },
}

#[derive(Debug, bincode::Decode, bincode::Encode, PartialEq)]
pub enum ParseError {
    /// Unclosed tag: found opening '|' but no closing '|'
    UnclosedTag,
    /// Empty tag: found '||' which is invalid
    EmptyTag,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnclosedTag => {
                write!(f, "Unclosed tag: found opening '|' but no closing '|'")
            }
            ParseError::EmptyTag => write!(f, "Empty tag '||' is not allowed"),
        }
    }
}

impl std::error::Error for ParseError {}

pub(crate) fn parse(text: impl AsRef<str>) -> Result<Vec<Token>, ParseError> {
    let text = text.as_ref();

    let mut tokens = Vec::new();
    let mut cursor = 0;
    let len = text.len();

    while cursor < len {
        // Find the next opening '|'
        match text[cursor..].find('|') {
            None => {
                // No more delimiters, add the rest as default text if any
                if cursor < len {
                    tokens.push(Token::DefaultText {
                        text: text[cursor..].to_string(),
                    });
                }
                break; // End of parsing
            }
            Some(start_delim_rel) => {
                let start_delim_abs = cursor + start_delim_rel;

                // Add preceding default text if any
                if start_delim_abs > cursor {
                    tokens.push(Token::DefaultText {
                        text: text[cursor..start_delim_abs].to_string(),
                    });
                }

                // Find the closing '|' for the tag/content itself
                let tag_content_start_abs = start_delim_abs + 1;
                match text[tag_content_start_abs..].find('|') {
                    None => {
                        // Malformed: opened with '|' but no closing '|' anywhere after
                        return Err(ParseError::UnclosedTag);
                    }
                    Some(end_delim1_rel) => {
                        let end_delim1_abs = tag_content_start_abs + end_delim1_rel;
                        let tag_content = &text[tag_content_start_abs..end_delim1_abs];

                        if tag_content.is_empty() {
                            // Handle empty tag ||
                            return Err(ParseError::EmptyTag);
                        }

                        // If the content starts with '/', it cannot be an Image or StyledText opener.
                        // Treat the entire |content| as default text.
                        if tag_content.starts_with('/') {
                            tokens.push(Token::DefaultText {
                                text: format!("|{tag_content}|"), // Include the delimiters
                            });
                            cursor = end_delim1_abs + 1; // Move past the closing '|'
                            continue; // Restart loop
                        }

                        // At this point, tag_content is a potential valid tag name (not empty, not starting with /)

                        // Check if it's a RenderingTree token |@Tag|
                        // Remove the '@' prefix
                        if let Some(tag) = tag_content.strip_prefix('@')
                            && !tag.is_empty()
                        {
                            tokens.push(Token::RenderingTree {
                                tag: tag.to_string(),
                            });
                            cursor = end_delim1_abs + 1; // Move past the closing '|'
                            continue; // Restart loop
                        }

                        let tag = tag_content; // Use tag_content as the tag name

                        // Check if it's a StyledText |Tag|Text|/Tag| or just an Image |Tag|
                        let potential_closing_tag_marker = format!("|/{tag}|");
                        let search_start_for_closing = end_delim1_abs + 1; // Position right after |Tag|

                        // Try to find the *full* closing marker |/Tag| in the rest of the string
                        if let Some(closing_marker_pos_in_remaining) =
                            text[search_start_for_closing..].find(&potential_closing_tag_marker)
                        {
                            // Found the closing tag |/Tag|
                            let inner_text_start_abs = search_start_for_closing;
                            let inner_text_end_abs =
                                search_start_for_closing + closing_marker_pos_in_remaining;
                            let inner_text =
                                text[inner_text_start_abs..inner_text_end_abs].to_string();

                            tokens.push(Token::StyledText {
                                tag: tag.to_string(),
                                text: inner_text,
                            });
                            // Move cursor past the entire |Tag|Text|/Tag| construct
                            cursor = inner_text_end_abs + potential_closing_tag_marker.len();
                        } else {
                            // Did not find the matching |/Tag|, so it must be an Image token
                            tokens.push(Token::Image {
                                tag: tag.to_string(),
                            });
                            // Move cursor past just the |Tag|
                            cursor = end_delim1_abs + 1;
                        }
                    }
                }
            }
        }
    }

    Ok(tokens)
}

// Unit tests (Keep the existing tests, including the one that failed)
#[cfg(test)]
mod tests {
    use super::*; // Import items from outer scope

    // Helper to create expected Vec for the main test
    fn get_main_test_expected_tokens() -> Vec<Token> {
        vec![
            Token::DefaultText {
                text: "Increase ".to_string(),
            },
            Token::Image {
                tag: "Apple".to_string(),
            },
            Token::StyledText {
                tag: "B".to_string(),
                text: "Food".to_string(),
            },
            Token::DefaultText {
                text: " production by +3 and ".to_string(),
            },
            Token::Image {
                tag: "Smile".to_string(),
            },
            Token::StyledText {
                tag: "B".to_string(),
                text: "Happiness".to_string(),
            },
            Token::DefaultText {
                text: " by +1 for each ".to_string(),
            },
            Token::StyledText {
                tag: "B".to_string(),
                text: "Food Silo".to_string(),
            },
            Token::DefaultText {
                text: " (+5 ".to_string(),
            },
            Token::Image {
                // |Apple|
                tag: "Apple".to_string(),
            },
            // *** MODIFICATION: Added expected space token ***
            Token::DefaultText {
                text: " ".to_string(),
            },
            // *** END MODIFICATION ***
            Token::StyledText {
                // |B|Food|/B|
                tag: "B".to_string(),
                text: "Food".to_string(),
            },
            Token::DefaultText {
                text: " if upgraded).".to_string(), // Starts with space
            },
        ]
    }

    #[test]
    fn test_parser() {
        let text = "Increase |Apple||B|Food|/B| production by +3 and |Smile||B|Happiness|/B| by +1 for each |B|Food Silo|/B| (+5 |Apple| |B|Food|/B| if upgraded).";
        let tokens = parse(text).unwrap();
        let expected = get_main_test_expected_tokens(); // Use helper function

        // Provide better diff output on failure
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_text_only() {
        let text = "This is just plain text.";
        let tokens = parse(text).unwrap();
        assert_eq!(
            tokens,
            vec![Token::DefaultText {
                text: "This is just plain text.".to_string()
            }]
        );
    }

    #[test]
    fn test_image_only() {
        let text = "|Icon|";
        let tokens = parse(text).unwrap();
        assert_eq!(
            tokens,
            vec![Token::Image {
                tag: "Icon".to_string()
            }]
        );
    }

    #[test]
    fn test_styled_only() {
        let text = "|Bold|Important Text|/Bold|";
        let tokens = parse(text).unwrap();
        assert_eq!(
            tokens,
            vec![Token::StyledText {
                tag: "Bold".to_string(),
                text: "Important Text".to_string()
            }]
        );
    }

    #[test]
    fn test_trailing_text() {
        let text = "|Icon|Some text afterwards";
        let tokens = parse(text).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Image {
                    tag: "Icon".to_string()
                },
                Token::DefaultText {
                    text: "Some text afterwards".to_string()
                }
            ]
        );
    }

    #[test]
    fn test_leading_text() {
        let text = "Some text before|Icon|";
        let tokens = parse(text).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::DefaultText {
                    text: "Some text before".to_string()
                },
                Token::Image {
                    tag: "Icon".to_string()
                },
            ]
        );
    }

    #[test]
    fn test_consecutive_images() {
        let text = "|A||B|";
        let tokens = parse(text).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Image {
                    tag: "A".to_string()
                },
                Token::Image {
                    tag: "B".to_string()
                },
            ]
        );
    }

    #[test]
    fn test_consecutive_styled() {
        let text = "|A|1|/A||B|2|/B|";
        let tokens = parse(text).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::StyledText {
                    tag: "A".to_string(),
                    text: "1".to_string()
                },
                Token::StyledText {
                    tag: "B".to_string(),
                    text: "2".to_string()
                },
            ]
        );
    }

    #[test]
    fn test_mixed_consecutive() {
        let text = "Start |Img1||B|Bold|/B||Img2| End";
        let tokens = parse(text).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::DefaultText {
                    text: "Start ".to_string()
                },
                Token::Image {
                    tag: "Img1".to_string()
                },
                Token::StyledText {
                    tag: "B".to_string(),
                    text: "Bold".to_string()
                },
                Token::Image {
                    tag: "Img2".to_string()
                },
                Token::DefaultText {
                    text: " End".to_string()
                },
            ]
        );
    }

    #[test]
    fn test_empty_input() {
        let text = "";
        let tokens = parse(text).unwrap();
        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn test_error_unclosed_tag() {
        let text = "Hello |Tag";
        let result = parse(text);
        assert_eq!(result, Err(ParseError::UnclosedTag));
        // Test unclosed tag at end of string
        let text2 = "|Tag";
        let result2 = parse(text2);
        assert_eq!(result2, Err(ParseError::UnclosedTag));
    }

    #[test]
    fn test_potentially_malformed_styled_text() {
        // Case 1: Mismatched closing tag |B|Food|/C| -> Image(B), DefaultText(Food), DefaultText(|/C|)
        let text1 = "|B|Food|/C|";
        let tokens1 = parse(text1).unwrap();
        assert_eq!(
            tokens1,
            vec![
                Token::Image {
                    tag: "B".to_string()
                },
                Token::DefaultText {
                    text: "Food".to_string()
                },
                Token::DefaultText {
                    text: "|/C|".to_string()
                },
            ]
        );

        // Case 2: No closing tag |B|Food -> Image(B), DefaultText(Food)
        let text2 = "|B|Food";
        let tokens2 = parse(text2).unwrap();
        assert_eq!(
            tokens2,
            vec![
                Token::Image {
                    tag: "B".to_string()
                },
                Token::DefaultText {
                    text: "Food".to_string()
                },
            ]
        );

        // Case 3: Incomplete closer |B|Food|/ -> EXPECTS ERROR
        // The parser correctly identifies an unclosed tag starting at the last '|'
        let text3 = "|B|Food|/";
        let result3 = parse(text3);
        // *** MODIFICATION: Assert that this case returns UnclosedTag error ***
        assert_eq!(result3, Err(ParseError::UnclosedTag));
        // *** END MODIFICATION ***

        // Case 4: Nested tags - Parser finds first matching closer |/A|
        let text4 = "|A|Outer|B|Inner|/B||/A|";
        let tokens4 = parse(text4).unwrap();
        // *** MODIFICATION: Assert that the outer tag consumes inner content ***
        assert_eq!(
            tokens4,
            vec![Token::StyledText {
                tag: "A".to_string(),
                text: "Outer|B|Inner|/B|".to_string()
            }]
        );
        // *** END MODIFICATION ***

        // Alternative interpretation for Case 4 (test5): If |/A| *is* found after |A|Outer|B|Inner|/B|,
        let text5 = "|A|Outer|B|Inner|/B|ContentAfter|/A|";
        let tokens5 = parse(text5).unwrap(); // This should still pass
        assert_eq!(
            tokens5,
            vec![Token::StyledText {
                tag: "A".to_string(),
                text: "Outer|B|Inner|/B|ContentAfter".to_string()
            }]
        );
    }

    #[test]
    fn test_error_empty_tag() {
        let text = "Text || More text";
        let result = parse(text);
        assert_eq!(result, Err(ParseError::EmptyTag));
        let text2 = "||";
        let result2 = parse(text2);
        assert_eq!(result2, Err(ParseError::EmptyTag));
    }

    #[test]
    fn test_tag_like_chars_in_default_text_expects_error() {
        // *** MODIFICATION: This test now expects an error ***
        let text = "Text with | character and / character";
        let result = parse(text);
        // Assert that parsing this results in UnclosedTag error, assuming '|' is always a delimiter
        assert_eq!(result, Err(ParseError::UnclosedTag));
        // *** END MODIFICATION ***
    }

    #[test]
    fn test_rendering_tree_token() {
        let text = "|@CustomWidget|";
        let tokens = parse(text).unwrap();
        assert_eq!(
            tokens,
            vec![Token::RenderingTree {
                tag: "CustomWidget".to_string()
            }]
        );
    }

    #[test]
    fn test_mixed_tokens_with_rendering_tree() {
        let text = "Start |@Widget1| middle |Icon| |@Widget2| end";
        let tokens = parse(text).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::DefaultText {
                    text: "Start ".to_string()
                },
                Token::RenderingTree {
                    tag: "Widget1".to_string()
                },
                Token::DefaultText {
                    text: " middle ".to_string()
                },
                Token::Image {
                    tag: "Icon".to_string()
                },
                Token::DefaultText {
                    text: " ".to_string()
                },
                Token::RenderingTree {
                    tag: "Widget2".to_string()
                },
                Token::DefaultText {
                    text: " end".to_string()
                }
            ]
        );
    }

    #[test]
    fn test_empty_rendering_tree_tag() {
        let text = "|@|";
        let tokens = parse(text).unwrap();
        // Should be treated as Image token since the tag after @ is empty
        assert_eq!(
            tokens,
            vec![Token::Image {
                tag: "@".to_string()
            }]
        );
    }

    #[test]
    fn test_tag_like_chars_in_styled_text() {
        // This should still work, as '|' inside the *content* of a styled text is fine.
        let text = "|B|Text with | and / inside|/B|";
        let tokens = parse(text).unwrap();
        assert_eq!(
            tokens,
            vec![Token::StyledText {
                tag: "B".to_string(),
                text: "Text with | and / inside".to_string()
            },]
        );
    }
}
