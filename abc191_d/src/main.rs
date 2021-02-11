use proconio::input;

fn main() {
    input! {
        x: f64,
        y: f64,
        r: f64,
    }

    let mut count = 0;

    let max_x = x + r;
    let min_x = x - r;
    let range_of_x = (min_x.ceil() as i64)..=(max_x.ceil() as i64);

    for target_x in range_of_x {
        let div_x = target_x as f64 - x;
        let div_y = (r * r - div_x * div_x).sqrt();
        let min_y = y - div_y;
        let max_y = y + div_y;
        let range_of_y = (min_y.ceil() as i64..=(max_y.ceil() as i64));
        count += range_of_y.count();
    }

    println!("{}", count);
}
