#[test]
#[cfg_attr(miri, ignore)]
#[cfg_attr(feature = "nightly", ignore)]
fn compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/*.rs");
    t.pass("tests/pass/*.rs");
}
