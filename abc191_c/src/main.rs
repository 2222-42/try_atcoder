use proconio::input;
use proconio::marker::Chars;

/// cf: https://atcoder.jp/contests/abc191/editorial/612
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }

    let mut angle = 0;
    for i in 0..h - 1 {
        for j in 0..w - 1 {
            let round_square = vec![s[i][j], s[i + 1][j], s[i][j + 1], s[i + 1][j + 1]];
            let black_count = round_square.iter().filter(|&&x| x == '#').count();
            if black_count == 1 || black_count == 3 {
                angle += 1
            }
        }
    }

    println!("{}", angle);
}
