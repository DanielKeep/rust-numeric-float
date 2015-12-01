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

custom_derive! {
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Default, PartialEq, PartialOrd, NumericImpl(option))]
    pub struct n32f(f32);
}

custom_derive! {
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Default, PartialEq, PartialOrd, NumericImpl(option))]
    pub struct n64f(f64);
}

custom_derive! {
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Default, PartialEq, PartialOrd, NumericImpl(panic))]
    pub struct n32p(f32);
}

custom_derive! {
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Default, PartialEq, PartialOrd, NumericImpl(panic))]
    pub struct n64p(f64);
}
