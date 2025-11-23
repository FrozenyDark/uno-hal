#[macro_export]
macro_rules! _VECTOR {
    ($n:literal) => {
        concat!["__vector_", $n]
    };
}

#[macro_export]
macro_rules! TIMER0_OVF_vect {
    () => {
        $crate::_VECTOR!(16)
    };
}

#[macro_export]
macro_rules! USART_UDRE_vect {
    () => {
        $crate::_VECTOR!(19)
    };
}
