use crate::{Chip, Error};

use logos::{Lexer, Logos};

pub fn parse(code: &str) -> Result<Vec<Chip>, Error> {
    let tokens = TokenType::lexer(code);
    Ok(Vec::new())
}

struct Token {
    index: usize,
    token_type: TokenType,
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

    #[regex(r"[ \t\n\f]+", ignore)]
    Ignore(Option<String>),

    #[regex(r"[a-zA-Z_$][a-zA-Z_$0-9]+", |lex| lex.slice().parse())]
    Identifier(String), // ^[a-zA-Z_$][a-zA-Z_$0-9]*$

    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Number(usize), // [0-9]

    #[error]
    #[regex(r"![a-zA-Z_$0-9]+")] //, |lex| lex.slice().to_string())]
    Unknown, // ?
}

fn ignore(lex: &mut Lexer<TokenType>) -> Option<Option<String>> {
    let slice = lex.slice();
    match slice {
        " " => Some(None),
        "\n" => Some(None),
        "\t" => Some(None),
        _ => Some(Some(slice.to_string())),
    }
}

#[test]
fn test_lex() {
    let code = r#"
    CHIP And {
        IN a12, b3;
        OUT 12 out;
     PARTS:
        () []
    }
    "#;
    let mut tokens = Vec::new();
    for t in TokenType::lexer(code) {
        tokens.push(t);
    }
    assert_eq!(
        tokens,
        vec![
            TokenType::Ignore(Some("\n    ".to_string())),
            TokenType::Chip,
            TokenType::Ignore(None),
            TokenType::Identifier("And".to_string()),
            TokenType::Ignore(None),
            TokenType::OpenC,
            TokenType::Ignore(Some("\n        ".to_string())),
            TokenType::In,
            TokenType::Ignore(None),
            TokenType::Identifier("a12".to_string()),
            TokenType::Comma,
            TokenType::Ignore(None),
            TokenType::Identifier("b3".to_string()),
            TokenType::Semicolon,
            TokenType::Ignore(Some("\n        ".to_string())),
            TokenType::Out,
            TokenType::Ignore(None),
            TokenType::Number(12),
            TokenType::Ignore(None),
            TokenType::Identifier("out".to_string()),
            TokenType::Semicolon,
            TokenType::Ignore(Some("\n     ".to_string())),
            TokenType::Parts,
            TokenType::Unknown,
            TokenType::Ignore(Some("\n        ".to_string())),
            TokenType::OpenP,
            TokenType::CloseP,
            TokenType::Ignore(None),
            TokenType::OpenB,
            TokenType::CloseB,
            TokenType::Ignore(Some("\n    ".to_string())),
            TokenType::CloseC,
            TokenType::Ignore(Some("\n    ".to_string()))
        ]
    )
}
