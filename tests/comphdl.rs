use hdl_parser::{comphdl::parse, Chip, ComponentIO};

// examples from https://badel2.github.io/comphdl/demo/v10/

// -------------------------------------------------- example1.txt --------------------------------------------------

#[test]
fn buf() {
    let code = "component Buf (d) -> q { d=q; }";
    let component = Chip::new(
        "Buf",
        vec!["d"],
        vec!["q"],
        vec![ComponentIO::new(vec!["d"], vec!["q"], "")],
    );

    assert_eq!(parse(code), Ok(vec![component]));
}

#[test]
fn not() {
    let code = "component Not (a) -> q { Nand(a) -> q; }";
    let component = Chip::new(
        "Not",
        vec!["a"],
        vec!["q"],
        vec![ComponentIO::new(vec!["a"], vec!["q"], "Nand")],
    );

    assert_eq!(parse(code), Ok(vec![component]));
}

#[test]
fn or2() {
    let code = r"
    component Or2(a, b) -> x {
        Not(a) -> n_a;
        Not(b) -> n_b;
        Nand(n_a, n_b) -> x;
    }";
    let component = Chip::new(
        "Or2",
        vec!["a", "b"],
        vec!["x"],
        vec![
            ComponentIO::new(vec!["a"], vec!["n_a"], "Not"),
            ComponentIO::new(vec!["b"], vec!["n_b"], "Not"),
            ComponentIO::new(vec!["a"], vec!["q"], "Nand"),
        ],
    );

    assert_eq!(parse(code), Ok(vec![component]));
}

#[test]
fn mux_4_1() {
    let code = r"
    component Mux_4_1(s1, s0, a, b, c, d) -> y {
        Buf  (s1) -> d_s1;
        Buf  (s0) -> d_s0;
        Not  (s1) -> n_s1;
        Not  (s0) -> n_s0;
        Buf(a) -> d_a; Buf(b) -> d_b; Buf(c) -> d_c; Buf(d) -> d_d;
        Nand (n_s0, n_s1, d_a) -> sel00;
        Nand (d_s0, n_s1, d_b) -> sel01;
        Nand (n_s0, d_s1, d_c) -> sel10;
        Nand (d_s0, d_s1, d_d) -> sel11;
        Nand (sel00, sel01, sel10, sel11) -> y;
    }";
    let component = Chip::new(
        "Or2",
        vec!["a", "b"],
        vec!["x"],
        vec![
            ComponentIO::new(vec!["a"], vec!["n_a"], "Not"),
            ComponentIO::new(vec!["b"], vec!["n_b"], "Not"),
            ComponentIO::new(vec!["a"], vec!["q"], "Nand"),
        ],
    );

    assert_eq!(parse(code), Ok(vec![component]));
}

#[test]
fn xor2() {
    let code = r"
    component Xor2(a, b) -> x {
        ConstantBit() -> (_0, _1, _X);
        Mux_4_1(a, b, _0, _1, _1, _0) -> x;
    }";
    let component = Chip::new(
        "Or2",
        vec!["a", "b"],
        vec!["x"],
        vec![
            ComponentIO::new(vec!["a"], vec!["n_a"], "Not"),
            ComponentIO::new(vec!["b"], vec!["n_b"], "Not"),
            ComponentIO::new(vec!["a"], vec!["q"], "Nand"),
        ],
    );

    assert_eq!(parse(code), Ok(vec![component]));
}

#[test]
fn and3() {
    let code = r"
    component And3(a, b, c) -> x {
        Nand(a, b, c) -> n_x;
        Not(n_x) -> x;
    }";
    let component = Chip::new(
        "Or2",
        vec!["a", "b"],
        vec!["x"],
        vec![
            ComponentIO::new(vec!["a"], vec!["n_a"], "Not"),
            ComponentIO::new(vec!["b"], vec!["n_b"], "Not"),
            ComponentIO::new(vec!["a"], vec!["q"], "Nand"),
        ],
    );

    assert_eq!(parse(code), Ok(vec![component]));
}

#[test]
fn demux_1_4() {
    let code = r"
    component Demux_1_4(s1, s0, i) -> (f0, f1, f2, f3) {
        Buf(i)  -> d_i;
        Buf(s1) -> d_s1; Buf(s0) -> d_s0;
        Not(s1) -> n_s1; Not(s0) -> n_s0;
        And3(n_s1, n_s0, d_i) -> f0;
        And3(n_s1, d_s0, d_i) -> f1;
        And3(d_s1, n_s0, d_i) -> f2;
        And3(d_s1, d_s0, d_i) -> f3;
    }";
    let component = Chip::new(
        "Or2",
        vec!["a", "b"],
        vec!["x"],
        vec![
            ComponentIO::new(vec!["a"], vec!["n_a"], "Not"),
            ComponentIO::new(vec!["b"], vec!["n_b"], "Not"),
            ComponentIO::new(vec!["a"], vec!["q"], "Nand"),
        ],
    );

    assert_eq!(parse(code), Ok(vec![component]));
}

#[test]
fn array_1d() {
    let code = r"
    component Array1D(a[3:0], b[3:0]) -> x[1:0] {
        Nand(a[3:0], b[3:0]) -> x[1:1];
        Nand(x[1:1]) -> x[0:0];
    }";
    let component = Chip::new(
        "Or2",
        vec!["a", "b"],
        vec!["x"],
        vec![
            ComponentIO::new(vec!["a"], vec!["n_a"], "Not"),
            ComponentIO::new(vec!["b"], vec!["n_b"], "Not"),
            ComponentIO::new(vec!["a"], vec!["q"], "Nand"),
        ],
    );

    assert_eq!(parse(code), Ok(vec![component]));
}

#[test]
fn mux_16_4() {
    let code = r"
    component Mux_16_4(s[1:0], a[0:15]) -> y[0:3] {
        Mux_4_1(s[1:0], a[0:3]) -> y[0];
        Mux_4_1(s[1:0], a[4:7]) -> y[1];
        Mux_4_1(s[1:0], a[8:11]) -> y[2];
        Mux_4_1(s[1:0], a[12:15]) -> y[3];
    }";
    let component = Chip::new(
        "Or2",
        vec!["a", "b"],
        vec!["x"],
        vec![
            ComponentIO::new(vec!["a"], vec!["n_a"], "Not"),
            ComponentIO::new(vec!["b"], vec!["n_b"], "Not"),
            ComponentIO::new(vec!["a"], vec!["q"], "Nand"),
        ],
    );

    assert_eq!(parse(code), Ok(vec![component]));
}

#[test]
fn mux_16_1() {
    let code = r"
    component Mux_16_1(s[3:0], a[0:15]) -> y {
        Mux_16_4(s[1:0], a[0:15]) -> j[0:3];
        Mux_4_1(s[3:2], j[0:3]) -> y;
    }";
    let component = Chip::new(
        "Or2",
        vec!["a", "b"],
        vec!["x"],
        vec![
            ComponentIO::new(vec!["a"], vec!["n_a"], "Not"),
            ComponentIO::new(vec!["b"], vec!["n_b"], "Not"),
            ComponentIO::new(vec!["a"], vec!["q"], "Nand"),
        ],
    );

    assert_eq!(parse(code), Ok(vec![component]));
}

#[test]
fn quad() {
    let code = r"
    component Quad(a) -> b[3:0] {
        (a, a, a, a) = b[3:0];
    }";
    let component = Chip::new(
        "Or2",
        vec!["a", "b"],
        vec!["x"],
        vec![
            ComponentIO::new(vec!["a"], vec!["n_a"], "Not"),
            ComponentIO::new(vec!["b"], vec!["n_b"], "Not"),
            ComponentIO::new(vec!["a"], vec!["q"], "Nand"),
        ],
    );

    assert_eq!(parse(code), Ok(vec![component]));
}

#[test]
fn mux_test() {
    let code = r"
    component MuxTest(s[3:0], a, b) -> (x, y) {
        Quad(a) -> aaaa[3:0];
        Quad(b) -> bbbb[3:0];
        Mux_16_1(s[3:0], aaaa[3:0], aaaa[3:0], bbbb[3:0], bbbb[3:0]) -> x;
        Mux_16_1(s[3:0], aaaa[3:0], bbbb[3:0], aaaa[3:0], bbbb[3:0]) -> y;
    }";
    let component = Chip::new(
        "Or2",
        vec!["a", "b"],
        vec!["x"],
        vec![
            ComponentIO::new(vec!["a"], vec!["n_a"], "Not"),
            ComponentIO::new(vec!["b"], vec!["n_b"], "Not"),
            ComponentIO::new(vec!["a"], vec!["q"], "Nand"),
        ],
    );

    assert_eq!(parse(code), Ok(vec![component]));
}

// -------------------------------------------------- bufbufbuf --------------------------------------------------
/* bufbufbuf.txt

component MultiBuf(a) -> (x1, x2, x3, x4, x5, x6) {
    Buf(a) -> x0;
    Buf60(x0) -> x1;
    Buf60(x1) -> x2;
    Buf60(x2) -> x3;
    Buf60(x3) -> x4;
    Buf60(x4) -> x5;
    Buf60(x5) -> x6;
}

component Buf(a) -> x {
    x = a;
}

component Buf4(a) -> x {
    Buf(a) -> a1;
    Buf(a1) -> a2;
    Buf(a2) -> a3;
    Buf(a3) -> x;
}

component Buf10(a) -> x {
    Buf(a) -> a1;
    Buf4(a1) -> a2;
    Buf4(a2) -> a3;
    Buf(a3) -> x;
}

component Buf60(a) -> x {
    Buf10(a) -> a1;
    Buf10(a1) -> a2;
    Buf10(a2) -> a3;
    Buf10(a3) -> a4;
    Buf10(a4) -> a5;
    Buf10(a5) -> x;
}

*/

// -------------------------------------------------- ram --------------------------------------------------
/* ram.txt
component Buf (d) -> q { d=q; }
component Not (a) -> q { Nand(a) -> q; }

component And3(a, b, c) -> x {
    Nand(a, b, c) -> n_x;
    Not(n_x) -> x;
}

component Mux_4_1(s1, s0, a, b, c, d) -> y {
    Buf  (s1) -> d_s1;
    Buf  (s0) -> d_s0;
    Not  (s1) -> n_s1;
    Not  (s0) -> n_s0;
    Buf(a) -> d_a; Buf(b) -> d_b; Buf(c) -> d_c; Buf(d) -> d_d;
    Nand (n_s0, n_s1, d_a) -> sel00;
    Nand (d_s0, n_s1, d_b) -> sel01;
    Nand (n_s0, d_s1, d_c) -> sel10;
    Nand (d_s0, d_s1, d_d) -> sel11;
    Nand (sel00, sel01, sel10, sel11) -> y;
}

component Demux_1_4(s1, s0, i) -> (f0, f1, f2, f3) {
    Buf(i)  -> d_i;
    Buf(s1) -> d_s1; Buf(s0) -> d_s0;
    Not(s1) -> n_s1; Not(s0) -> n_s0;
    And3(n_s1, n_s0, d_i) -> f0;
    And3(n_s1, d_s0, d_i) -> f1;
    And3(d_s1, n_s0, d_i) -> f2;
    And3(d_s1, d_s0, d_i) -> f3;
}

component RSLatch_raw(n_R, n_S) -> Q {
    Nand(n_S, n_Q) -> Q;
    Nand(n_R, Q) -> n_Q;
}

component DLatch(enable, d) -> q {
    // set = enable and d
    // reset = enable and not d
    Nand(enable, d) -> n_S;
    Nand(n_S, enable) -> n_R;
    RSLatch_raw(n_R, n_S) -> q;
}

component Register8(enable, d[7:0]) -> q[7:0] {
    DLatch(enable, d[7]) -> q[7];
    DLatch(enable, d[6]) -> q[6];
    DLatch(enable, d[5]) -> q[5];
    DLatch(enable, d[4]) -> q[4];
    DLatch(enable, d[3]) -> q[3];
    DLatch(enable, d[2]) -> q[2];
    DLatch(enable, d[1]) -> q[1];
    DLatch(enable, d[0]) -> q[0];
}

component Register32(enable, d[31:0]) -> q[31:0] {
    Register8(enable, d[31:24]) -> q[31:24];
    Register8(enable, d[23:16]) -> q[23:16];
    Register8(enable, d[15:8]) -> q[15:8];
    Register8(enable, d[7:0]) -> q[7:0];
}

component Mux_32_8(sel[1:0], a[7:0], b[7:0], c[7:0], d[7:0]) -> y[7:0] {
    Mux_4_1(sel[1:0], a[0], b[0], c[0], d[0]) -> y[0];
    Mux_4_1(sel[1:0], a[1], b[1], c[1], d[1]) -> y[1];
    Mux_4_1(sel[1:0], a[2], b[2], c[2], d[2]) -> y[2];
    Mux_4_1(sel[1:0], a[3], b[3], c[3], d[3]) -> y[3];
    Mux_4_1(sel[1:0], a[4], b[4], c[4], d[4]) -> y[4];
    Mux_4_1(sel[1:0], a[5], b[5], c[5], d[5]) -> y[5];
    Mux_4_1(sel[1:0], a[6], b[6], c[6], d[6]) -> y[6];
    Mux_4_1(sel[1:0], a[7], b[7], c[7], d[7]) -> y[7];
}

component Ram4x8(write, addr[1:0], d[7:0]) -> q[7:0] {
    Demux_1_4(addr[1:0], write) -> w[0:3];
    Register8(w[0], d[7:0]) -> qa[7:0];
    Register8(w[1], d[7:0]) -> qb[7:0];
    Register8(w[2], d[7:0]) -> qc[7:0];
    Register8(w[3], d[7:0]) -> qd[7:0];
    Mux_32_8(addr[1:0], qa[7:0], qb[7:0], qc[7:0], qd[7:0]) -> q[7:0];
}

component Ram16x8(write, addr[3:0], d[7:0]) -> q[7:0] {
    Demux_1_4(addr[3:2], write) -> w[0:3];
    Ram4x8(w[0], addr[1:0], d[7:0]) -> qa[7:0];
    Ram4x8(w[1], addr[1:0], d[7:0]) -> qb[7:0];
    Ram4x8(w[2], addr[1:0], d[7:0]) -> qc[7:0];
    Ram4x8(w[3], addr[1:0], d[7:0]) -> qd[7:0];
    Mux_32_8(addr[3:2], qa[7:0], qb[7:0], qc[7:0], qd[7:0]) -> q[7:0];
}

component Ram64x8(write, addr[5:0], d[7:0]) -> q[7:0] {
    Demux_1_4(addr[5:4], write) -> w[0:3];
    Ram16x8(w[0], addr[3:0], d[7:0]) -> qa[7:0];
    Ram16x8(w[1], addr[3:0], d[7:0]) -> qb[7:0];
    Ram16x8(w[2], addr[3:0], d[7:0]) -> qc[7:0];
    Ram16x8(w[3], addr[3:0], d[7:0]) -> qd[7:0];
    Mux_32_8(addr[5:4], qa[7:0], qb[7:0], qc[7:0], qd[7:0]) -> q[7:0];
}

component Ram256x8(write, addr[7:0], d[7:0]) -> q[7:0] {
    Demux_1_4(addr[7:6], write) -> w[0:3];
    Ram64x8(w[0], addr[5:0], d[7:0]) -> qa[7:0];
    Ram64x8(w[1], addr[5:0], d[7:0]) -> qb[7:0];
    Ram64x8(w[2], addr[5:0], d[7:0]) -> qc[7:0];
    Ram64x8(w[3], addr[5:0], d[7:0]) -> qd[7:0];
    Mux_32_8(addr[7:6], qa[7:0], qb[7:0], qc[7:0], qd[7:0]) -> q[7:0];
}

component Ram1024x8(write, addr[9:0], d[7:0]) -> q[7:0] {
    Demux_1_4(addr[9:8], write) -> w[0:3];
    Ram256x8(w[0], addr[7:0], d[7:0]) -> qa[7:0];
    Ram256x8(w[1], addr[7:0], d[7:0]) -> qb[7:0];
    Ram256x8(w[2], addr[7:0], d[7:0]) -> qc[7:0];
    Ram256x8(w[3], addr[7:0], d[7:0]) -> qd[7:0];
    Mux_32_8(addr[9:8], qa[7:0], qb[7:0], qc[7:0], qd[7:0]) -> q[7:0];
}

component Ram4096x8(write, addr[11:0], d[7:0]) -> q[7:0] {
    Demux_1_4(addr[11:10], write) -> w[0:3];
    Ram1024x8(w[0], addr[9:0], d[7:0]) -> qa[7:0];
    Ram1024x8(w[1], addr[9:0], d[7:0]) -> qb[7:0];
    Ram1024x8(w[2], addr[9:0], d[7:0]) -> qc[7:0];
    Ram1024x8(w[3], addr[9:0], d[7:0]) -> qd[7:0];
    Mux_32_8(addr[11:10], qa[7:0], qb[7:0], qc[7:0], qd[7:0]) -> q[7:0];
}

component Ram16384x8(write, addr[13:0], d[7:0]) -> q[7:0] {
    Demux_1_4(addr[13:12], write) -> w[0:3];
    Ram4096x8(w[0], addr[11:0], d[7:0]) -> qa[7:0];
    Ram4096x8(w[1], addr[11:0], d[7:0]) -> qb[7:0];
    Ram4096x8(w[2], addr[11:0], d[7:0]) -> qc[7:0];
    Ram4096x8(w[3], addr[11:0], d[7:0]) -> qd[7:0];
    Mux_32_8(addr[13:12], qa[7:0], qb[7:0], qc[7:0], qd[7:0]) -> q[7:0];
}

component Ram65536x8(write, addr[15:0], d[7:0]) -> q[7:0] {
    Demux_1_4(addr[15:14], write) -> w[0:3];
    Ram16384x8(w[0], addr[13:0], d[7:0]) -> qa[7:0];
    Ram16384x8(w[1], addr[13:0], d[7:0]) -> qb[7:0];
    Ram16384x8(w[2], addr[13:0], d[7:0]) -> qc[7:0];
    Ram16384x8(w[3], addr[13:0], d[7:0]) -> qd[7:0];
    Mux_32_8(addr[15:14], qa[7:0], qb[7:0], qc[7:0], qd[7:0]) -> q[7:0];
}
*/

// -------------------------------------------------- srlatch --------------------------------------------------

/* srlatch.txt

component nSnRLatch(n_S, n_R) -> (Q, n_Q) {
    Nand(n_S, n_Q) -> Q;
    Nand(n_R, Q) -> n_Q;
}

component SRLatch(E, S, R) -> Q {
    Nand(E, S) -> n_S;
    Nand(E, R) -> n_R;
    nSnRLatch(n_S, n_R) -> (Q, n_Q);
}

component DLatch(E, D) -> Q {
    Nand(D, E) -> n_S;
    Nand(n_S, E) -> n_R;
    nSnRLatch(n_S, n_R) -> (Q, n_Q);
}

component Reg4(E, D3, D2, D1, D0) -> (Q3, Q2, Q1, Q0) {
    DLatch(E, D3) -> Q3;
    DLatch(E, D2) -> Q2;
    DLatch(E, D1) -> Q1;
    DLatch(E, D0) -> Q0;
}

component Reg8(E, D[7:0]) -> Q[7:0] {
    Reg4(E, D[7:4]) -> Q[7:4];
    Reg4(E, D[3:0]) -> Q[3:0];
}

*/

/* cat.txt
component Catv0(clk) -> eof {
    Stdin(clk) -> (eof, x[7:0]);
    Stdout(clk, x[7:0]);
}

component Cat(enable) -> (eof3) {
    Clk2(enable) -> clk;
    Stdin(clk) -> (eof, x[7:0]);
    Buf(eof) -> eof1;
    Buf(eof1) -> eof2;
    Buf(eof2) -> eof3;
    Nand(eof) -> neof;
    Buf(clk) -> bclk1;
    Buf(bclk1) -> bclk;
    Nand(neof, bclk) -> nclk2;
    Nand(nclk2) -> clk2;
    Bufw8(x[7:0]) -> x1[7:0];
    Bufw8(x1[7:0]) -> x2[7:0];
    Bufw8(x2[7:0]) -> x3[7:0];
    Stdout(clk2, x3[7:0]);
}

component Buf(a) -> x {
    a = x;
}

component Bufw8(a[7:0]) -> x[7:0] {
    Buf(a[0]) -> x[0];
    Buf(a[1]) -> x[1];
    Buf(a[2]) -> x[2];
    Buf(a[3]) -> x[3];
    Buf(a[4]) -> x[4];
    Buf(a[5]) -> x[5];
    Buf(a[6]) -> x[6];
    Buf(a[7]) -> x[7];
}

component Clk4(a) -> x1 {
    Nand(a, x3) -> x;
    Nand(x) -> x1;
    Nand(x1) -> x2;
    Buf(x2) -> x3;
}

component Clk2(a) -> x {
    Nand(a, x) -> x;
}
*/
