use hdl_parser::{shdl, Chip, Component, LookupTable};

#[test]
fn and_from_nand() {
    let code = r#"
    chip And {
        in: a, b;
        out: out;
        parts:
            nand = Nand(a, b);
            out = Nand(nand, nand);
    }
    "#;

    let result = shdl::parse(code);
    // let parts = vec![
    //     Component::new(vec![("a", "a"), ("b", "b"), ("out", "nand")], "Nand"),
    //     Component::new(vec![("a", "nand"), ("b", "nand"), ("out", "out")], "Nand"),
    // ];

    assert_eq!(
        result,
        Ok((
            vec![Chip::new("And", vec!["a", "b"], vec!["out"], Vec::new())],
            Vec::new()
        ))
    );
}

#[test]
fn and_full() {
    let code = r#"
    chip And {
        in: a, b;
        out: out;
        full:
            00 0
            01 0
            10 0
            11 1
    }
    "#;

    let result = shdl::parse(code);

    assert_eq!(
        result,
        Ok((
            Vec::new(),
            vec![LookupTable::new(
                "And",
                vec!["a", "b"],
                vec!["out"],
                vec![false, false, false, true]
            )]
        ))
    );
}

#[test]
fn and_fill1() {
    let code = r#"
    chip And {
        in: a, b;
        out: out;
        fill1:
            00 0
            01 0
            10 0
    }
    "#;

    let result = shdl::parse(code);

    assert_eq!(
        result,
        Ok((
            Vec::new(),
            vec![LookupTable::new(
                "And",
                vec!["a", "b"],
                vec!["out"],
                vec![false, false, false, true]
            )]
        ))
    );
}

#[test]
fn and_fill0() {
    let code = r#"
    chip And {
        in: a, b;
        out: out;
        fill0:
            11 1
    }
    "#;

    let result = shdl::parse(code);

    assert_eq!(
        result,
        Ok((
            Vec::new(),
            vec![LookupTable::new(
                "And",
                vec!["a", "b"],
                vec!["out"],
                vec![false, false, false, true]
            )]
        ))
    );
}

#[test]
fn and_count() {
    let code = r#"
    chip And {
        in: a, b;
        out: out;
        count:
            0001
    }
    "#;

    let result = shdl::parse(code);

    assert_eq!(
        result,
        Ok((
            Vec::new(),
            vec![LookupTable::new(
                "And",
                vec!["a", "b"],
                vec!["out"],
                vec![false, false, false, true]
            )]
        ))
    );
}

#[test]
fn and_func() {
    let code = r#"
    chip And {
        in: a, b;
        out: out;
        func:
            out = a & b;
    }
    "#;

    let result = shdl::parse(code);

    assert_eq!(
        result,
        Ok((
            Vec::new(),
            vec![LookupTable::new(
                "And",
                vec!["a", "b"],
                vec!["out"],
                vec![false, false, false, true]
            )]
        ))
    );
}

#[test]
fn jk_ff() {
    unimplemented!();
}
