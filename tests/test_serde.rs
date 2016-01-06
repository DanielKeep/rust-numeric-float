#![cfg(feature="serde")]

extern crate numeric_float;
extern crate serde;
extern crate serde_json;

#[macro_use] mod util;

use std::error::Error;
use numeric_float::n32f;
use serde::{Deserialize, Serialize};
use serde_json as json;
use util::corner_cases as ccs;

#[test]
fn test_rustc_serialize_roundtrip() {
    println!("test_rustc_serialize_roundtrip:");
    for case in ccs::<f32>() {
        println!("- {:?}", case);

        let f_rt: f32 = match round_trip(&case) {
            Ok(v) => v,
            Err(_) => {
                // No point in even trying; we can't even round-trip this float *normally*.
                println!("  - skipping; can't round-trip f32");
                continue;
            }
        };

        let case = n32f::from_float(case).expect("could not convert case to numeric");
        let n_rt: Result<n32f, _> = round_trip(&case);

        if f_rt.is_nan() {
            assert!(n_rt.is_err());
        } else {
            assert_eq!(n_rt.expect("could not decode"),
                n32f::from_float(f_rt).unwrap());
        }
    }

    {
        let case = std::f32::NAN;
        println!("- {:?}", case);
        let encoded = json::to_string(&case).unwrap();
        let decoded: Result<n32f, _> = json::from_str(&encoded);
        assert!(decoded.is_err());
    }
}

fn round_trip<T: Deserialize + Serialize>(v: &T) -> Result<T, Box<Error>> {
    let encoded = json::to_string(v).expect("couldn't encode");
    println!("  - encoded: {:?}", encoded);
    Ok(try!(json::from_str(&encoded)))
}
