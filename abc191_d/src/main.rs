use proconio::input;

fn main() {
    input! {
        x: f64,
        y: f64,
        r: f64,
    }

    let mut count = 0;

    let coeff: i64 = 10000;

    let (x, y, r) = (
        (x * coeff as f64).round() as i64,
        (y * coeff as f64).round() as i64,
        (r * coeff as f64).round() as i64,
    );

    let min_x = (x - r + coeff - 1).div_euclid(coeff) * coeff;
    let max_x = (x + r).div_euclid(coeff) * coeff;

    let mut target_x = min_x;
    while target_x <= max_x {
        let div_x = target_x - x;
        let div_y = newton_sqrt(r * r - div_x * div_x);
        let min_y = (y - div_y + coeff - 1).div_euclid(coeff);
        let max_y = (y + div_y).div_euclid(coeff);
        count += (max_y - min_y) + 1;

        target_x += coeff;
    }

    println!("{}", count);
}

fn newton_sqrt(n: i64) -> i64 {
    let mut x0 = n / 2;
    if x0 == 0 {
        return n;
    }
    let mut x1 = (x0 + n / x0) / 2;
    while (x1 < x0) {
        x0 = x1;
        x1 = (x0 + n / x0) / 2;
    }
    x0
}
