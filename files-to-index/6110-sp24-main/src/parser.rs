use std::fmt;

use serde::{Deserialize, Serialize};

use crate::tokenizer::{Span, TokenInfo};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParseError {
    message: String,
    token: Option<TokenInfo>,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // todo: show token info if given
        write!(f, "Parse Error: {}", self.message,)
    }
}

impl std::error::Error for ParseError {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Program {
    import_decls: Vec<ImportDecl>,
    field_decls: Vec<FieldDecl>,
    method_decls: Vec<MethodDecl>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImportDecl {
    id: String,
    // span: Span,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FieldDecl {
    span: Span,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MethodDecl {
    span: Span,
}

struct TokenStream {
    tokens: Vec<TokenInfo>,
    cur_idx: usize,
}

impl TokenStream {
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        Self { tokens, cur_idx: 0 }
    }

    // Returns true if the end of the token stream has been reached.
    pub fn eof(&self) -> bool {
        self.cur_idx >= self.tokens.len()
    }

    // Returns the next token without advacing the stream.
    pub fn peek(&self) -> Result<TokenInfo, ParseError> {
        if self.eof() {
            Err(ParseError {
                message: "Unexpected end of file".to_string(),
                token: None,
            })
        } else {
            Ok(self.tokens.get(self.cur_idx).unwrap().clone())
        }
    }

    // Returns the next token and advances the stream.
    pub fn get_next(&mut self) -> Result<TokenInfo, ParseError> {
        let token = self.peek()?;
        self.cur_idx += 1;
        Ok(token)
    }

    pub fn get_keyword(&mut self, keyword: &str) -> Result<TokenInfo, ParseError> {
        let token = self.get_next()?;
        if !token.is_keyword(keyword) {
            Err(ParseError {
                message: format!("Unexpected token {}; expected keyword {}", token, keyword),
                token: Some(token.clone()),
            })
        } else {
            Ok(token)
        }
    }

    pub fn get_identifier(&mut self) -> Result<TokenInfo, ParseError> {
        let token = self.get_next()?;
        if !token.is_identifier() {
            Err(ParseError {
                message: format!("Unexpected token {}; expected identifier", token),
                token: Some(token.clone()),
            })
        } else {
            Ok(token)
        }
    }

    pub fn get_punctuation(&mut self, punctuation: &str) -> Result<TokenInfo, ParseError> {
        let token = self.get_next()?;
        if !token.is_punctuation(punctuation) {
            Err(ParseError {
                message: format!("Unexpected token {}; expected {}", token, punctuation),
                token: Some(token.clone()),
            })
        } else {
            Ok(token)
        }
    }
}

pub fn parse_program(tokens: Vec<TokenInfo>) -> Result<Program, ParseError> {
    let mut import_decls = vec![];
    let mut tokens = TokenStream::new(tokens);

    // todo: can we assert tokens.peek() here?
    while !tokens.eof() && tokens.peek()?.is_keyword("import") {
        import_decls.push(parse_import_decl(&mut tokens)?);
    }

    Ok(Program {
        import_decls,
        field_decls: vec![],
        method_decls: vec![],
    })
}

fn parse_import_decl(tokens: &mut TokenStream) -> Result<ImportDecl, ParseError> {
    tokens.get_keyword("import")?;
    let identifier = tokens.get_identifier()?;
    tokens.get_punctuation(";")?;
    Ok(ImportDecl {
        id: identifier.to_string(),
    })
}
