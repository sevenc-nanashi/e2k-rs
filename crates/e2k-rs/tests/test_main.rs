#[test]
fn test_c2k() {
    let src = "constants";

    let c2k = e2k::C2k::new(32);
    let dst = c2k.infer(src);
    dbg!(dst);
}

#[test]
fn test_p2k() {
    let p2k = e2k::P2k::new(32);
    let pronunciation = ["K", "AA1", "N", "S", "T", "AH0", "N", "T", "S"];
    let dst = p2k.infer(&pronunciation);
    dbg!(dst);
}
