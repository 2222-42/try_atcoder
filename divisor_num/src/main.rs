fn main() {
    let result = factorization(48024900);
    println!("{:?}", result);

    println!("{}", count_divisor_num(result));
}

fn count_divisor_num(divs: Vec<(i32, i32)>) -> i32 {
    let mut result = 1;
    for &op in divs.iter() {
        result = result * (op.1 + 1)
    }

    result
}

fn factorization(n: i32) -> Vec<(i32, i32)> {
    fn factor_sub(n: i32, m: i32) -> (i32, i32) {
        let mut c = 0;
        let mut x = n;
        while x % m == 0 {
            c += 1;
            x /= m;
        }
        (c, x)
    }

    let mut result = vec![];

    let (c, n) = factor_sub(n, 2);
    if c > 0 {
        result.push((2, c));
    }
    let mut x = 3;
    let mut m = n;
    while x * x <= m {
        let (c, n) = factor_sub(m, x);
        if c > 0 {
            result.push((x, c));
        }
        m = n;
        x += 2;
    }
    if m > 1 {
        result.push((m, 1));
    }
    result
}
