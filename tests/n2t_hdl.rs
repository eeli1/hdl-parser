use hdl_parser::{n2t_hdl, Chip, Component};

#[test]
fn std_gates() {
    let code = r#"
    CHIP And {
        IN a, b;
        OUT out;
     PARTS:
        Nand(a=a, b=b, out=nand);
        Nand(a=nand, b=nand, out=out);
    }

    CHIP Not {
        IN a;
        OUT out;
     PARTS:
        Nand(a=a, b=a, out=out);
    }

    CHIP Or {
        IN a, b;
        OUT out;
     PARTS:
        Not(a=a, out=not_a);
        Not(a=b, out=not_b);
        Nand(a=not_a, b=not_b, out=out);
    }

    CHIP Xor {
        IN a, b;
        OUT out;
     PARTS:
        Not(a=a, out=not_a);
        Not(a=b, out=not_b);
        And(a=a, b=not_b, out=and_a);
        And(a=not_a, b=b, out=and_b);
        Or(a=and_a, b=and_b, out=out);
    }
    "#;

    let chips = n2t_hdl::parse(code);
    let parts = vec![
        Component::new(vec![("a", "a"), ("b", "b"), ("out", "nand")], "Nand"),
        Component::new(vec![("a", "nand"), ("b", "nand"), ("out", "out")], "Nand"),
    ];
    assert_eq!(
        chips,
        Ok(vec![Chip::new("and", vec!["a", "b"], vec!["out"], parts)])
    );
}

#[test]
fn std_flip_flop() {
    let code = r#"
    CHIP RS_FF {
        IN R, S;
        OUT Q, Q_n;
     PARTS:
        Not(a=R, out=R_n);
        Not(a=S, out=S_n);
        Nand(a=R_n, b=Q_n, out=Q);
        Nand(a=S_n, b=Q, out=Q_n);
    }

    CHIP JK_FF {
        IN J, K, Clk;
        OUT Q, Q_n;
     PARTS:
        And(a=J, b=Clk, out=J_Clk);
        And(a=K, b=Clk, out=K_Clk);

        And(a=J_Clk, b=Q_n, out=R);
        And(a=K_Clk, b=Q, out=S);

        RS_FF(R=R, S=S, Q=Q, Q_n=Q_n);
    }

    CHIP D_FF {
        IN D, Clk;
        OUT Q, Q_n;
     PARTS:
        Not(a=D, out=D_n);

        And(a=D, b=Clk, out=D_Clk);
        And(a=D_n, b=Clk, out=D_n_Clk);

        RS_FF(R=D_Clk, S=D_n_Clk, Q=Q, Q_n=Q_n);
    }
    "#;

    let chips = n2t_hdl::parse(code);
    let parts = vec![
        Component::new(vec![("a", "a"), ("b", "b"), ("out", "nand")], "Nand"),
        Component::new(vec![("a", "nand"), ("b", "nand"), ("out", "out")], "Nand"),
    ];
    assert_eq!(
        chips,
        Ok(vec![Chip::new("and", vec!["a", "b"], vec!["out"], parts)])
    );
}
