use std::time::Duration;
use std::thread;

pub fn expensive_computation<T>(x: T) -> T {
    thread::sleep(Duration::from_micros(100));
    x
}