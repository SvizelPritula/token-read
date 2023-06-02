pub trait LineCount {
    fn decrement(&mut self);
    fn empty(&self) -> bool;
}

macro_rules! impl_uint {
    ($t: ty) => {
        impl LineCount for $t {
            fn decrement(&mut self) {
                *self = self.saturating_sub(1)
            }

            fn empty(&self) -> bool {
                *self == 0
            }
        }
    };
}

impl_uint!(u8);
impl_uint!(u16);
impl_uint!(u32);
impl_uint!(u64);
impl_uint!(u128);
impl_uint!(usize);
