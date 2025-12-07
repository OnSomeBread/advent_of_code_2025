#[allow(unused)]
use rayon::prelude::*;
fn main() {
    divan::main();
}

const N: usize = 12;

#[divan::bench(args=[""])]
fn largest_num(line: &str) -> i64 {
    let nums: Vec<i64> = line.bytes().rev().map(|x| (x - b'0') as i64).collect();
    let m = nums.len();

    let mut dpi1 = [i64::MIN >> 5; N + 1];
    dpi1[N] = 0;

    for i in (0..m).rev() {
        let mut dp = [0; N + 1];

        for j in (0..N).rev() {
            dp[j] = dpi1[j].max(dpi1[j + 1] * 10 + nums[i]);
        }
        dpi1 = dp;
    }

    dpi1[0]
}

#[divan::bench()]
pub fn lobby2() -> i64 {
    include_str!("../2025/d3.txt")
        .lines()
        .par_bridge()
        .map(largest_num)
        .sum()
}
