#![cfg(feature = "deref")]

extern crate numeric_float;
use numeric_float::n32f;

#[test]
fn test_deref() {
    let n = n32f::from_float(1.0).unwrap();
    assert!(!n.is_nan());
}
