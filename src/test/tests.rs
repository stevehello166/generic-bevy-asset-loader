use std::path::PathBuf;

#[test]
pub fn test_expand() {
    macrotest::expand(Into::<PathBuf>::into("test/expand/*.rs"));
}