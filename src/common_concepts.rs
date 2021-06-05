#![allow(unused)]

pub fn fibonacci(x: u32) -> u32
{
    if x == 0 || x == 1 {
        x
    } else {
        fibonacci(x-1) + fibonacci(x-2)
    }
}
