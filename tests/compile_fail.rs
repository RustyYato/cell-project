#[test]
#[cfg_attr(miri, ignore)]
fn compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/*.rs");
    t.pass("tests/pass/*.rs");
}
