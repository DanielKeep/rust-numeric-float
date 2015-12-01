extern crate ieee754;

pub mod clone_vec_iter;

use self::ieee754::Ieee754;
use self::clone_vec_iter::CloneVecIter;

macro_rules! as_expr {
    ($e:expr) => { $e };
}

macro_rules! either_side_of {
    ($e:expr; +-$n:expr) => {
        {
            let be = (1..$n).scan($e, |v, _| { *v = v.prev(); Some(*v) });
            let ab = (0..$n).scan($e, |v, _| left(Some(*v), *v = v.next()));
            be.chain(ab)
        }
    };
}

pub trait CornerCase: Sized {
    type Iter: Iterator<Item=Self>;
    fn corner_cases() -> Self::Iter;
}

macro_rules! corner_case_impl {
    ($fty:ident) => {
        impl CornerCase for $fty {
            type Iter = CloneVecIter<$fty>;

            fn corner_cases() -> Self::Iter {
                use std::$fty::*;

                let mut cases = vec![];

                cases.push(INFINITY);
                cases.push(NEG_INFINITY);

                cases.extend((0..4).scan(MIN, |v, _| left(Some(*v), *v = v.next())));

                cases.extend(either_side_of![        -10.0; +-4]);
                cases.extend(either_side_of![         -1.0; +-4]);
                cases.extend(either_side_of![-MIN_POSITIVE; +-4]);
                cases.extend(either_side_of![          0.0; +-4]);
                cases.extend(either_side_of![ MIN_POSITIVE; +-4]);
                cases.extend(either_side_of![          1.0; +-4]);
                cases.extend(either_side_of![         10.0; +-4]);

                cases.extend((0..4).scan(MAX, |v, _| left(Some(*v), *v = v.prev())));

                CloneVecIter::new(cases)
            }
        }
    };
}

corner_case_impl!(f32);
corner_case_impl!(f64);

pub fn corner_cases<T: CornerCase>() -> T::Iter {
    T::corner_cases()
}

fn left<A, B>(a: A, b: B) -> A {
    drop(b);
    a
}
