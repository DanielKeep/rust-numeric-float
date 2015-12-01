#[cfg(not(feature="num"))]
macro_rules! num_impl_num {($($_tts:tt)*) => {}}

#[cfg(feature="num")]
macro_rules! num_impl_num {
    (($kind:tt) pub struct $name:ident($fty:ty);) => {
        impl num::traits::Bounded for $name {
            #[inline]
            fn min_value() -> $name {
                $name(num::traits::Bounded::min_value())
            }

            #[inline]
            fn max_value() -> $name {
                $name(num::traits::Bounded::max_value())
            }
        }

        // `Float`: requires `Num`.
        nan_branch! {
            $kind;
            option {}
            panic {
                impl num::traits::Float for $name {
                    #[inline]
                    fn nan() -> $name {
                        panic!(concat!(stringify!($name), " does not have a NaN representation"))
                    }

                    #[inline]
                    fn infinity() -> $name {
                        $name::from_float(<$fty>::infinity()).expect(::NAN_MSG)
                    }

                    #[inline]
                    fn neg_infinity() -> $name {
                        $name::from_float(<$fty>::neg_infinity()).expect(::NAN_MSG)
                    }

                    #[inline]
                    fn neg_zero() -> $name {
                        $name::from_float(<$fty>::neg_zero()).expect(::NAN_MSG)
                    }

                    #[inline]
                    fn min_value() -> $name {
                        $name::from_float(<$fty>::min_value()).expect(::NAN_MSG)
                    }

                    #[inline]
                    fn min_positive_value() -> $name {
                        $name::from_float(<$fty>::min_positive_value()).expect(::NAN_MSG)
                    }

                    #[inline]
                    fn max_value() -> $name {
                        $name::from_float(<$fty>::max_value()).expect(::NAN_MSG)
                    }

                    #[inline]
                    fn is_nan(self) -> bool {
                        false
                    }

                    #[inline]
                    fn is_infinite(self) -> bool {
                        (self.0).is_infinite()
                    }

                    #[inline]
                    fn is_finite(self) -> bool {
                        (self.0).is_finite()
                    }

                    #[inline]
                    fn is_normal(self) -> bool {
                        (self.0).is_normal()
                    }

                    #[inline]
                    fn classify(self) -> ::std::num::FpCategory {
                        (self.0).classify()
                    }

                    #[inline]
                    fn floor(self) -> $name {
                        $name::floor(self)
                    }

                    #[inline]
                    fn ceil(self) -> $name {
                        $name::ceil(self)
                    }

                    #[inline]
                    fn round(self) -> $name {
                        $name::round(self)
                    }

                    #[inline]
                    fn trunc(self) -> $name {
                        $name::trunc(self)
                    }

                    #[inline]
                    fn fract(self) -> $name {
                        $name::fract(self)
                    }

                    #[inline]
                    fn abs(self) -> $name {
                        $name::abs(self)
                    }

                    #[inline]
                    fn signum(self) -> $name {
                        $name::signum(self)
                    }

                    #[inline]
                    fn is_sign_positive(self) -> bool {
                        $name::is_sign_positive(self)
                    }

                    #[inline]
                    fn is_sign_negative(self) -> bool {
                        $name::is_sign_negative(self)
                    }

                    #[inline]
                    fn mul_add(self, a: $name, b: $name) -> $name {
                        $name::mul_add(self, a, b)
                    }

                    #[inline]
                    fn recip(self) -> $name {
                        $name::recip(self)
                    }

                    #[inline]
                    fn powi(self, n: i32) -> $name {
                        $name::powi(self, n)
                    }

                    #[inline]
                    fn powf(self, n: $name) -> $name {
                        $name::powf(self, n)
                    }

                    #[inline]
                    fn sqrt(self) -> $name {
                        $name::sqrt(self)
                    }

                    #[inline]
                    fn exp(self) -> $name {
                        $name::exp(self)
                    }

                    #[inline]
                    fn exp2(self) -> $name {
                        $name::exp2(self)
                    }

                    #[inline]
                    fn ln(self) -> $name {
                        $name::ln(self)
                    }

                    #[inline]
                    fn log(self, base: $name) -> $name {
                        $name::log(self, base)
                    }

                    #[inline]
                    fn log2(self) -> $name {
                        $name::log2(self)
                    }

                    #[inline]
                    fn log10(self) -> $name {
                        $name::log10(self)
                    }

                    #[inline]
                    fn max(self, other: $name) -> $name {
                        $name::max(self, other)
                    }

                    #[inline]
                    fn min(self, other: $name) -> $name {
                        $name::min(self, other)
                    }

                    #[inline]
                    fn abs_sub(self, other: $name) -> $name {
                        $name::abs_sub(self, other)
                    }

                    #[inline]
                    fn cbrt(self) -> $name {
                        $name::cbrt(self)
                    }

                    #[inline]
                    fn hypot(self, other: $name) -> $name {
                        $name::hypot(self, other)
                    }

                    #[inline]
                    fn sin(self) -> $name {
                        $name::sin(self)
                    }

                    #[inline]
                    fn cos(self) -> $name {
                        $name::cos(self)
                    }

                    #[inline]
                    fn tan(self) -> $name {
                        $name::tan(self)
                    }

                    #[inline]
                    fn asin(self) -> $name {
                        $name::asin(self)
                    }

                    #[inline]
                    fn acos(self) -> $name {
                        $name::acos(self)
                    }

                    #[inline]
                    fn atan(self) -> $name {
                        $name::atan(self)
                    }

                    #[inline]
                    fn atan2(self, other: $name) -> $name {
                        $name::atan2(self, other)
                    }

                    #[inline]
                    fn sin_cos(self) -> ($name, $name) {
                        $name::sin_cos(self)
                    }

                    #[inline]
                    fn exp_m1(self) -> $name {
                        $name::exp_m1(self)
                    }

                    #[inline]
                    fn ln_1p(self) -> $name {
                        $name::ln_1p(self)
                    }

                    #[inline]
                    fn sinh(self) -> $name {
                        $name::sinh(self)
                    }

                    #[inline]
                    fn cosh(self) -> $name {
                        $name::cosh(self)
                    }

                    #[inline]
                    fn tanh(self) -> $name {
                        $name::tanh(self)
                    }

                    #[inline]
                    fn asinh(self) -> $name {
                        $name::asinh(self)
                    }

                    #[inline]
                    fn acosh(self) -> $name {
                        $name::acosh(self)
                    }

                    #[inline]
                    fn atanh(self) -> $name {
                        $name::atanh(self)
                    }

                    #[inline]
                    fn integer_decode(self) -> (u64, i16, i8) {
                        num::traits::Float::integer_decode(self.0)
                    }
                }
            }
        }

        impl num::traits::FromPrimitive for $name {
            #[inline]
            fn from_i64(n: i64) -> Option<$name> {
                num::traits::FromPrimitive::from_i64(n).and_then($name::from_float)
            }

            #[inline]
            fn from_u64(n: u64) -> Option<$name> {
                num::traits::FromPrimitive::from_u64(n).and_then($name::from_float)
            }

            #[inline]
            fn from_isize(n: isize) -> Option<$name> {
                num::traits::FromPrimitive::from_isize(n).and_then($name::from_float)
            }

            #[inline]
            fn from_i8(n: i8) -> Option<$name> {
                num::traits::FromPrimitive::from_i8(n).and_then($name::from_float)
            }

            #[inline]
            fn from_i16(n: i16) -> Option<$name> {
                num::traits::FromPrimitive::from_i16(n).and_then($name::from_float)
            }

            #[inline]
            fn from_i32(n: i32) -> Option<$name> {
                num::traits::FromPrimitive::from_i32(n).and_then($name::from_float)
            }

            #[inline]
            fn from_usize(n: usize) -> Option<$name> {
                num::traits::FromPrimitive::from_usize(n).and_then($name::from_float)
            }

            #[inline]
            fn from_u8(n: u8) -> Option<$name> {
                num::traits::FromPrimitive::from_u8(n).and_then($name::from_float)
            }

            #[inline]
            fn from_u16(n: u16) -> Option<$name> {
                num::traits::FromPrimitive::from_u16(n).and_then($name::from_float)
            }

            #[inline]
            fn from_u32(n: u32) -> Option<$name> {
                num::traits::FromPrimitive::from_u32(n).and_then($name::from_float)
            }

            #[inline]
            fn from_f32(n: f32) -> Option<$name> {
                num::traits::FromPrimitive::from_f32(n).and_then($name::from_float)
            }

            #[inline]
            fn from_f64(n: f64) -> Option<$name> {
                num::traits::FromPrimitive::from_f64(n).and_then($name::from_float)
            }
        }

        // `Num`: requires `One`, plus `+`, `-`, `*`, `/`, and `%` to result in `$name`.
        nan_branch! {
            $kind;
            option {}
            panic {
                impl num::traits::Num for $name {
                    type FromStrRadixErr = num::traits::ParseFloatError;

                    #[inline]
                    fn from_str_radix(s: &str, radix: u32) -> Result<$name, Self::FromStrRadixErr> {
                        let v = try!(<$fty as num::traits::Num>::from_str_radix(s, radix));
                        if v.is_nan() {
                            return Err(num::traits::ParseFloatError {
                                kind: num::traits::FloatErrorKind::Invalid
                            });
                        } else {
                            unsafe { Ok($name::from_float_unchecked(v)) }
                        }
                    }
                }
            }
        }

        impl num::traits::NumCast for $name {
            #[inline]
            fn from<T: num::traits::ToPrimitive>(n: T) -> Option<$name> {
                num::traits::NumCast::from(n).and_then($name::from_float)
            }
        }

        // `One`: requires `$name * $name -> $name`.
        nan_branch! {
            $kind;
            option {}
            panic {
                impl num::traits::One for $name {
                    #[inline]
                    fn one() -> $name {
                        $name(1.0)
                    }
                }
            }
        }

        // `Signed`: requires `Num`.
        nan_branch! {
            $kind;
            option {}
            panic {
                // TODO
            }
        }

        impl num::traits::ToPrimitive for $name {
            #[inline]
            fn to_i64(&self) -> Option<i64> {
                self.0.to_i64()
            }

            #[inline]
            fn to_u64(&self) -> Option<u64> {
                self.0.to_u64()
            }

            #[inline]
            fn to_isize(&self) -> Option<isize> {
                self.0.to_isize()
            }

            #[inline]
            fn to_i8(&self) -> Option<i8> {
                self.0.to_i8()
            }

            #[inline]
            fn to_i16(&self) -> Option<i16> {
                self.0.to_i16()
            }

            #[inline]
            fn to_i32(&self) -> Option<i32> {
                self.0.to_i32()
            }

            #[inline]
            fn to_usize(&self) -> Option<usize> {
                self.0.to_usize()
            }

            #[inline]
            fn to_u8(&self) -> Option<u8> {
                self.0.to_u8()
            }

            #[inline]
            fn to_u16(&self) -> Option<u16> {
                self.0.to_u16()
            }

            #[inline]
            fn to_u32(&self) -> Option<u32> {
                self.0.to_u32()
            }

            #[inline]
            fn to_f32(&self) -> Option<f32> {
                self.0.to_f32()
            }

            #[inline]
            fn to_f64(&self) -> Option<f64> {
                self.0.to_f64()
            }
        }

        // `Zero`: requires `$name + $name -> $name`.
        nan_branch! {
            $kind;
            option {}
            panic {
                impl num::traits::Zero for $name {
                    #[inline]
                    fn zero() -> $name {
                        $name(0.0)
                    }

                    #[inline]
                    fn is_zero(&self) -> bool {
                        (self.0).is_zero()
                    }
                }
            }
        }
    };
}
