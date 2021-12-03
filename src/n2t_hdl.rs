use crate::{Chip, Component, Error};
use logos::{Lexer, Logos};
use std::iter::Peekable;
use std::slice::Iter;

pub fn parse(code: &str) -> Result<Vec<Chip>, Error> {
    let tokens = tokenize(code);
    let mut t_iter = tokens.iter().peekable();

    let mut chips = Vec::new();

    let mut name;
    let mut inputs;
    let mut outputs;
    let mut parts;

    loop {
        Error::expect(t_iter.next(), TokenType::Chip)?;
        name = get_identifier(t_iter.next())?;

        Error::expect(t_iter.next(), TokenType::OpenC)?;

        Error::expect(t_iter.next(), TokenType::In)?;
        inputs = get_names(&mut t_iter)?;
        Error::expect(t_iter.next(), TokenType::Out)?;
        outputs = get_names(&mut t_iter)?;

        Error::expect(t_iter.next(), TokenType::Parts)?;
        Error::expect(t_iter.next(), TokenType::Colon)?;
        parts = get_parts(&mut t_iter)?;
        Error::expect(t_iter.next(), TokenType::CloseC)?;

        chips.push(Chip::new_string(name, inputs, outputs, parts));
        if t_iter.peek().is_none() {
            break;
        }
    }
    Ok(chips)
}

fn get_names(t_iter: &mut Peekable<Iter<Token>>) -> Result<Vec<String>, Error> {
    let identifier = get_identifier(t_iter.next())?;
    if let Some(&token) = t_iter.peek() {
        if !token.eq_type(TokenType::OpenB) {
            return Ok(vec![identifier]);
        }
        Error::expect(t_iter.next(), TokenType::OpenB)?;
        let start = get_num(t_iter.next())?;
        Error::expect(t_iter.next(), TokenType::DoubleDot)?;
        let end = get_num(t_iter.next())?;
        Error::expect(t_iter.next(), TokenType::CloseB)?;
        let mut result = Vec::new();
        for i in start..end {
            result.push(format!("{}{}", identifier, i));
        }
        return Ok(result);
    }
    Ok(vec![identifier])
}

fn get_num(token: Option<&Token>) -> Result<usize, Error> {
    if let TokenType::Number(num) = Error::expect(token, TokenType::Number(0))? {
        return Ok(num);
    } else {
        unreachable!();
    }
}

fn get_identifier(token: Option<&Token>) -> Result<String, Error> {
    let token = Error::expect(token, TokenType::Identifier(String::new()))?;
    if let TokenType::Identifier(name) = token {
        return Ok(name);
    } else {
        unreachable!();
    }
}

fn get_parts(t_iter: &mut Peekable<Iter<Token>>) -> Result<Vec<Component>, Error> {
    let mut parts = Vec::new();

    parts.push(get_component(t_iter)?);

    while let Some(&token) = t_iter.peek() {
        if !token.eq_type(TokenType::Identifier(String::new())) {
            break;
        }
        t_iter.next();
        parts.push(get_component(t_iter)?);
    }

    Ok(parts)
}

fn get_component(t_iter: &mut Peekable<Iter<Token>>) -> Result<Component, Error> {
    let chip_name = get_identifier(t_iter.next())?;
    Error::expect(t_iter.next(), TokenType::OpenP)?;

    let mut var_map = get_eq(t_iter)?;

    let mut token = t_iter.next();
    while let Some(t) = token {
        if !t.eq_type(TokenType::Comma) {
            break;
        }
        get_eq(t_iter)?
            .iter()
            .for_each(|temp| var_map.push(temp.to_owned()));
        token = t_iter.next();
    }
    Error::expect(token, TokenType::CloseP)?;
    Error::expect(t_iter.next(), TokenType::Semicolon)?;

    Ok(Component::new_string(var_map, chip_name))
}

fn get_eq(t_iter: &mut Peekable<Iter<Token>>) -> Result<Vec<(String, String)>, Error> {
    let first = get_names(t_iter)?;
    Error::expect(t_iter.next(), TokenType::Equals)?;
    let second = get_names(t_iter)?;

    if first.len() != second.len() {
        todo!();
    }

    let mut var_map = Vec::new();
    for i in 0..first.len() {
        var_map.push((first[i].to_owned(), second[i].to_owned()));
    }
    Ok(var_map)
}

fn tokenize(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut line = 0;
    let mut lex = TokenType::lexer(code);

    while let Some(token_type) = lex.next() {
        if let TokenType::Ignore(ignore) = token_type {
            if let Some(comment) = ignore {
                if comment == "newline".to_string() {
                    line += 1;
                }
            }
        } else {
            tokens.push(Token {
                index: lex.span().start,
                line,
                len: lex.span().len(),
                token_type,
            });
        }
    }

    tokens
}

#[derive(Debug, Clone)]
struct Token {
    index: usize,
    line: usize,
    len: usize,
    token_type: TokenType,
}

impl crate::Token for Token {
    type TokenType = TokenType;
    fn line(&self) -> usize {
        self.line
    }
    fn index(&self) -> usize {
        self.index
    }
    fn len(&self) -> usize {
        self.len
    }
    fn token_type(&self) -> Self::TokenType {
        self.token_type.clone()
    }
}

impl Token {
    fn eq_type(&self, token_type: TokenType) -> bool {
        self.token_type == token_type
    }
}

#[derive(Logos, Debug, Clone)]
enum TokenType {
    #[token("CHIP")]
    Chip,
    #[token("IN")]
    In,
    #[token("OUT")]
    Out,
    #[token("PARTS")]
    Parts,

    #[token("{")]
    OpenC,
    #[token("}")]
    CloseC,
    #[token("(")]
    OpenP,
    #[token(")")]
    CloseP,
    #[token("[")]
    OpenB,
    #[token("]")]
    CloseB,

    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token("=")]
    Equals,
    #[token("..")]
    DoubleDot,
    #[token(":")]
    Colon,

    #[token("\t", ignore)]
    #[token(" ", ignore)]
    #[token("\n", ignore)]
    Ignore(Option<String>),

    #[regex(r"[a-zA-Z_$][a-zA-Z_$0-9]+", |lex| lex.slice().parse())]
    Identifier(String),

    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Number(usize),

    #[error]
    #[regex(r"![a-zA-Z_$0-9]")]
    Unknown,
}

impl PartialEq for TokenType {
    fn eq(&self, other: &TokenType) -> bool {
        match (self, other) {
            (TokenType::Number(_), TokenType::Number(_)) => true,
            (TokenType::Identifier(_), TokenType::Identifier(_)) => true,
            (TokenType::Ignore(_), TokenType::Ignore(_)) => true,

            (TokenType::Chip, TokenType::Chip) => true,
            (TokenType::In, TokenType::In) => true,
            (TokenType::Out, TokenType::Out) => true,
            (TokenType::Parts, TokenType::Parts) => true,

            (TokenType::CloseB, TokenType::CloseB) => true,
            (TokenType::CloseC, TokenType::CloseC) => true,
            (TokenType::CloseP, TokenType::CloseP) => true,
            (TokenType::OpenB, TokenType::OpenB) => true,
            (TokenType::OpenC, TokenType::OpenC) => true,
            (TokenType::OpenP, TokenType::OpenP) => true,

            (TokenType::Colon, TokenType::Colon) => true,
            (TokenType::Semicolon, TokenType::Semicolon) => true,
            (TokenType::Comma, TokenType::Comma) => true,
            (TokenType::Equals, TokenType::Equals) => true,
            (TokenType::DoubleDot, TokenType::DoubleDot) => true,

            (TokenType::Unknown, TokenType::Unknown) => true,

            _ => false,
        }
    }
}

fn ignore(lex: &mut Lexer<TokenType>) -> Option<Option<String>> {
    let slice = lex.slice();
    match slice {
        " " => Some(None),
        "\n" => Some(Some("newline".to_string())),
        "\t" => Some(None),
        _ => Some(Some(slice.to_string())),
    }
}

mod test {
    use super::*;
    #[test]
    fn test_get_name() {
        unimplemented!();
    }
}
