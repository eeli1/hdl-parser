use crate::{Chip, Component, Error};
use logos::{Lexer, Logos};
use std::slice::Iter;

pub fn parse(code: &str) -> Result<Vec<Chip>, Error> {
    let tokens = lex(code);
    let mut t_iter = tokens.iter();

    let mut chips = Vec::new();

    let mut name;
    let mut inputs;
    let mut outputs;
    let mut parts;

    while let Some(token) = t_iter.next() {
        expect(Some(token), TokenType::Chip)?;
        name = get_identifier(t_iter.next())?;

        expect(t_iter.next(), TokenType::OpenC)?;

        expect(t_iter.next(), TokenType::In)?;
        inputs = get_names(&mut t_iter)?;
        expect(t_iter.next(), TokenType::Out)?;
        outputs = get_names(&mut t_iter)?;

        expect(t_iter.next(), TokenType::Parts)?;
        expect(t_iter.next(), TokenType::Colon)?;
        parts = get_parts(&mut t_iter)?;

        chips.push(Chip::new_string(name, inputs, outputs, parts));
    }
    Ok(chips)
}

fn expect(token: Option<&Token>, expected: TokenType) -> Result<TokenType, Error> {
    Ok(TokenType::Unknown)
}

fn get_names(t_iter: &mut Iter<Token>) -> Result<Vec<String>, Error> {
    Ok(Vec::new())
}

fn get_identifier(token: Option<&Token>) -> Result<String, Error> {
    let token = expect(token, TokenType::Identifier(String::new()))?;
    if let TokenType::Identifier(name) = token {
        return Ok(name);
    } else {
        unreachable!();
    }
}

/// note gets parts and also parser TokenType::CloseC
fn get_parts(t_iter: &mut Iter<Token>) -> Result<Vec<Component>, Error> {
    let mut parts = Vec::new();

    parts.push(get_component(t_iter)?);

    let mut token = t_iter.next();
    while let Some(t) = token {
        if !t.eq_type(TokenType::Identifier(String::new())) {
            break;
        }
        parts.push(get_component(t_iter)?);
        token = t_iter.next();
    }

    expect(token, TokenType::CloseC)?;
    Ok(parts)
}

fn get_component(t_iter: &mut Iter<Token>) -> Result<Component, Error> {
    let chip_name = get_identifier(t_iter.next())?;
    expect(t_iter.next(), TokenType::OpenP)?;

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
    expect(token, TokenType::CloseP)?;
    expect(t_iter.next(), TokenType::Semicolon)?;

    Ok(Component::new_string(var_map, chip_name))
}

fn get_eq(t_iter: &mut Iter<Token>) -> Result<Vec<(String, String)>, Error> {
    let first = get_names(t_iter)?;
    expect(t_iter.next(), TokenType::Equals)?;
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

fn lex(code: &str) -> Vec<Token> {
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

struct Token {
    index: usize,
    line: usize,
    len: usize,
    token_type: TokenType,
}

impl Token {
    fn eq_type(&self, token_type: TokenType) -> bool {
        self.token_type == token_type
    }
}

#[derive(Logos, Debug, PartialEq, Clone)]
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

fn ignore(lex: &mut Lexer<TokenType>) -> Option<Option<String>> {
    let slice = lex.slice();
    match slice {
        " " => Some(None),
        "\n" => Some(Some("newline".to_string())),
        "\t" => Some(None),
        _ => Some(Some(slice.to_string())),
    }
}
