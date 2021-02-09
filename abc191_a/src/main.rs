// use std::io::Read;
use proconio::input;

fn main() {
    input! {
        v: u32,
        t: u32,
        s: u32,
        d: u32,
    }

    let min = v * t;
    let max = v * s;

    if min <= d && d <= max {
        println!("No")
    } else {
        println!("Yes")
    }
}
