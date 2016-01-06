#[macro_use] mod std_impls;
#[macro_use] mod conv_impls;
#[macro_use] mod num_impls;
#[macro_use] mod rustc_serialize_impls;

macro_rules! as_items {
    ($($is:item)*) => {$($is)*};
}

macro_rules! NewtypeFmt {
    (($fmt:ident) $(pub)* struct $name:ident $($tail:tt)*) => {
        impl ::std::fmt::$fmt for $name {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.0.fmt(fmt)
            }
        }
    };
}

macro_rules! RewrapFrom {
    (
        ($src:ty)
        $(pub)* struct $name:ident($fty:ty);
    ) => {
        impl ::std::convert::From<$src> for $name {
            #[inline]
            fn from(v: $src) -> Self {
                $name(v.into())
            }
        }
    };
}

macro_rules! WrapOp {
    (
        ($opt:ident, $opf:ident, $ops:tt, option)
        $(pub)* struct $name:ident($fty:ty);
    ) => {
        as_items! {
            impl ::std::ops::$opt<$name> for $name {
                type Output = $fty;
                #[inline]
                fn $opf(self, rhs: $name) -> $fty {
                    (&self.0).$opf(&rhs.0)
                }
            }

            impl<'a> ::std::ops::$opt<&'a $name> for &'a $name {
                type Output = $fty;
                #[inline]
                fn $opf(self, rhs: &'a $name) -> $fty {
                    (&self.0).$opf(&rhs.0)
                }
            }

            impl<'a> ::std::ops::$opt<$name> for &'a $name {
                type Output = $fty;
                #[inline]
                fn $opf(self, rhs: $name) -> $fty {
                    (&self.0).$opf(&rhs.0)
                }
            }

            impl<'a> ::std::ops::$opt<&'a $name> for $name {
                type Output = $fty;
                #[inline]
                fn $opf(self, rhs: &'a $name) -> $fty {
                    (&self.0).$opf(&rhs.0)
                }
            }
        }
    };

    (
        ($opt:ident, $opf:ident, $ops:tt, panic)
        $(pub)* struct $name:ident($fty:ty);
    ) => {
        as_items! {
            impl ::std::ops::$opt<$name> for $name {
                type Output = $name;
                #[inline]
                fn $opf(self, rhs: $name) -> $name {
                    $name::from_float((&self.0).$opf(&rhs.0)).expect(NAN_MSG)
                }
            }

            impl<'a> ::std::ops::$opt<&'a $name> for &'a $name {
                type Output = $name;
                #[inline]
                fn $opf(self, rhs: &'a $name) -> $name {
                    $name::from_float((&self.0).$opf(&rhs.0)).expect(NAN_MSG)
                }
            }

            impl<'a> ::std::ops::$opt<$name> for &'a $name {
                type Output = $name;
                #[inline]
                fn $opf(self, rhs: $name) -> $name {
                    $name::from_float((&self.0).$opf(&rhs.0)).expect(NAN_MSG)
                }
            }

            impl<'a> ::std::ops::$opt<&'a $name> for $name {
                type Output = $name;
                #[inline]
                fn $opf(self, rhs: &'a $name) -> $name {
                    $name::from_float((&self.0).$opf(&rhs.0)).expect(NAN_MSG)
                }
            }
        }
    };
}

macro_rules! WrapMethod {
    (
        ($method:ident(self $(, $an:ident: $aty:ty)*))
        pub struct $name:ident $($tail:tt)*
    ) => {
        impl $name {
            #[inline]
            pub fn $method(self $(, $an: $aty)*) -> $name {
                $name((self.0).$method($($an),*))
            }
        }
    };

    (
        ($method:ident/$try_method:ident(self) -> option)
        pub struct $name:ident($fty:ty);
    ) => {
        impl $name {
            #[inline]
            pub fn $method(self) -> $fty {
                (self.0).$method()
            }
        }
    };

    (
        ($method:ident/$try_method:ident(self) -> panic)
        pub struct $name:ident($fty:ty);
    ) => {
        impl $name {
            #[inline]
            pub fn $method(self) -> $name {
                self.$try_method().expect(NAN_MSG)
            }

            #[inline]
            pub fn $try_method(self) -> Option<$name> {
                $name::from_float((self.0).$method())
            }
        }
    };

    (
        ($method:ident(self) -> $resty:ty)
        pub struct $name:ident $($tail:tt)*
    ) => {
        impl $name {
            #[inline]
            pub fn $method(self) -> $resty {
                (self.0).$method()
            }
        }
    };
}

macro_rules! expand_multiple {
    (
        {$($suffix:tt)*}
        $name:ident $arg:tt,
        $($tail:tt)*
    ) => {
        as_items! {
            $name!{ $arg $($suffix)* }
        }
        expand_multiple! {
            {$($suffix)*}
            $($tail)*
        }
    };

    (
        {$($_suffix:tt)*}
        $(,)*
    ) => {};
}

macro_rules! nan_branch {
    (option; option {$($tts:tt)*} panic $_panic:tt) => {
        as_items! { $($tts)* }
    };
    (panic; option $_option:tt panic {$($tts:tt)*}) => {
        as_items! { $($tts)* }
    };
}
