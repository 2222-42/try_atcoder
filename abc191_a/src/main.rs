use std::io::Read;
// use proconio::input;

fn main() {
    let mut buf = String::new();

    std::io::stdin().read_to_string(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();

    let v: usize = iter.next().unwrap().parse().unwrap();
    let t: usize = iter.next().unwrap().parse().unwrap();
    let s: usize = iter.next().unwrap().parse().unwrap();
    let d: usize = iter.next().unwrap().parse().unwrap();

    let min = v * t;
    let max = v * s;

    if min <= d && d <= max {
        println!("No")
    } else {
        println!("Yes")
    }
}
