#[cfg(not(feature="serde"))]
macro_rules! num_impl_serde {($($_tts:tt)*) => {}}

#[cfg(feature="serde")]
macro_rules! num_impl_serde {
    (($_kind:tt) pub struct $name:ident($fty:ty);) => {
        impl serde::Deserialize for $name {
            fn deserialize<D: serde::Deserializer>(d: &mut D)
            -> Result<Self, D::Error> {
                use serde::de::Error;
                let v = try!(serde::Deserialize::deserialize(d));
                $name::from_float(v)
                    .ok_or_else(|| D::Error::syntax(::NAN_MSG))
            }
        }

        impl serde::Serialize for $name {
            #[inline]
            fn serialize<S: serde::Serializer>(&self, s: &mut S)
            -> Result<(), S::Error> {
                self.0.serialize(s)
            }
        }
    }
}
