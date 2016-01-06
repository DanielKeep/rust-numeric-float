macro_rules! NumericImpl {
    (($kind:tt) pub struct $name:ident($fty:ty);) => {
        /*
        Inherent impl.
        */
        impl $name {
            #[inline]
            pub fn from_float(v: $fty) -> Option<$name> {
                if v.is_nan() {
                    None
                } else {
                    Some($name(v))
                }
            }

            #[inline]
            pub unsafe fn from_float_unchecked(v: $fty) -> $name {
                $name(v)
            }

            #[inline]
            pub fn into_float(self) -> $fty {
                self.0
            }

            #[inline]
            pub fn powf(self, n: $name) -> $name {
                $name((self.0).powf(n.0))
            }

            #[inline]
            pub fn max(self, other: $name) -> $name {
                $name((self.0).max(other.0))
            }

            #[inline]
            pub fn min(self, other: $name) -> $name {
                $name((self.0).min(other.0))
            }

            #[inline]
            pub fn abs_sub(self, other: $name) -> $name {
                $name((self.0).abs_sub(other.0))
            }

            #[inline]
            pub fn hypot(self, other: $name) -> $name {
                $name((self.0).hypot(other.0))
            }

            #[inline]
            pub fn atan2(self, other: $name) -> $name {
                $name((self.0).atan2(other.0))
            }
        }

        /*
        Forward on to other derivation macros.
        */
        expand_multiple! {
            {pub struct $name($fty);}

            WrapOp(Add, add, +, $kind),
            WrapOp(Sub, sub, -, $kind),
            WrapOp(Mul, mul, *, $kind),
            WrapOp(Div, div, /, $kind),
            WrapOp(Rem, rem, %, $kind),
            NewtypeNeg(), NewtypeNeg(&self),
            NewtypeFmt(Debug), NewtypeFmt(Display),
            NewtypeFmt(LowerExp), NewtypeFmt(UpperExp),

            WrapMethod(is_infinite(self) -> bool),
            WrapMethod(is_finite(self) -> bool),
            WrapMethod(is_normal(self) -> bool),
            WrapMethod(classify(self) -> ::std::num::FpCategory),
            WrapMethod(floor(self)),
            WrapMethod(ceil(self)),
            WrapMethod(round(self)),
            WrapMethod(trunc(self)),
            WrapMethod(fract/try_fract(self) -> $kind),
            WrapMethod(abs(self)),
            WrapMethod(signum(self)),
            WrapMethod(is_sign_positive(self) -> bool),
            WrapMethod(is_sign_negative(self) -> bool),
            WrapMethod(recip(self)),
            WrapMethod(powi(self, n: i32)),
            WrapMethod(sqrt/try_sqrt(self) -> $kind),
            WrapMethod(exp(self)),
            WrapMethod(exp2(self)),
            WrapMethod(ln/try_ln(self) -> $kind),
            WrapMethod(log2/try_log2(self) -> $kind),
            WrapMethod(log10/try_log10(self) -> $kind),
            WrapMethod(cbrt(self)),
            WrapMethod(sin/try_sin(self) -> $kind),
            WrapMethod(cos/try_cos(self) -> $kind),
            WrapMethod(tan/try_tan(self) -> $kind),
            WrapMethod(asin/try_asin(self) -> $kind),
            WrapMethod(acos/try_acos(self) -> $kind),
            WrapMethod(atan/try_atan(self) -> $kind),
            WrapMethod(exp_m1(self)),
            WrapMethod(ln_1p/try_ln_1p(self) -> $kind),
            WrapMethod(sinh/try_sinh(self) -> $kind),
            WrapMethod(cosh/try_cosh(self) -> $kind),
            WrapMethod(tanh/try_tanh(self) -> $kind),
            WrapMethod(asinh(self)),
            WrapMethod(acosh/try_acosh(self) -> $kind),
            WrapMethod(atanh/try_atanh(self) -> $kind),
        }

        nan_branch! {
            $kind;
            option {
                impl $name {
                    #[inline]
                    pub fn mul_add(self, a: $name, b: $name) -> $fty {
                        (self.0).mul_add(a.0, b.0)
                    }

                    #[inline]
                    pub fn log(self, base: $name) -> $fty {
                        (self.0).log(base.0)
                    }

                    #[inline]
                    pub fn sin_cos(self) -> ($fty, $fty) {
                        (self.0).sin_cos()
                    }
                }
            }
            panic {
                impl $name {
                    #[inline]
                    pub fn mul_add(self, a: $name, b: $name) -> $name {
                        self.try_mul_add(a, b).expect(::NAN_MSG)
                    }

                    #[inline]
                    pub fn try_mul_add(self, a: $name, b: $name) -> Option<$name> {
                        $name::from_float((self.0).mul_add(a.0, b.0))
                    }

                    #[inline]
                    pub fn log(self, base: $name) -> $name {
                        self.try_log(base).expect(::NAN_MSG)
                    }

                    #[inline]
                    pub fn try_log(self, base: $name) -> Option<$name> {
                        $name::from_float((self.0).log(base.0))
                    }

                    #[inline]
                    pub fn sin_cos(self) -> ($name, $name) {
                        self.try_sin_cos().expect(::NAN_MSG)
                    }

                    #[inline]
                    pub fn try_sin_cos(self) -> Option<($name, $name)> {
                        let (s, c) = (self.0).sin_cos();
                        let s = $name::from_float(s);
                        let c = $name::from_float(c);
                        s.and_then(|s| c.map(|c| (s, c)))
                    }
                }
            }
        }

        /*
        Local crate impls.
        */
        impl ::Numeric for $name {
            type Float = $fty;
        }

        /*
        std impls.
        */
        #[cfg(feature = "deref")]
        impl ::std::ops::Deref for $name {
            type Target = $fty;

            fn deref(&self) -> &$fty {
                &self.0
            }
        }

        impl Eq for $name {}

        impl Into<$fty> for $name {
            #[inline]
            fn into(self) -> $fty {
                self.0
            }
        }

        impl Ord for $name {
            #[inline]
            fn cmp(&self, other: &$name) -> ::std::cmp::Ordering {
                match self.0.partial_cmp(&other.0) {
                    Some(ord) => ord,
                    None => unsafe { unreachable::unreachable() }
                }
            }
        }

        impl std::str::FromStr for $name {
            type Err = ParseNumericError;

            #[inline]
            fn from_str(src: &str) -> Result<$name, Self::Err> {
                match <$fty as std::str::FromStr>::from_str(src) {
                    Ok(v) => $name::from_float(v).ok_or(ParseNumericError::NotANumber),
                    Err(err) => Err(ParseNumericError::FloatError(err))
                }
            }
        }

        impl ::std::hash::Hash for $name {
            fn hash<H>(&self, state: &mut H) where H: ::std::hash::Hasher {
                ::util::HashFloat::hash(&self.0, state)
            }
        }

        num_impl_conv!{ ($kind) pub struct $name($fty); }
        num_impl_num!{ ($kind) pub struct $name($fty); }
    };
}
