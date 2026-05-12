use core::marker::PhantomData;

mod sealed {
    pub trait Sealed {}
}

pub trait PinMode: sealed::Sealed {}
pub trait IOMode: PinMode + sealed::Sealed {}

pub struct OutputMode;

impl PinMode for OutputMode {}
impl IOMode for OutputMode {}
impl sealed::Sealed for OutputMode {}

pub trait InputState: sealed::Sealed {}

pub struct InputMode<S: InputState> {
    _p: PhantomData<S>,
}

impl<S: InputState> PinMode for InputMode<S> {}
impl<S: InputState> IOMode for InputMode<S> {}
impl<S: InputState> sealed::Sealed for InputMode<S> {}

pub struct Floating;
pub struct PullUp;

impl InputState for Floating {}
impl sealed::Sealed for Floating {}

impl InputState for PullUp {}
impl sealed::Sealed for PullUp {}

pub struct PwmMode;

impl PinMode for PwmMode {}
impl sealed::Sealed for PwmMode {}

pub struct AnalogMode;

impl PinMode for AnalogMode {}
impl sealed::Sealed for AnalogMode {}
