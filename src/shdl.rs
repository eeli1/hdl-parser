use crate::{Chip, ComponentIO, Error, LookupTable};

pub fn parse(code: &str) -> Result<(Vec<Chip<ComponentIO>>, Vec<LookupTable>), Error> {
    let mut chips = Vec::new();
    let mut lut = Vec::new();
    Ok((chips, lut))
}
