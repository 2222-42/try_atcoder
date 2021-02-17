use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: [(usize, usize, u32)],
    }

    // n行の出力をする

    // qのそれぞれの第一変数と第二引数は1からnのいずれか

    // i (>=1, <= n)について、iからiまでの経路を探す。
    // 存在しなかったら-1を、
    // 存在したら、そのうち、最小の合計値を出力する。

    // 方針
    // (1)iからi自身への道がないか探す。 -> 自明に最小の値になるので、それを採用する。

    // (2)ダイクストラ法を使う
    // i から iとは異なるjへの道を探し、
    // 次に、jからiへの道を探す。
    // (2-1)道を全て反転させ、iからjへの道を探す。

    println!("{}", 1);
}
