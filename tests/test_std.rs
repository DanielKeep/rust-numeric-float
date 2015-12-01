#![feature(catch_panic)]

extern crate itertools;
extern crate numeric_float;

#[macro_use] mod util;

use std::thread::catch_panic;
use itertools::Itertools;
use numeric_float::{Numeric, n32f, n64f, n32p, n64p};
use util::corner_cases as ccs;

#[test]
fn test_arith() {
    macro_rules! test_bin_op {
        ($op:tt; $ty:ty; option) => {
            {
                type F = <$ty as Numeric>::Float;
                for (af, bf) in ccs::<F>().cartesian_product(ccs::<F>()) {
                    let cf = as_expr!(af $op bf);
                    let an = <$ty>::from_float(af).unwrap();
                    let bn = <$ty>::from_float(bf).unwrap();
                    let cn = <$ty>::from_float(as_expr!(an $op bn));
                    match cn {
                        Some(cn) => {
                            let cnf = cn.into_float();
                            assert!(!cnf.is_nan());
                            assert_eq!(cnf, cf);
                        },
                        None => {
                            assert!(cf.is_nan());
                        }
                    }
                }
            }
        };

        ($op:tt; $ty:ty; panic) => {
            {
                type F = <$ty as Numeric>::Float;
                for (af, bf) in ccs::<F>().cartesian_product(ccs::<F>()) {
                    let cf = as_expr!(af $op bf);
                    let an = <$ty>::from_float(af).unwrap();
                    let bn = <$ty>::from_float(bf).unwrap();
                    let cn = catch_panic(move || as_expr!(an $op bn)).ok();
                    match cn {
                        Some(cn) => {
                            let cnf = cn.into_float();
                            assert!(!cnf.is_nan());
                            assert_eq!(cnf, cf);
                        },
                        None => {
                            assert!(cf.is_nan());
                        }
                    }
                }
            }
        };
    }

    test_bin_op!(+; n32f; option);
    test_bin_op!(-; n32f; option);
    test_bin_op!(*; n32f; option);
    test_bin_op!(/; n32f; option);
    test_bin_op!(%; n32f; option);

    test_bin_op!(+; n64f; option);
    test_bin_op!(-; n64f; option);
    test_bin_op!(*; n64f; option);
    test_bin_op!(/; n64f; option);
    test_bin_op!(%; n64f; option);

    test_bin_op!(+; n32p; panic);
    test_bin_op!(-; n32p; panic);
    test_bin_op!(*; n32p; panic);
    test_bin_op!(/; n32p; panic);
    test_bin_op!(%; n32p; panic);

    test_bin_op!(+; n64p; panic);
    test_bin_op!(-; n64p; panic);
    test_bin_op!(*; n64p; panic);
    test_bin_op!(/; n64p; panic);
    test_bin_op!(%; n64p; panic);
}
