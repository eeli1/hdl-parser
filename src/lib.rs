use std::fmt::Debug;

pub mod comphdl;
pub mod n2t_hdl;
pub mod open_gal;
pub mod shdl;

#[derive(Debug, Clone, PartialEq)]
pub struct Chip<T> {
    name: String,
    inputs: Vec<String>,
    outputs: Vec<String>,
    parts: Vec<T>,
}

impl<T> Chip<T> {
    pub fn new(name: &str, inputs: Vec<&str>, outputs: Vec<&str>, parts: Vec<T>) -> Self {
        Self {
            name: name.to_string(),
            inputs: inputs
                .iter()
                .map(|&s| -> String { s.to_string() })
                .collect(),
            outputs: outputs
                .iter()
                .map(|&s| -> String { s.to_string() })
                .collect(),
            parts,
        }
    }

    pub fn new_string(
        name: String,
        inputs: Vec<String>,
        outputs: Vec<String>,
        parts: Vec<T>,
    ) -> Self {
        Self {
            name,
            inputs,
            outputs,
            parts,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LookupTable {
    name: String,
    inputs: Vec<String>,
    output: Vec<String>,
    table: Vec<Vec<bool>>,
}

impl LookupTable {
    pub fn new(name: &str, inputs: Vec<&str>, output: Vec<&str>, table: Vec<Vec<bool>>) -> Self {
        Self {
            name: name.to_string(),
            inputs: inputs
                .iter()
                .map(|&s| -> String { s.to_string() })
                .collect(),
            output: output
                .iter()
                .map(|&s| -> String { s.to_string() })
                .collect(),
            table,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComponentMap {
    var_map: Vec<(String, String)>,
    name: String,
}

impl ComponentMap {
    pub fn new(var_map: Vec<(&str, &str)>, name: &str) -> Self {
        Self {
            var_map: var_map
                .iter()
                .map(|&(s1, s2)| -> (String, String) { (s1.to_string(), s2.to_string()) })
                .collect(),
            name: name.to_string(),
        }
    }

    pub fn new_string(var_map: Vec<(String, String)>, name: String) -> Self {
        Self { var_map, name }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComponentIO {
    inputs: Vec<String>,
    ouputs: Vec<String>,
    name: String,
}

impl ComponentIO {
    pub fn new(inputs: Vec<&str>, ouputs: Vec<&str>, name: &str) -> Self {
        Self {
            inputs: inputs
                .iter()
                .map(|&s| -> String { s.to_string() })
                .collect(),
            ouputs: ouputs
                .iter()
                .map(|&s| -> String { s.to_string() })
                .collect(),
            name: name.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Component {
    inputs: Vec<(String, String)>,
    ouputs: Vec<(String, String)>,
    name: String,
}

impl Component {
    pub fn new(inputs: Vec<(&str, &str)>, ouputs: Vec<(&str, &str)>, name: &str) -> Self {
        Self {
            inputs: inputs
                .iter()
                .map(|&(s1, s2)| -> (String, String) { (s1.to_string(), s2.to_string()) })
                .collect(),

            ouputs: ouputs
                .iter()
                .map(|&(s1, s2)| -> (String, String) { (s1.to_string(), s2.to_string()) })
                .collect(),
            name: name.to_string(),
        }
    }
}

pub trait Token: Clone {
    type TokenType: PartialEq + Debug;
    fn line(&self) -> usize;
    fn index(&self) -> usize;
    fn len(&self) -> usize;
    fn token_type(&self) -> Self::TokenType;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    line: Option<usize>,
    index: Option<usize>,
    len: Option<usize>,
    msg: String,
}

impl Error {
    pub fn expect<T: Token>(got: Option<&T>, expected: T::TokenType) -> Result<T::TokenType, Self> {
        if let Some(token) = got {
            let token = token.clone();
            if token.token_type() == expected {
                Ok(token.token_type())
            } else {
                Err(Self {
                    line: Some(token.line()),
                    index: Some(token.index()),
                    len: Some(token.len()),
                    msg: format!(
                        "unexpected token expected <{:?}> but got <{:?}>",
                        expected,
                        token.token_type()
                    ),
                })
            }
        } else {
            Err(Self {
                line: None,
                index: None,
                len: None,
                msg: format!("unexpected end of file expected token <{:?}> ", expected),
            })
        }
    }

    pub fn msg_token<T: Token>(msg: &str, token: T) -> Self {
        Self {
            line: Some(token.line()),
            index: Some(token.index()),
            len: Some(token.len()),
            msg: msg.to_string(),
        }
    }

    pub fn msg_len<T: Token>(msg: &str, token: T, len: usize) -> Self {
        Self {
            line: Some(token.line()),
            index: Some(token.index()),
            len: Some(len),
            msg: msg.to_string(),
        }
    }

    pub fn msg(msg: &str) -> Self {
        Self {
            line: None,
            index: None,
            len: None,
            msg: msg.to_string(),
        }
    }
}
