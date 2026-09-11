pub trait AtomicRepr {
    fn to_bits(self) -> u32;
    fn from_bits(bits: u32) -> Self;
}

macro_rules! atomic_repr_impl {
    ($type:ty) => {
        impl AtomicRepr for $type {
            fn to_bits(self) -> u32 {
                self as u32
            }

            fn from_bits(bits: u32) -> Self {
                bits as $type
            }
        }
    };
}

atomic_repr_impl!(u8);
atomic_repr_impl!(u16);
atomic_repr_impl!(u32);
atomic_repr_impl!(i8);
atomic_repr_impl!(i16);
atomic_repr_impl!(i32);

impl AtomicRepr for f32 {
    fn to_bits(self) -> u32 {
        self.to_bits()
    }

    fn from_bits(bits: u32) -> Self {
        f32::from_bits(bits)
    }
}
