/*!
This crate defines variants of the `f32` and `f64` types which *exclude* Not-a-Number.  These types are collectively called "numeric".  As a consequence of never being `NaN`, these types implement `Eq`, `Ord`, and `Hash`.

They also attempt to define all relevant operations one would expect to be able to perform on floating-point numbers.  The *problem* is what to do about operations that might result in `NaN`.

The `n*f` types handle operations that result in `NaN` by instead returning the underlying `f*` result directly.  For example, adding two `n32f`s results in an `f32`.

The `n*p` types handle operations that result in `NaN` by *panicking* when `NaN` is produced.  Some operations which panic also have `try_*` alternatives that return `Option<Self>` instead.

# Supported Traits

Where possible, the types in this crate implement all traits which are implemented on the corresponding `f*` type.  In addition to those in the standard library, the following crates are also optionally supported:

* [`conv`](https://crates.io/crates/conv/)
* [`num`](https://crates.io/crates/num/)
* [`rustc-serialize`](https://crates.io/crates/rustc-serialize/)

The following are explicitly *not* supported:

* `DecodableFloat`, and `RawFloat` are internal and not stable to implement.
* `One`, `Zero`, and `*Assign` are unstable.
* [`ieee754`](https://crates.io/crates/ieee754/): the `Ieee754` trait *cannot* be implemented for new types.
* `num`: `Float`, `Num`, `One`, `Signed`, and `Zero` *cannot* be implemented for `n*f`.
*/
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate newtype_derive;

extern crate unreachable;

#[cfg(feature="conv")] extern crate conv;
#[cfg(feature="num")] extern crate num;
#[cfg(feature="rustc-serialize")] extern crate rustc_serialize;

pub use err::ParseNumericError;

#[macro_use] mod macros;
mod err;
mod util;

const NAN_MSG: &'static str = "operation resulted in NaN";

/**
This trait is implemented for all numeric float types in this crate.
*/
pub trait Numeric {
    /// The corresponding "full float" type.
    type Float;
}

custom_derive! {
    /**
    A 32-bit floating point type which excludes Not-a-Number.

    Operations which would normally produce Not-a-Number instead result in `f32`.
    */
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Default, PartialEq, PartialOrd,
        NumericImpl(option), RewrapFrom(n32p))]
    pub struct n32f(f32);
}

custom_derive! {
    /**
    A 64-bit floating point type which excludes Not-a-Number.

    Operations which would normally produce Not-a-Number instead result in `f64`.
    */
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Default, PartialEq, PartialOrd,
        NumericImpl(option), RewrapFrom(n64p))]
    pub struct n64f(f64);
}

custom_derive! {
    /**
    A 32-bit floating point type which excludes Not-a-Number.

    Operations which would normally produce Not-a-Number instead panic.
    */
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Default, PartialEq, PartialOrd,
        NumericImpl(panic), RewrapFrom(n32f))]
    pub struct n32p(f32);
}

custom_derive! {

    /**
    A 64-bit floating point type which excludes Not-a-Number.

    Operations which would normally produce Not-a-Number instead panic.
    */
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Default, PartialEq, PartialOrd,
        NumericImpl(panic), RewrapFrom(n64f))]
    pub struct n64p(f64);
}
