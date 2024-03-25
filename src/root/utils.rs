use std::ops::{Add, Rem, Sub};
use crate::root::parser::parse::ParseError;
use crate::root::processor::processor::ProcessorError;
#[macro_export]
macro_rules! time {
    ($($tts:tt)*) => {
        let t = Instant::now();
        $($tts)*;
        let end = t.elapsed();
        println!("Completed [{:?}]", end);
    };
}

pub fn align<T: Copy + Sub<Output = T> + Rem<Output = T> + Add<Output = T>>(
    num: T,
    alignment: T,
) -> T {
    num + (alignment - (num % alignment)) % alignment
}

pub enum AnyError {
    Parse(ParseError),
    Processing(ProcessorError)
}

impl From<ParseError> for AnyError {
    fn from(value: ParseError) -> Self {
        AnyError::Parse(value)
    }
}

impl From<ProcessorError> for AnyError {
    fn from(value: ProcessorError) -> Self {
        AnyError::Processing(value)
    }
}