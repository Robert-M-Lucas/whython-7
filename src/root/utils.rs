use std::ops::{Add, Rem, Sub};
#[macro_export]
macro_rules! time {
    ($($tts:tt)*) => {
        let t = Instant::now();
        $($tts)*;
        let end = t.elapsed();
        println!("Completed [{:?}]", end);
    };
}

pub fn align<T: Copy + Sub<Output=T> + Rem<Output=T> + Add<Output=T>>(num: T, alignment: T) -> T {
    num + (alignment - (num % alignment)) % alignment
}