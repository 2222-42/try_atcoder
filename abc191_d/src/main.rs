use proconio::input;

fn main() {
    input! {
        x: f64,
        y: f64,
        r: f64,
    }

    let mut count = 0;

    let (x, y, r) = (
        (x * 10000.0) as i64,
        (y * 10000.0) as i64,
        (r * 10000.0) as i64,
    );

    let max_x = x + r;
    let min_x = x - r;
    let range_of_x: Vec<i64> = (min_x..=max_x)
        .map(i64::from)
        .filter(|&x| x % 10000 == 0)
        .collect();

    for target_x in range_of_x {
        let div_x = target_x - x;
        let div_y = ((r * r - div_x * div_x) as f64).sqrt();
        let min_y = (y as f64) - div_y;
        let max_y = (y as f64) + div_y;
        let range_of_y: Vec<i64> = (min_y.ceil() as i64..=(max_y.ceil() as i64))
            .map(i64::from)
            .collect();
        count += range_of_y.iter().filter(|&&y| y % 10000 == 0).count()
    }

    println!("{}", count);
}
