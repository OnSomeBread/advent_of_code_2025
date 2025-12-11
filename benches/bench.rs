#[allow(unused)]
use rayon::prelude::*;
fn main() {
    divan::main();
}

const INPUT: &str = include_str!("../inputs/d8.txt");

pub fn factory_dfs(btns: &[u16], target: u16) -> u16 {
    let mut st: Vec<(u8, u16)> = (0..btns.len() as u8).map(|x| (x, 0)).collect();

    let mut depth = 1;
    while !st.is_empty() {
        let mut sub_stack = vec![];
        while let Some((idx, mut progress)) = st.pop() {
            progress ^= btns[idx as usize];
            if progress == target {
                return depth;
            }

            for sub_idx in idx + 1..btns.len() as u8 {
                sub_stack.push((sub_idx, progress));
            }
        }
        st = sub_stack;
        depth += 1;
    }

    0
}

#[divan::bench()]
pub fn factory() -> u16 {
    let mut targets = vec![];
    let mut buttons = vec![];
    for line in INPUT.lines() {
        let mut parts = line.split_ascii_whitespace();
        let t: Vec<char> = parts.next().unwrap().chars().collect();
        targets.push(
            t[1..t.len() - 1]
                .iter()
                .enumerate()
                .fold(0, |acc, (i, &x)| acc | (u16::from(x == '#') << i)),
        );

        let mut btns = vec![];
        let remaining: Vec<&str> = parts.collect();
        for section in remaining.iter().take(remaining.len() - 1) {
            let s: Vec<char> = section.chars().collect();
            let s: String = s[1..s.len() - 1].iter().collect();
            btns.push(
                s.split(',')
                    .fold(0, |acc, x| acc | (1 << x.parse::<u16>().unwrap())),
            );
        }
        buttons.push(btns);
    }

    let mut ans = 0;
    for (&target, btns) in targets.iter().zip(buttons.iter()) {
        ans += factory_dfs(btns, target);
    }

    ans
}

#[divan::bench()]
pub fn factory_par() -> u16 {
    let mut targets = vec![];
    let mut buttons = vec![];
    for line in INPUT.lines() {
        let mut parts = line.split_ascii_whitespace();
        let t: Vec<char> = parts.next().unwrap().chars().collect();
        targets.push(
            t[1..t.len() - 1]
                .iter()
                .enumerate()
                .fold(0, |acc, (i, &x)| acc | (u16::from(x == '#') << i)),
        );

        let mut btns = vec![];
        let remaining: Vec<&str> = parts.collect();
        for section in remaining.iter().take(remaining.len() - 1) {
            let s: Vec<char> = section.chars().collect();
            let s: String = s[1..s.len() - 1].iter().collect();
            btns.push(
                s.split(',')
                    .fold(0, |acc, x| acc | (1 << x.parse::<u16>().unwrap())),
            );
        }
        buttons.push(btns);
    }

    targets
        .par_iter()
        .zip(buttons.par_iter())
        .map(|(&target, btns)| factory_dfs(btns, target))
        .sum()
}
