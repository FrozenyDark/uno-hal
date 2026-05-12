#[macro_export]
macro_rules! init_register {
    ($name:ident: RegRW<$tmp:ty> = $func:ident($addr:literal) {$($bit_name:ident),+$(,)?}) => {
        impl $name {
            pub const REG: RegRW<$tmp> = RegRW::$func::<$addr>();

            pub(crate) const fn new() -> Self {
                Self {
                    $($bit_name: Bit::new(Self::REG)),+,
                    _p: PhantomData,
                }
            }

            #[inline]
            pub const fn reg(&self) -> RegRO<$tmp> {
                Self::REG.as_ro()
            }

            #[inline]
            pub const fn reg_mut(&mut self) -> RegRW<$tmp> {
                Self::REG
            }
        }
    };

    ($name:ident: RegRW<$tmp:ty> = $func:ident($addr:literal)) => {
        impl $name {
            pub const REG: RegRW<$tmp> = RegRW::$func::<$addr>();

            pub(crate) const fn new() -> Self {
                Self(PhantomData)
            }

            #[inline]
            pub const fn reg(&self) -> RegRO<$tmp> {
                Self::REG.as_ro()
            }

            #[inline]
            pub const fn reg_mut(&mut self) -> RegRW<$tmp> {
                Self::REG
            }
        }
    };

    ($name:ident: RegRO<$tmp:ty> = $func:ident($addr:literal) {$($bit_name:ident),+$(,)?}) => {
        impl $name {
            pub const REG: RegRO<$tmp> = RegRO::$func::<$addr>();

            pub(crate) const fn new() -> Self {
                Self {
                    $($bit_name: Bit::new(Self::REG)),+,
                    _p: PhantomData,
                }
            }

            #[inline]
            pub const fn reg(&self) -> RegRO<$tmp> {
                Self::REG
            }
        }
    };

    ($name:ident: RegRO<$tmp:ty> = $func:ident($addr:literal)) => {
        impl $name {
            pub const REG: RegRO<$tmp> = RegRO::$func::<$addr>();

            pub(crate) const fn new() -> Self {
                Self(PhantomData)
            }

            #[inline]
            pub const fn reg(&self) -> RegRO<$tmp> {
                Self::REG
            }
        }
    };
}
