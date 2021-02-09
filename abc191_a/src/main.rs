use std::io::Read;

fn main() {
    let mut buf = String::new();

    std::io::stdin().read_to_string(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();

    let v: usize = iter.next().unwrap().parse().unwrap();
    let t: usize = iter.next().unwrap().parse().unwrap();
    let s: usize = iter.next().unwrap().parse().unwrap();
    let d: usize = iter.next().unwrap().parse().unwrap();

    if d < v*t || d > v*s {
        println!("yes")
    } else {
        println!("no")
    }
}
