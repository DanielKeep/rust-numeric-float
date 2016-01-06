#[macro_use] extern crate custom_derive;
#[macro_use] extern crate newtype_derive;

extern crate unreachable;

#[cfg(feature="conv")] extern crate conv;
#[cfg(feature="num")] extern crate num;

pub use err::ParseNumericError;

#[macro_use] mod macros;
mod err;
mod util;

const NAN_MSG: &'static str = "operation resulted in NaN";

pub trait Numeric {
    type Float;
}

custom_derive! {
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Default, PartialEq, PartialOrd,
        NumericImpl(option), RewrapFrom(n32p))]
    pub struct n32f(f32);
}

custom_derive! {
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Default, PartialEq, PartialOrd,
        NumericImpl(option), RewrapFrom(n64p))]
    pub struct n64f(f64);
}

custom_derive! {
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Default, PartialEq, PartialOrd,
        NumericImpl(panic), RewrapFrom(n32f))]
    pub struct n32p(f32);
}

custom_derive! {
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Default, PartialEq, PartialOrd,
        NumericImpl(panic), RewrapFrom(n64f))]
    pub struct n64p(f64);
}
