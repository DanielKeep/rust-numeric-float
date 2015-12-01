
# `numeric-float`

This crate defines variants of the `f32` and `f64` types which *exclude* Not-a-Number.  These types are collectively called "numeric".  As a consequence of never being `NaN`, these types implement `Eq`, `Ord`, and `Hash`.

They also attempt to define all relevant operations one would expect to be able to perform on floating-point numbers.  The *problem* is what to do about operations that might result in `NaN`.

The `n*f` types handle operations that result in `NaN` by instead returning the underlying `f*` result directly.  For example, adding two `n32f`s results in an `f32`.

The `n*p` types handle operations that result in `NaN` by *panicking* when `NaN` is produced.  Some operations which panic also have `try_*` alternatives that return `Option<Self>` instead.
