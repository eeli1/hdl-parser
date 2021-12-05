use crate::{Component, Error, LookupTable};
use logos::{Lexer, Logos};
use std::iter::Peekable;
use std::slice::Iter;

pub struct OGalParse {
    pin_map: Vec<(String, usize)>,
    lookup_table: Vec<LookupTable>,
    dff_enable: Vec<String>,
}

pub fn parse(code: &str) -> Result<OGalParse, Error> {
    let tokens = tokenize(code);
    let mut t_iter = tokens.iter().peekable();

    let mut pin_map = Vec::new();
    let mut lookup_table = Vec::new();
    let mut dff_enable = Vec::new();

    loop {
        if let Some(&token) = t_iter.peek() {
            if token.eq_type(TokenType::Pin) {
                parse_pin(&mut t_iter)?
                    .iter()
                    .for_each(|pin| pin_map.push(pin.to_owned()));
            } else if token.eq_type(TokenType::Table) {
                parse_table(&mut t_iter)?
                    .iter()
                    .for_each(|lt| lookup_table.push(lt.to_owned()));
            } else {
                // no double peek (hope that compiler opimizeis clone of t_iter)
                let mut temp = t_iter.clone();
                get_identifier(temp.next())?;
                if let Some(&token) = temp.peek() {
                    if token.eq_type(TokenType::Dot) {
                        parse_dff(&mut t_iter)?
                            .iter()
                            .for_each(|dff| dff_enable.push(dff.to_owned()));
                    }
                } else {
                    parse_func(&mut t_iter)?
                        .iter()
                        .for_each(|lt| lookup_table.push(lt.to_owned()));
                }
            }
        }

        if t_iter.peek().is_none() {
            break;
        }
    }

    Ok(OGalParse {
        pin_map,
        lookup_table,
        dff_enable,
    })
}

fn parse_dff(t_iter: &mut Peekable<Iter<Token>>) -> Result<Vec<String>, Error> {
    let inputs = get_names(t_iter)?;
    Error::expect(t_iter.next(), TokenType::Dot)?;
    Error::expect(t_iter.next(), TokenType::Dff)?;
    Error::expect(t_iter.next(), TokenType::Semicolon)?;

    Ok(inputs)
}

fn parse_func(t_iter: &mut Peekable<Iter<Token>>) -> Result<Vec<LookupTable>, Error> {
    Ok(Vec::new())
}

fn parse_table(t_iter: &mut Peekable<Iter<Token>>) -> Result<Vec<LookupTable>, Error> {
    Error::expect(t_iter.next(), TokenType::Table)?;
    Error::expect(t_iter.next(), TokenType::OpenP)?;
    let in_names = get_names(t_iter)?;
    Error::expect(t_iter.next(), TokenType::Arrow)?;
    let out_names = get_names(t_iter)?;
    Error::expect(t_iter.next(), TokenType::CloseP)?;

    let mut is_count = false;
    let mut is_fill = false;
    let mut fill = false;

    if let Some(&token) = t_iter.peek() {
        if token.eq_type(TokenType::Dot) {
            Error::expect(t_iter.next(), TokenType::Dot)?;
        }
    }

    if is_count {
    } else if is_fill {
    } else {
    }

    Ok(Vec::new())
}

fn parse_pin(t_iter: &mut Peekable<Iter<Token>>) -> Result<Vec<(String, usize)>, Error> {
    Error::expect(t_iter.next(), TokenType::Pin)?;
    let names = get_names(t_iter)?;
    Ok(Vec::new())
}

// --------------------------------- utils ---------------------------------

fn get_names(t_iter: &mut Peekable<Iter<Token>>) -> Result<Vec<String>, Error> {
    let mut names = get_name(t_iter)?;
    while let Some(&token) = t_iter.peek() {
        if !token.eq_type(TokenType::Comma) {
            break;
        }
        Error::expect(t_iter.next(), TokenType::Comma)?;
        for name in get_name(t_iter)? {
            names.push(name);
        }
    }

    Ok(names)
}

fn get_name(t_iter: &mut Peekable<Iter<Token>>) -> Result<Vec<String>, Error> {
    let identifier = get_identifier(t_iter.next())?;
    if let Some(&token) = t_iter.peek() {
        if !token.eq_type(TokenType::OpenB) {
            return Ok(vec![identifier]);
        }
        Error::expect(t_iter.next(), TokenType::OpenB)?;
        let start = get_num(t_iter.next())?;
        Error::expect(t_iter.next(), TokenType::DoubleDot)?;
        let end = get_num(t_iter.next())? + 1;
        Error::expect(t_iter.next(), TokenType::CloseB)?;
        let mut result = Vec::new();
        for i in start..end {
            result.push(format!("{}{}", identifier, i));
        }
        return Ok(result);
    }
    Ok(vec![identifier])
}

fn get_nums(t_iter: &mut Peekable<Iter<Token>>) -> Result<Vec<usize>, Error> {
    Ok(Vec::new())
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

// ------------------------------- tokens ------------------------------------------------

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

#[derive(Debug, Clone, PartialEq)]
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
pub enum TokenType {
    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Number(usize),
    #[regex(r"[0|1]+", table, priority = 3)]
    BoolTable(Vec<bool>),
    #[regex(r"[a-zA-Z_$][a-zA-Z_$0-9]+", |lex| lex.slice().parse())]
    #[regex(r"[a-zA-Z]", |lex| lex.slice().parse())]
    Identifier(String),
    #[token("\t", ignore)]
    #[token(" ", ignore)]
    #[token("\n", ignore)]
    Ignore(Option<String>),

    #[token("pin")]
    Pin,
    #[token("table")]
    Table,
    #[token("count")]
    Count,
    #[token("fill")]
    Fill,
    #[token("dff")]
    Dff,

    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token("=")]
    Equals,
    #[token(".")]
    Dot,
    #[token("->")]
    Arrow,
    #[token("..")]
    DoubleDot,

    #[token("&")]
    And,
    #[token("|")]
    Or,
    #[token("^")]
    Xor,
    #[token("!")]
    Not,

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

    #[error]
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

fn table(lex: &mut Lexer<TokenType>) -> Option<Vec<bool>> {
    let slice = lex.slice();
    let mut result = Vec::new();
    for c in slice.chars() {
        result.push(c == '1');
    }
    Some(result)
}

impl PartialEq for TokenType {
    fn eq(&self, other: &TokenType) -> bool {
        match (self, other) {
            (TokenType::Number(_), TokenType::Number(_)) => true,
            (TokenType::Identifier(_), TokenType::Identifier(_)) => true,
            (TokenType::Ignore(_), TokenType::Ignore(_)) => true,

            (TokenType::Pin, TokenType::Pin) => true,
            (TokenType::Table, TokenType::Table) => true,
            (TokenType::Count, TokenType::Count) => true,
            (TokenType::Fill, TokenType::Fill) => true,

            (TokenType::CloseB, TokenType::CloseB) => true,
            (TokenType::CloseC, TokenType::CloseC) => true,
            (TokenType::CloseP, TokenType::CloseP) => true,
            (TokenType::OpenB, TokenType::OpenB) => true,
            (TokenType::OpenC, TokenType::OpenC) => true,
            (TokenType::OpenP, TokenType::OpenP) => true,

            (TokenType::Not, TokenType::Not) => true,
            (TokenType::Or, TokenType::Or) => true,
            (TokenType::And, TokenType::And) => true,
            (TokenType::Xor, TokenType::Xor) => true,

            (TokenType::Dff, TokenType::Dff) => true,
            (TokenType::Semicolon, TokenType::Semicolon) => true,
            (TokenType::Comma, TokenType::Comma) => true,
            (TokenType::Equals, TokenType::Equals) => true,
            (TokenType::DoubleDot, TokenType::DoubleDot) => true,
            (TokenType::Arrow, TokenType::Arrow) => true,
            (TokenType::Dot, TokenType::Dot) => true,

            (TokenType::Unknown, TokenType::Unknown) => true,

            _ => false,
        }
    }
}
