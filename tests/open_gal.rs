use hdl_parser::{open_gal, open_gal::OGalParse, LookupTable};

#[test]
fn full() {
    let code = r"
    table(i0, i1 -> and, or) {
        00 00
        01 01
        10 01
        11 11
    }";

    assert_eq!(
        open_gal::parse(code),
        Ok(OGalParse::lut(vec![LookupTable::new(
            "",
            vec!["i0", "i1"],
            vec!["and"],
            vec![
                vec![false, false, false, true],
                vec![false, true, true, true]
            ]
        ),]))
    );
}

#[test]
fn easy_gal() {
    let code = r"
    pin 13 = i0;
    pin 11 = i1;
    pin 17 = and;
    pin 18 = or;
    pin 19 = xor;

    table(i0, i1 -> and) {
        00 0
        01 0
        10 0
        11 1
    }

    table(i0, i1 -> xor).count {
        0110
    }

    table(i0, i1 -> or).fill(1) {
        00 0
        01 1
        10 1
    }

    pin 23 = a;
    pin 3 = b;
    pin 2 = c;

    a = (!b | (c));
    a.dff;";

    assert_eq!(
        open_gal::parse(code),
        Ok(OGalParse::new(
            vec![
                ("i0", 13),
                ("i1", 11),
                ("and", 17),
                ("or", 18),
                ("xor", 19),
                ("a", 23),
                ("b", 3),
                ("c", 2)
            ],
            vec![
                LookupTable::new(
                    "",
                    vec!["i0", "i1"],
                    vec!["and"],
                    vec![vec![false, false, false, true]]
                ),
                LookupTable::new(
                    "",
                    vec!["i0", "i1"],
                    vec!["or"],
                    vec![vec![false, true, true, true]]
                ),
                LookupTable::new(
                    "",
                    vec!["i0", "i1"],
                    vec!["xor"],
                    vec![vec![false, true, true, false]]
                ),
                LookupTable::new(
                    "",
                    vec!["b", "c"],
                    vec!["a"],
                    vec![vec![true, true, false, true]]
                )
            ],
            vec!["a"]
        ))
    );
}

#[test]
fn open_gal() {
    let code = r"
    pin 1, 2 = i[0..1];
    pin [13..16] = and, or, xor, not;
    table(i0, i1 -> and).fill(0) {
        11 1
    }
    
    table(i0, i1 -> or).fill(1) {
        00 0
    }
    
    table(i0, i1 -> xor).count {
        0110
    }
    
    table(i0 -> not) {
        01
        10
    }";

    assert_eq!(
        open_gal::parse(code),
        Ok(OGalParse::new(
            vec![
                ("i0", 1),
                ("i1", 2),
                ("and", 13),
                ("or", 14),
                ("xor", 15),
                ("not", 16)
            ],
            vec![
                LookupTable::new(
                    "",
                    vec!["i0", "i1"],
                    vec!["and"],
                    vec![vec![false, false, false, true]]
                ),
                LookupTable::new(
                    "",
                    vec!["i0", "i1"],
                    vec!["or"],
                    vec![vec![false, true, true, true]]
                ),
                LookupTable::new(
                    "",
                    vec!["i0", "i1"],
                    vec!["xor"],
                    vec![vec![false, true, true, false]]
                ),
                LookupTable::new("", vec!["i0", "i1"], vec!["not"], vec![vec![true, false]])
            ],
            Vec::new()
        ))
    );
}
