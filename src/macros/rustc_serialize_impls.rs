#[cfg(not(feature="rustc-serialize"))]
macro_rules! num_impl_rustc_serialize {($($_tts:tt)*) => {}}

#[cfg(feature="rustc-serialize")]
macro_rules! num_impl_rustc_serialize {
    (($_kind:tt) pub struct $name:ident($fty:ty);) => {
        impl rustc_serialize::Decodable for $name {
            fn decode<D: rustc_serialize::Decoder>(d: &mut D)
            -> Result<Self, D::Error> {
                let v = try!(rustc_serialize::Decodable::decode(d));
                $name::from_float(v)
                    .ok_or_else(|| d.error(::NAN_MSG))
            }
        }

        impl rustc_serialize::Encodable for $name {
            #[inline]
            fn encode<S: rustc_serialize::Encoder>(&self, s: &mut S)
            -> Result<(), S::Error> {
                self.0.encode(s)
            }
        }
    }
}
