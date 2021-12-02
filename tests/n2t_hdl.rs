use hdl_parser::{n2t_hdl, Chip, Component};

#[test]
#[ignore = "not implemented"]
fn and() {
    let code = r#"
    CHIP And {
        IN a, b;
        OUT out;
     PARTS:
        Nand(a=a, b=b, out=nand);
        Nand(a=nand, b=nand, out=out);
    }
    "#;

    let chips = n2t_hdl::parse(code);
    let parts = vec![
        Component::new(vec![("a", "a"), ("b", "b")], vec![("out", "nand")], "Nand"),
        Component::new(
            vec![("a", "nand"), ("b", "nand")],
            vec![("out", "out")],
            "Nand",
        ),
    ];
    assert_eq!(
        chips,
        Ok(vec![Chip::new("and", vec!["a", "b"], vec!["out"], parts)])
    );
}
