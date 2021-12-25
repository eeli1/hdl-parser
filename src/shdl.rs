use crate::{Chip, Error, LookupTable};

pub fn parse(code: &str) -> Result<(Vec<Chip>, Vec<LookupTable>), Error> {
    let mut chips = Vec::new();
    let mut lut = Vec::new();
    Ok((chips, lut))
}
