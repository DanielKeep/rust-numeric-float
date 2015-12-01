pub trait HashFloat {
    fn hash<H>(&self, state: &mut H) where H: ::std::hash::Hasher;
}

impl HashFloat for f32 {
    fn hash<H>(&self, state: &mut H) where H: ::std::hash::Hasher {
        let v = unsafe { *(self as *const f32 as *const u32) };
        state.write_u32(v)
    }
}

impl HashFloat for f64 {
    fn hash<H>(&self, state: &mut H) where H: ::std::hash::Hasher {
        let v = unsafe { *(self as *const f64 as *const u64) };
        state.write_u64(v)
    }
}
