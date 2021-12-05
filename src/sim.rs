use crate::{Chip, Component, Error, LookupTable};
use bool_algebra::bool_to_u32;
use logicsim::graph::{OFF, ON};
use logicsim::{GateGraphBuilder, LeverHandle, OutputHandle};

/// an internal implementation for a lockup table in lib is only the definition
#[derive(Debug, Clone, PartialEq)]
struct LookupTableImpl {
    lut_def: LookupTable,
    in_values: Vec<bool>,
    in_names: Vec<String>,
}

impl LookupTable {
    fn get_name(&self) -> Vec<String> {
        self.inputs.clone()
    }
    fn at_index(&self, index: usize) -> bool {
        self.table[index]
    }
}

impl LookupTableImpl {
    fn new(lut_def: LookupTable) -> Self {
        let in_names = lut_def.get_name();
        LookupTableImpl {
            lut_def,
            in_values: vec![false; in_names.len()],
            in_names,
        }
    }

    fn set(&mut self, name: &str, value: bool) -> Result<(), Error> {
        for (i, in_name) in self.in_names.iter().enumerate() {
            if in_name == name {
                self.in_values[i] = value;
                return Ok(());
            }
        }
        todo!();
        return Err(Error::msg(""));
    }

    fn get(&self) -> bool {
        let index = bool_to_u32(self.in_values.clone());
        self.lut_def.at_index(index as usize)
    }

    fn output(&self) -> String {
        self.lut_def.output.clone()
    }
}
pub struct Circuit {
    chips: Vec<Chip>,
    lut: Vec<LookupTableImpl>,
}

impl Circuit {
    pub fn new(chips: Vec<Chip>, lut: Vec<LookupTable>) -> Result<Self, Error> {
        Ok(Self {
            chips,
            lut: lut
                .iter()
                .map(|lut_def| -> LookupTableImpl { LookupTableImpl::new(lut_def.to_owned()) })
                .collect(),
        })
    }

    pub fn set(&mut self, name: &str, value: bool) -> Result<(), Error> {
        for lt in self.lut.iter_mut() {
            if lt.set(name, value).is_ok() {
                return Ok(());
            }
        }
        todo!();
    }

    pub fn get(&self, name: &str) -> Result<bool, Error> {
        for lt in self.lut.iter() {
            if lt.output() == name {
                return Ok(lt.get());
            }
        }
        todo!();
    }
}

#[test]
fn nand() {
    let mut circuit = Circuit::new(
        Vec::new(),
        vec![LookupTable::new(
            "nand",
            vec!["a", "b"],
            "out",
            vec![true, true, true, false],
        )],
    )
    .unwrap();

    assert_eq!(circuit.get("out"), Ok(true));
    assert_eq!(circuit.set("b", true), Ok(()));
    assert_eq!(circuit.get("out"), Ok(true));
    assert_eq!(circuit.set("b", false), Ok(()));
    assert_eq!(circuit.set("a", true), Ok(()));
    assert_eq!(circuit.get("out"), Ok(true));
    assert_eq!(circuit.set("b", true), Ok(()));
    assert_eq!(circuit.get("out"), Ok(false));
}

#[test]
fn and_from_nand() {
    let chips = vec![Chip::new(
        "And",
        vec!["a", "b"],
        vec!["out"],
        vec![
            Component::new(vec![("a", "a"), ("b", "b"), ("out", "nand")], "Nand"),
            Component::new(vec![("a", "nand"), ("b", "nand"), ("out", "out")], "Nand"),
        ],
    )];

    let nand = LookupTable::new("nand", vec!["a", "b"], "out", vec![true, true, true, false]);
}

#[test]
fn simple_gates() {
    let mut g = GateGraphBuilder::new();

    // Providing each gate with a string name allows for great debugging.
    // If you don't want them affecting performance, you can disable
    // feature "debug_gates" and all of the strings will be optimized away.
    let or = g.or2(ON, OFF, "or");
    let or_output = g.output1(or, "or_output");

    let nand = g.nand2(ON, OFF, "nand");
    let nand_out = g.output1(nand, "nand_out");

    let and = g.and2(ON, OFF, "and");
    let and_output = g.output1(and, "and_output");

    let ig = &g.init();

    // `b0()` accesses the 0th bit of the output.
    // Outputs can have as many bits as you want
    // and be accessed with methods like `u8()`, `char()` or `i128()`.
    assert_eq!(or_output.b0(ig), true);
    assert_eq!(and_output.b0(ig), false);
    assert_eq!(nand_out.b0(ig), true);
    assert_eq!(nand_out.u8(ig), 1);
}

#[test]

fn levers() {
    let mut g = GateGraphBuilder::new();

    let l1 = g.lever("l1");
    let l2 = g.lever("l2");

    let or = g.or2(l1.bit(), l2.bit(), "or");
    let or_output = g.output1(or, "or_output");

    let and = g.and2(l1.bit(), l2.bit(), "and");
    let and_output = g.output1(and, "and_output");

    let ig = &mut g.init();

    assert_eq!(or_output.b0(ig), false);
    assert_eq!(and_output.b0(ig), false);

    // `_stable` means that the graph will run until gate states
    //  have stopped changing. This might not be what you want
    // if you have a circuit that never stabilizes like 3 not gates
    // connected in a loop!
    // See [InitializedGateGraph::run_until_stable].
    ig.flip_lever_stable(l1);
    assert_eq!(or_output.b0(ig), true);
    assert_eq!(and_output.b0(ig), false);

    ig.flip_lever_stable(l2);
    assert_eq!(or_output.b0(ig), true);
    assert_eq!(and_output.b0(ig), true);
}

#[test]
fn sr_latch() {
    let mut g = GateGraphBuilder::new();

    let r = g.lever("l1");
    let s = g.lever("l2");

    let q = g.nor2(r.bit(), OFF, "q");
    let nq = g.nor2(s.bit(), q, "nq");

    let q_output = g.output1(q, "q");
    let nq_output = g.output1(nq, "nq");

    // `d1()` replaces the dependency at index 1 with nq.
    // We used OFF as a placeholder above.
    g.d1(q, nq);

    let ig = &mut g.init();
    // With latches, the initial state should be treated as undefined,
    // so remember to always reset your latches at the beginning
    // of the simulation.
    ig.pulse_lever_stable(r);
    assert_eq!(q_output.b0(ig), false);
    assert_eq!(nq_output.b0(ig), true);

    ig.pulse_lever_stable(s);
    assert_eq!(q_output.b0(ig), true);
    assert_eq!(nq_output.b0(ig), false);

    ig.pulse_lever_stable(r);
    assert_eq!(q_output.b0(ig), false);
    assert_eq!(nq_output.b0(ig), true);
}
