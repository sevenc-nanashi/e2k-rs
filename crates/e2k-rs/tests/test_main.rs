#[test]
fn test_main() {
    let src = "constants";

    let c2k = e2k::C2k::new(32);
    let dst = c2k.infer(src);
    dbg!(dst);
}
