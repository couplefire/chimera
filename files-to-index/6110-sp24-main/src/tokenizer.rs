//! Handles the first phase of the compiler: lexical analysis (scanning).
//! Reads in the characaters of a source file and converts them into tokens.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct SyntaxError {
    message: String,
    pos: InputPosition,
    // the source code at the error line
    code_error_line: String,
}

impl SyntaxError {
    fn new(input: &InputStream, message: &str) -> SyntaxError {
        let code_error_line = input
            .input
            .lines()
            .nth(input.position.line - 1)
            .expect("SyntaxError::new() was given an out-of-bounds error line")
            .to_string();

        SyntaxError {
            message: message.to_string(),
            pos: input.position.clone(),
            code_error_line,
        }
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Syntax Error on line {}:{}: {}\n{}",
            self.pos.line, self.pos.col, self.message, self.code_error_line
        )
    }
}

impl std::error::Error for SyntaxError {}

/// A token represents a single unit of the source code.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Token {
    CharLiteral(char),
    IntLiteralDec(String),
    IntLiteralHex(String),
    BooleanLiteral(bool),
    StringLiteral(String),
    Identifier(String),
    Keyword(String),
    Punctuation(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            Token::CharLiteral(c) => format!("CHARLITERAL {:?}", c),
            Token::IntLiteralDec(s) => format!("INTLITERAL {}", s.clone()),
            Token::IntLiteralHex(s) => format!("INTLITERAL 0x{}", s),
            Token::BooleanLiteral(b) => format!("BOOLEANLITERAL {}", b),
            Token::StringLiteral(s) => format!("STRINGLITERAL {:?}", s),
            Token::Identifier(s) => format!("IDENTIFIER {}", s.clone()),
            Token::Keyword(s) => s.clone(),
            Token::Punctuation(s) => s.clone(),
        };
        write!(f, "{}", out,)
    }
}

/// A token and its position in the input stream.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenInfo {
    pub token: Token,
    pub span: Span,
}

impl fmt::Display for TokenInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.span.start.line, self.token)
    }
}

impl TokenInfo {
    pub fn is_keyword(&self, keyword: &str) -> bool {
        matches!(&self.token, Token::Keyword(x) if x == keyword)
    }

    pub fn is_identifier(&self) -> bool {
        matches!(self.token, Token::Identifier(_))
    }

    pub fn is_punctuation(&self, punctuation: &str) -> bool {
        matches!(&self.token, Token::Punctuation(x) if x == punctuation)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Span {
    // inclusive
    pub start: InputPosition,
    // exclusive
    pub end: InputPosition,
}

struct InputStream {
    input: String,
    position: InputPosition,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputPosition {
    pub index: usize,
    pub line: usize,
    pub col: usize,
}

impl InputStream {
    fn new(input: String) -> InputStream {
        InputStream {
            input,
            position: InputPosition {
                index: 0,
                line: 1,
                col: 0,
            },
        }
    }

    /// Returns the next character. Does not advance the input stream.
    fn peek_char(&self) -> Option<char> {
        self.input.chars().nth(self.position.index)
    }

    fn peek_str(&self, n: usize) -> Option<&str> {
        self.input.get(self.position.index..self.position.index + n)
    }

    /// Moves the input stream forward one character. Panics if the end of the input stream is reached.
    fn advance_char(&mut self) {
        let c = self
            .peek_char()
            .expect("called advance_char() on an empty input stream");
        self.position.index += 1;
        if c == '\n' {
            self.position.line += 1;
            self.position.col = 0;
        } else {
            self.position.col += 1;
        }
    }

    /// Moves the input stream forward n characters. Panics if the end of the input stream is reached.
    fn advance(&mut self, n: usize) {
        for _ in 0..n {
            self.advance_char();
        }
    }
}

/// Returns the next token and advances the input stream.
fn get_token(input: &mut InputStream) -> Result<Option<TokenInfo>, SyntaxError> {
    let punctuation = vec![
        "(", ")", "{", "}", "[", "]", ";", "+", "-", "*", "/", "%", "<", ">", "=", "!", "++", "--",
        "==", "<=", ">=", "/=", "*=", "+=", "-=", "%=", "&&", "||", "!=", ",",
    ];

    // skip any whitespace
    while let Some(c) = input.peek_char() {
        if !c.is_whitespace() {
            break;
        }
        input.advance_char();
    }

    let token_start_pos = input.position.clone();

    // check for two-character tokens
    match input.peek_str(2) {
        Some("//") => {
            // single line comment
            input.advance(2);
            while let Some(c) = input.peek_char() {
                input.advance_char();
                if c == '\n' {
                    break;
                }
            }
            return get_token(input);
        }
        Some("/*") => {
            // multi-line comment
            input.advance(2);
            while let Some(c) = input.peek_str(2) {
                if c == "/*" {
                    return Err(SyntaxError::new(
                        input,
                        "nested multi-line comments are not allowed",
                    ));
                }
                if c == "*/" {
                    input.advance(2);
                    break;
                }
                input.advance_char();
            }
            return get_token(input);
        }
        Some("0x") => {
            input.advance(2);
            let mut s = String::new();
            while let Some(c) = input.peek_char() {
                if c.is_alphanumeric() {
                    s.push(c);
                    input.advance_char();
                } else if c == '_' {
                    input.advance_char();
                } else {
                    break;
                }
            }
            return Ok(Some(TokenInfo {
                token: Token::IntLiteralHex(s),
                span: Span {
                    start: token_start_pos,
                    end: input.position.clone(),
                },
            }));
        }
        Some(token) => {
            if punctuation.contains(&token) {
                let token = token.to_string();
                input.advance(2);
                return Ok(Some(TokenInfo {
                    token: Token::Punctuation(token),
                    span: Span {
                        start: token_start_pos,
                        end: input.position.clone(),
                    },
                }));
            }
        }
        _ => {}
    }

    let token = match input.peek_char() {
        Some('"') => {
            // start of a string
            input.advance_char();
            let mut s = String::new();
            while let Some(c) = get_char_token(input) {
                s.push(c);
            }
            if let Some('\"') = input.peek_char() {
                input.advance_char();
                Token::StringLiteral(s)
            } else {
                return Err(SyntaxError::new(input, "unterminated string literal"));
            }
        }
        Some('\'') => {
            // start of a character literal
            input.advance_char();
            let Some(c) = get_char_token(input) else {
                return Err(SyntaxError::new(input, "invalid character literal"));
            };
            if let Some('\'') = input.peek_char() {
                input.advance_char();
            } else {
                return Err(SyntaxError::new(
                    input,
                    "unterminated character literal; expected '",
                ));
            }
            Token::CharLiteral(c)
        }
        Some('0'..='9') => {
            // start of a decimal number
            let mut s = String::new();
            while let Some(c) = input.peek_char() {
                if c.is_ascii_digit() {
                    s.push(c);
                    input.advance_char();
                } else if c == '_' {
                    input.advance_char();
                } else {
                    break;
                }
            }
            Token::IntLiteralDec(s)
        }
        Some('a'..='z' | 'A'..='Z' | '_') => {
            // start of an identifier or keyword
            let mut s = String::new();
            while let Some(c) = input.peek_char() {
                if c.is_alphanumeric() || c == '_' {
                    s.push(c);
                    input.advance_char();
                } else {
                    break;
                }
            }

            if s == "true" {
                Token::BooleanLiteral(true)
            } else if s == "false" {
                Token::BooleanLiteral(false)
            } else {
                let keywords = HashSet::from([
                    "bool", "break", "const", "import", "continue", "else", "for", "while", "if",
                    "int", "return", "len", "void",
                ]);
                if keywords.contains(s.as_str()) {
                    // it's a keyword
                    Token::Keyword(s)
                } else {
                    // it's an identifier
                    Token::Identifier(s)
                }
            }
        }
        Some(c) => {
            let c_string_ref = &c.to_string()[..];
            if punctuation.contains(&c_string_ref) {
                input.advance(1);
                Token::Punctuation(c.to_string())
            } else {
                return Err(SyntaxError::new(
                    input,
                    &format!("unexpected character: {}", c),
                ));
            }
        }
        None => {
            return Ok(None);
        }
    };

    Ok(Some(TokenInfo {
        token,
        span: Span {
            start: token_start_pos,
            end: input.position.clone(),
        },
    }))
}

/// Returns the next character *token*, or None if the end of the input stream is reached or the next character is not a valid token. Advances the input stream.
fn get_char_token(input: &mut InputStream) -> Option<char> {
    match input.peek_str(2) {
        Some(r"\") => {
            input.advance(2);
            return Some('\'');
        }
        Some(r#"\""#) => {
            input.advance(2);
            return Some('"');
        }
        Some(r"\\") => {
            input.advance(2);
            return Some('\\');
        }
        Some(r"\t") => {
            input.advance(2);
            return Some('\t');
        }
        Some(r"\n") => {
            input.advance(2);
            return Some('\n');
        }
        _ => {}
    }
    let Some(c) = input.peek_char() else {
        return None;
    };
    if c == '"' || c == '\'' || c == '\\' {
        return None;
    }
    if (c as u8) < 32 || (c as u8) > 126 {
        return None;
    }
    input.advance_char();
    Some(c)
}

/// Tokenizes the given input.
///
/// Returns an error if the input cannot be tokenized.
pub fn tokenize(input: String) -> Result<Vec<TokenInfo>, SyntaxError> {
    let mut input = InputStream::new(input);
    let mut tokens = Vec::new();

    while let Some(token) = get_token(&mut input)? {
        tokens.push(token);
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::set_snapshot_suffix;
    use base64::{engine::general_purpose, Engine as _};
    use insta::{assert_yaml_snapshot, with_settings};
    use rstest::rstest;

    #[rstest]
    #[case("'a'")]
    #[case(r"'\n'")]
    #[case(r"'\\'")]
    #[case("'mult'")]
    #[case("!= -- ++ - ! = > >= < <= * / % + - ; [ ] { } ( )")]
    #[case("hello world")]
    #[case("// this is a comment\nint main")]
    #[case("0x123hello")]
    fn test_tokenizer(#[case] input: &str) {
        let mut buf = String::new();
        general_purpose::STANDARD.encode_string(input.as_bytes(), &mut buf);
        set_snapshot_suffix!("{}", buf);

        with_settings!({
            description => format!("Tokenizing {input}")
        }, {
            assert_yaml_snapshot!(tokenize(input.to_string()));
        })
    }

    #[test]
    fn test_tokenizer_int_underscore() {
        let input = "1_000__000";
        let tokens = tokenize(input.to_string()).unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::IntLiteralDec("1000000".to_string()));
    }

    #[test]
    fn test_tokenizer_hex_underscore() {
        let input = "0x1_000__000";
        let tokens = tokenize(input.to_string()).unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token, Token::IntLiteralHex("1000000".to_string()));
    }

    #[test]
    fn test_tokenizer_hex_invalid() {
        // hex literal validation happens during the parsing phase.
        let input = "0x1234098INVALIDinvalid";
        let tokens = tokenize(input.to_string()).unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens[0].token,
            Token::IntLiteralHex("1234098INVALIDinvalid".to_string())
        );
    }
}
