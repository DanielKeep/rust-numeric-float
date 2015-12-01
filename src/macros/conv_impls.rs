#[cfg(not(feature="conv"))]
macro_rules! num_impl_conv {($($_tts:tt)*) => {}}

#[cfg(feature="conv")]
macro_rules! num_impl_conv {
    (($_kind:tt) pub struct $name:ident($fty:ty);) => {
        impl conv::TryFrom<$fty> for $name {
            type Err = conv::errors::Unrepresentable<$fty>;

            #[inline]
            fn try_from(v: $fty) -> Result<$name, Self::Err> {
                $name::from_float(v).ok_or(conv::errors::Unrepresentable(v))
            }
        }

        impl conv::TryInto<$fty> for $name {
            type Err = conv::errors::NoError;

            #[inline]
            fn try_into(self) -> Result<$fty, Self::Err> {
                Ok(self.into_float())
            }
        }

        impl conv::ValueFrom<$fty> for $name {
            type Err = conv::errors::Unrepresentable<$fty>;

            #[inline]
            fn value_from(v: $fty) -> Result<$name, Self::Err> {
                $name::from_float(v).ok_or(conv::errors::Unrepresentable(v))
            }
        }

        impl conv::ValueInto<$fty> for $name {
            type Err = conv::errors::NoError;

            #[inline]
            fn value_into(self) -> Result<$fty, Self::Err> {
                Ok(self.into_float())
            }
        }
    };
}
