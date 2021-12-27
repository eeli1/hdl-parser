use hdl_parser::{shdl, Chip, ComponentIO, LookupTable};

#[test]
fn and_from_nand() {
    let code = r"
    chip And {
        in: a, b;
        out: out;
        parts:
            nand = Nand(a, b);
            out = Nand(nand, nand);
    }";

    assert_eq!(
        shdl::parse(code),
        Ok((
            vec![Chip::new(
                "And",
                vec!["a", "b"],
                vec!["out"],
                vec![
                    ComponentIO::new(vec!["a", "b"], vec!["nand"], "Nand"),
                    ComponentIO::new(vec!["nand", "nand"], vec!["out"], "Nand")
                ]
            ),],
            Vec::new()
        ))
    );
}

#[test]
fn and_full() {
    let code = r"
    chip And {
        in: a, b;
        out: out;
        full:
            00 0
            01 0
            10 0
            11 1
    }";

    assert_eq!(
        shdl::parse(code),
        Ok((
            Vec::new(),
            vec![LookupTable::new(
                "And",
                vec!["a", "b"],
                vec!["out"],
                vec![vec![false, false, false, true]]
            )]
        ))
    );
}

#[test]
fn and_fill1() {
    let code = r"
    chip And {
        in: a, b;
        out: out;
        fill1:
            00 0
            01 0
            10 0
    }";

    assert_eq!(
        shdl::parse(code),
        Ok((
            Vec::new(),
            vec![LookupTable::new(
                "And",
                vec!["a", "b"],
                vec!["out"],
                vec![vec![false, false, false, true]]
            )]
        ))
    );
}

#[test]
fn and_fill0() {
    let code = r"
    chip And {
        in: a, b;
        out: out;
        fill0:
            11 1
    }";

    assert_eq!(
        shdl::parse(code),
        Ok((
            Vec::new(),
            vec![LookupTable::new(
                "And",
                vec!["a", "b"],
                vec!["out"],
                vec![vec![false, false, false, true]]
            )]
        ))
    );
}

#[test]
fn and_count() {
    let code = r"
    chip And {
        in: a, b;
        out: out;
        count:
            0001
    }";

    assert_eq!(
        shdl::parse(code),
        Ok((
            Vec::new(),
            vec![LookupTable::new(
                "And",
                vec!["a", "b"],
                vec!["out"],
                vec![vec![false, false, false, true]]
            )]
        ))
    );
}

#[test]
fn and_func() {
    let code = r"
    chip And {
        in: a, b;
        out: out;
        func:
            out = a & b;
    }";

    assert_eq!(
        shdl::parse(code),
        Ok((
            Vec::new(),
            vec![LookupTable::new(
                "And",
                vec!["a", "b"],
                vec!["out"],
                vec![vec![false, false, false, true]]
            )]
        ))
    );
}

#[test]
fn rs_ff() {
    let code = r"
    chip RS_FF {
        in: r, s;
        out: q, q_n;
        parts:
            q = Nand(s, q_n);
            q_n = Nand(r, q);
    }";

    assert_eq!(
        shdl::parse(code),
        Ok((
            vec![Chip::new(
                "RS_FF",
                vec!["r", "s"],
                vec!["q", "q_n"],
                vec![
                    ComponentIO::new(vec!["s", "q_n"], vec!["q"], "Nand"),
                    ComponentIO::new(vec!["s", "q_n"], vec!["q"], "Nand")
                ]
            )],
            Vec::new()
        ))
    );
}

#[test]
fn jk_ff() {
    let code = r"
    chip JK_FF {
        in: j, k, clk;
        out: q, q_n;
        parts:
            j_clk = And(j, clk);
            k_clk = And(k, clk);
            r = And(j_clk, q_n);
            s = And(k_clk, q);
            q, q_n = RS_FF(r, s);
    }";

    assert_eq!(
        shdl::parse(code),
        Ok((
            vec![Chip::new(
                "JK_FF",
                vec!["j", "k", "clk"],
                vec!["q", "q_n"],
                vec![
                    ComponentIO::new(vec!["j", "clk"], vec!["j_clk"], "And"),
                    ComponentIO::new(vec!["k", "clk"], vec!["k_clk"], "And"),
                    ComponentIO::new(vec!["j_clk", "q_n"], vec!["r"], "And"),
                    ComponentIO::new(vec!["k_clk", "q"], vec!["s"], "And"),
                    ComponentIO::new(vec!["r", "s"], vec!["q", "q_n"], "RS_FF"),
                ]
            )],
            Vec::new()
        ))
    );
}

#[test]
fn test() {
    let code = r"
    chip Test {
        in: a[2..5], b, c1;
        out: out0, out[1..4];
        parts:
            x[0..3] = A(a5, b, c1);
            out2, out4 = B(a[2..4]);
            out[0..3] = x0, x[1..3];
            out4 = c1;
    }";

    assert_eq!(
        shdl::parse(code),
        Ok((
            vec![Chip::new(
                "JK_FF",
                vec!["a2", "a3", "a4", "a5", "b", "c1"],
                vec!["out0", "out1", "out2", "out3", "out4", "out5"],
                vec![
                    ComponentIO::new(
                        vec!["a5", "b", "c1"],
                        vec!["x0", "x1", "x2", "x3", "x4"],
                        "A"
                    ),
                    ComponentIO::new(vec!["a2", "a3", "a4"], vec!["out0", "out1", "out2", "out3"], "B"),
                    ComponentIO::new(
                        vec!["x0", "x1", "x2", "x3", "x4"],
                        vec!["out0", "out1", "out2", "out3", "out4"],
                        ""
                    ),
                    ComponentIO::new(vec!["out4"], vec!["c1"], ""),
                ]
            )],
            Vec::new()
        ))
    );
}
