#[macro_export]
macro_rules! init_bits {
    ($(#[$meta:meta])* $name:ident {$($(#[$meta_item:meta])* $bit_name:ident = $bit:literal),+$(,)?}) => {
        $(#[$meta])*
        #[repr(u8)]
        #[derive(Clone, Copy)]
        pub enum $name {
            $($(#[$meta_item])* $bit_name = $bit),+
        }

        impl $name {
            pub const fn bit(self) -> u8 {
                self as u8
            }

            pub const fn mask(self) -> u8 {
                1 << self.bit()
            }
        }

        impl From<$name> for u8 {
            fn from(val: $name) -> Self {
                val.mask()
            }
        }

        impl core::ops::BitOr for $name {
            type Output = u8;

            fn bitor(self, rhs: Self) -> Self::Output {
                self.mask() | rhs.mask()
            }
        }
    };
}

#[macro_export]
macro_rules! bits_ro {
    ($bits:ident) => {
        #[inline]
        pub fn is_set(&self, mask: u8) -> bool {
            self.reg().is_set(mask)
        }

        #[inline]
        pub fn is_clear(&self, mask: u8) -> bool {
            self.reg().is_clear(mask)
        }

        #[inline]
        pub fn is_set_bit(&self, bit: $bits) -> bool {
            self.reg().is_set(bit.into())
        }

        #[inline]
        pub fn is_clear_bit(&self, bit: $bits) -> bool {
            self.reg().is_clear(bit.into())
        }
    };
}

#[macro_export]
macro_rules! bits_rw {
    ($bits:ident) => {
        #[inline]
        pub unsafe fn set(&mut self, mask: u8) {
            self.reg_mut().set(mask);
        }

        #[inline]
        pub unsafe fn clear(&mut self, mask: u8) {
            self.reg_mut().clear(mask);
        }

        #[inline]
        pub unsafe fn set_bit(&mut self, bit: $bits) {
            self.reg_mut().set(bit.into());
        }

        #[inline]
        pub unsafe fn clear_bit(&mut self, bit: $bits) {
            self.reg_mut().clear(bit.into());
        }
    };
}

#[macro_export]
macro_rules! init_register {
    ($(#[$meta:meta])* $name:ident: RegRW<$tmp:ty> = $func:ident($addr:literal)$(;)?) => {
        $(#[$meta])*
        pub struct $name(core::marker::PhantomData<*const ()>);

        impl $name {
            pub const REG: RegRW<$tmp> = RegRW::$func::<$addr>();

            pub(crate) const fn new() -> Self {
                Self(core::marker::PhantomData)
            }

            pub const fn reg(&self) -> RegRO<$tmp> {
                Self::REG.as_ro()
            }

            pub const fn reg_mut(&mut self) -> RegRW<$tmp> {
                Self::REG
            }
        }
    };

    ($(#[$meta:meta])* $name:ident: RegRW<$tmp:ty> = $func:ident($addr:literal) + $bits:ident$(;)?) => {
        $(#[$meta])*
        #[doc = "___"]
        #[doc = "Uses `"]
        #[doc = stringify!($bits)]
        #[doc = "` as bits."]
        pub struct $name(core::marker::PhantomData<*const ()>);

        impl $name {
            pub const REG: RegRW<$tmp> = RegRW::$func::<$addr>();

            pub(crate) const fn new() -> Self {
                Self(core::marker::PhantomData)
            }

            pub const fn reg(&self) -> RegRO<$tmp> {
                Self::REG.as_ro()
            }

            $crate::bits_ro!($bits);

            #[inline]
            pub const fn reg_mut(&mut self) -> RegRW<$tmp> {
                Self::REG
            }

            $crate::bits_rw!($bits);
        }
    };

    ($(#[$meta:meta])* $name:ident: RegRO<$tmp:ty> = $func:ident($addr:literal)$(;)?) => {
        $(#[$meta])*
        pub struct $name(core::marker::PhantomData<*const ()>);

        impl $name {
            pub const REG: RegRO<$tmp> = RegRO::$func::<$addr>();

            pub(crate) const fn new() -> Self {
                Self(core::marker::PhantomData)
            }

            #[inline]
            pub const fn reg(&self) -> RegRO<$tmp> {
                Self::REG
            }
        }
    };

    ($(#[$meta:meta])* $name:ident: RegRO<$tmp:ty> = $func:ident($addr:literal) + $bits:ident$(;)?) => {
        $(#[$meta])*
        #[doc = "___"]
        #[doc = "Uses `"]
        #[doc = stringify!($bits)]
        #[doc = "` as bits."]
        pub struct $name(core::marker::PhantomData<*const ()>);

        impl $name {
            pub const REG: RegRO<$tmp> = RegRO::$func::<$addr>();

            pub(crate) const fn new() -> Self {
                Self(core::marker::PhantomData)
            }

            #[inline]
            pub const fn reg(&self) -> RegRO<$tmp> {
                Self::REG
            }

            $crate::bits_ro!($bits);
        }
    };
}
