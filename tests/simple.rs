extern crate hermit;
use std::fs;
use hermit::*;

#[test]
fn test_count_kvs() {

    let cases: Vec<(&str, usize)> = vec![
        ("tests/test.toml", 6),
        ("tests/provisioner.toml", 6),
    ];
    for case in cases {
        let raw = fs::read_to_string(case.0).expect("can't read file");
        let kvs = parse::parse_s(raw);
        assert_eq!(kvs.len(), case.1)
    }
}


