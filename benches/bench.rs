#[allow(unused)]
use rayon::prelude::*;
fn main() {
    divan::main();
}

const INPUT: &str = include_str!("../inputs/d8.txt");

#[divan::bench()]
fn create_coords() -> Vec<(i32, i32, i32)> {
    INPUT
        .lines()
        .map(|line| {
            let p = line
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            (p[0], p[1], p[2])
        })
        .collect()
}

#[divan::bench()]
fn create_coords_par() -> Vec<(i32, i32, i32)> {
    INPUT
        .lines()
        .par_bridge()
        .map(|line| {
            let p = line
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            (p[0], p[1], p[2])
        })
        .collect()
}
