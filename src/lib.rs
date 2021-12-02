pub mod n2t_hdl;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Bit {
    One,
    Zero,
    X,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chip {
    name: String,
    inputs: Vec<String>,
    outputs: Vec<String>,
    parts: Vec<Component>,
}

impl Chip {
    pub fn new(name: &str, inputs: Vec<&str>, outputs: Vec<&str>, parts: Vec<Component>) -> Self {
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct LookupTable {
    name: String,
    inputs: Vec<String>,
    outputs: Vec<String>,
    table: Vec<Vec<Bit>>,
}

impl LookupTable {
    pub fn new(name: &str, inputs: Vec<&str>, outputs: Vec<&str>, table: Vec<Vec<Bit>>) -> Self {
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
            table,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Component {
    inputs: Vec<(String, String)>,
    outputs: Vec<(String, String)>,
    chip_name: String,
}

impl Component {
    pub fn new(inputs: Vec<(&str, &str)>, outputs: Vec<(&str, &str)>, chip_name: &str) -> Self {
        Self {
            inputs: inputs
                .iter()
                .map(|&(s1, s2)| -> (String, String) { (s1.to_string(), s2.to_string()) })
                .collect(),
            outputs: outputs
                .iter()
                .map(|&(s1, s2)| -> (String, String) { (s1.to_string(), s2.to_string()) })
                .collect(),
            chip_name: chip_name.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Error {}
