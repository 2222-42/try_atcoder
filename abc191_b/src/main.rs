use proconio::input;

fn main() {
    input! {
        n: u32,
        x: u32,
        query: [u32; n]
    }

    let result: Vec<String> = query
        .iter()
        .filter(|&&i| i != x)
        .map(|&i| i.to_string())
        .collect();

    println!("{}", result.join(" "));
}
