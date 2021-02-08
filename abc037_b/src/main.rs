use std::io::Read;

fn main() {
    let mut buf = String::new();

    std::io::stdin().read_to_string(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();

    let lrt:Vec<(usize, usize, u64)> = (0..q).map(|_| {
        (iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),)
    }).collect();

    let mut a = vec![0; n];

    for (le, ri, t) in lrt {
        for i in le..=ri {
            a[i-1] = t;
        }
    }

    for e in a.iter() {
        println!("{}", e);
    }
}
