use crate::{Chip, Error};

use logos::Logos;

pub fn parse(code: &str) -> Result<Vec<Chip>, Error> {
    Ok(Vec::new())
}

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[token("CHIP")]
    Chip, // CHIP
    #[token("IN")]
    In, // IN
    #[token("OUT")]
    Out, // OUT
    #[token("PARTS")]
    Parts, // PARTS
    #[token("{")]
    OpenC, // {
    #[token("}")]
    CloseC, // }
    #[token("(")]
    OpenP, // (
    #[token(")")]
    CloseP, // )
    #[token("[")]
    OpenB, // [
    #[token("]")]
    CloseB, // ]

    #[token(",")]
    Comma, // ,
    #[token(";")]
    Semicolon, // ;
    #[token("=")]
    Equals, // =
    #[token("..")]
    DoubleDot, // ..

    // #[regex(r"[ \t\n\f]+", logos::skip)]
    // Ignore(Option<String>), // _
    // Identifier(String), // [a-Z][0-9]
    // Number(usize),      // [0-9]
    #[error]
    Unknown, //   Unknown(String), // ?
}

fn lex(code: Vec<char>) {}
