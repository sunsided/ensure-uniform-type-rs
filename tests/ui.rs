//! Compile-time tests: `tests/ui/pass` must compile, `tests/ui/fail` must fail with the recorded error.

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/pass/*.rs");
    t.compile_fail("tests/ui/fail/*.rs");
}
