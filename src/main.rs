#![feature(const_trait_impl, const_range)]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use ahash::{AHashMap, AHashSet};
use itertools::Itertools;
use rayon::prelude::*;
use smallvec::SmallVec;
use std::collections::BinaryHeap;
use tracing::info;

mod tests;

// DAY 1 PART 1
pub fn secret_entrance(input: &'static str) -> i32 {
    let mut curr: i32 = 50;
    let mut ans = 0;

    for turn in input.lines() {
        let mut letters_iter = turn.chars();
        let l0 = letters_iter.next().unwrap();
        let amt = letters_iter.collect::<String>().parse::<i32>().unwrap();

        curr = (curr + if l0 == 'R' { amt } else { -amt }).rem_euclid(100);
        if curr == 0 {
            ans += 1;
        }
    }
    ans
}

// DAY 1 PART 2
pub fn secret_entrance2(input: &'static str) -> i32 {
    let mut curr: i32 = 50;
    let mut lands_on = [0; 100];
    for turn in input.lines() {
        let mut letters_iter = turn.chars();
        let l0 = letters_iter.next().unwrap();
        let amt = letters_iter.collect::<String>().parse::<i32>().unwrap();

        for i in 1..=amt {
            if l0 == 'R' {
                lands_on[((curr + i) % 100) as usize] += 1;
            } else {
                lands_on[(curr - i).rem_euclid(100) as usize] += 1;
            }
        }

        curr = (curr + if l0 == 'R' { amt } else { -amt }).rem_euclid(100);
    }
    lands_on[0]
}

// DAY 2 PART 1
pub fn gift_shop(input: &'static str) -> i64 {
    input
        .split(',')
        .par_bridge()
        .map(|range_str| {
            let mut range = range_str.split('-').map(|x| x.parse::<i64>().unwrap());

            let mut total = 0;
            for num in range.next().unwrap()..=range.next().unwrap() {
                let str_num: SmallVec<[char; 20]> = num.to_string().chars().collect();
                let num_len = str_num.len();
                if num_len.is_multiple_of(2) {
                    let n = num_len / 2;
                    if (0..n).all(|i| str_num[i] == str_num[i + n]) {
                        total += num;
                    }
                }
            }
            total
        })
        .sum()
}

// DAY 2 PART 2
pub fn gift_shop2(input: &'static str) -> i64 {
    input
        .split(',')
        .par_bridge()
        .map(|range_str| {
            let mut range = range_str.split('-').map(|x| x.parse::<i64>().unwrap());

            let mut total = 0;
            for num in range.next().unwrap()..=range.next().unwrap() {
                let str_num: SmallVec<[char; 20]> = num.to_string().chars().collect();
                let num_len = str_num.len();

                let mut invalid = false;
                for parts in 2..=num_len {
                    if num_len.is_multiple_of(parts) {
                        let part_size = num_len / parts;

                        if (0..part_size).all(|i| {
                            (1..parts)
                                .all(|part_num| str_num[i] == str_num[i + part_size * part_num])
                        }) {
                            invalid = true;
                            break;
                        }
                    }
                }
                if invalid {
                    total += num;
                }
            }
            total
        })
        .sum()
}

// DAY 3 PART 1
pub fn lobby(input: &'static str) -> i32 {
    input
        .lines()
        .par_bridge()
        .map(|line| {
            let mut largest = i32::MIN / 100;
            let mut best = 0;
            for digit_char in line.chars() {
                if let Some(digit) = digit_char.to_digit(10) {
                    best = best.max(largest * 10 + digit as i32);
                    largest = largest.max(digit as i32);
                }
            }
            if largest == i32::MIN / 100 {
                return 0;
            }
            best
        })
        .sum()
}

// DAY 3 PART 2 SLOW SOLUTION
pub fn lobby2_top_down(input: &'static str) -> i64 {
    fn dp(nums: &Vec<i32>, i: usize, count: i32, cache: &mut AHashMap<(usize, i32), i64>) -> i64 {
        if count == 12 {
            return 0;
        }

        if i >= nums.len() {
            return i64::MIN / 1000;
        }

        if let Some(&ans) = cache.get(&(i, count)) {
            return ans;
        }

        let include = dp(nums, i + 1, count + 1, cache) * 10 + nums[i] as i64;
        let dont_include = dp(nums, i + 1, count, cache);
        let ans = include.max(dont_include);

        cache.insert((i, count), ans);
        ans
    }

    input
        .lines()
        .par_bridge()
        .map(|line| {
            let nums: Vec<i32> = line
                .chars()
                .rev()
                .map(|x| x.to_digit(10).unwrap() as i32)
                .collect();

            dp(&nums, 0, 0, &mut AHashMap::new())
        })
        .sum()
}

// DAY 3 PART 2
pub fn lobby2(input: &'static str) -> i64 {
    const N: usize = 12;

    input
        .lines()
        .par_bridge()
        .map(|line| {
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
        })
        .sum()
}

const fn in_bounds(i: i32, j: i32, m: usize, n: usize) -> bool {
    (0..m as i32).contains(&i) && (0..n as i32).contains(&j)
}

const DIRS_8: [(i32, i32); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
];

// DAY 4 PART 1
pub fn printing_department(input: &'static str) -> i32 {
    let map: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.bytes().map(|x| x == b'@').collect())
        .collect();

    let m = map.len();
    let n = map[0].len();

    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            if !map[i][j] {
                continue;
            }
            let mut count = 0;
            for (di, dj) in DIRS_8 {
                let (ni, nj) = (i as i32 + di, j as i32 + dj);

                if in_bounds(ni, nj, m, n) && map[ni as usize][nj as usize] {
                    count += 1;
                }
            }
            if count < 4 {
                ans += 1;
            }
        }
    }

    ans
}

// DAY 4 PART 2
pub fn printing_department2(input: &'static str) -> i32 {
    let mut map: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.bytes().map(|x| x == b'@').collect())
        .collect();

    let m = map.len();
    let n = map[0].len();

    let mut total = 0;
    let mut prev_ans = i32::MAX;
    while prev_ans > 0 {
        let mut ans = 0;
        let sub_map = map.clone();
        for i in 0..m {
            for j in 0..n {
                if !sub_map[i][j] {
                    continue;
                }
                let mut count = 0;
                for (di, dj) in DIRS_8 {
                    let (ni, nj) = (i as i32 + di, j as i32 + dj);

                    if in_bounds(ni, nj, m, n) && sub_map[ni as usize][nj as usize] {
                        count += 1;
                    }
                }
                if count < 4 {
                    map[i][j] = false;
                    ans += 1;
                }
            }
        }

        total += ans;
        prev_ans = ans;
    }

    total
}

const fn cafe_order((start, end): (i64, i64), id: i64) -> std::cmp::Ordering {
    if (start..=end).contains(&id) {
        std::cmp::Ordering::Equal
    } else if start > id {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Less
    }
}

// DAY 5 PART 1
pub fn cafeteria(input: &'static str) -> i32 {
    let mut parts = input.split("\r\n\r\n");
    let mut pre_intervals: Vec<(i64, i64)> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut range = line.split('-').map(|x| x.parse::<i64>().unwrap());
            (range.next().unwrap(), range.next().unwrap())
        })
        .collect();

    pre_intervals.par_sort_unstable();
    let mut intervals = vec![pre_intervals[0]];
    for &(start, end) in pre_intervals.iter().skip(1) {
        let idx = intervals.len() - 1;
        if intervals[idx].1 >= start {
            intervals[idx].1 = intervals[idx].1.max(end);
        } else {
            intervals.push((start, end));
        }
    }
    let intervals = intervals;

    parts
        .next()
        .unwrap()
        .lines()
        .par_bridge()
        .map(|line| {
            let id = line.parse::<i64>().unwrap();
            intervals.binary_search_by(|&e| cafe_order(e, id)).is_ok() as i32
        })
        .sum()
}

// DAY 5 PART 2
pub fn cafeteria2(input: &'static str) -> i64 {
    let mut parts = input.split("\r\n\r\n");
    let mut pre_intervals: Vec<(i64, i64)> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut range = line.split('-').map(|x| x.parse::<i64>().unwrap());
            (range.next().unwrap(), range.next().unwrap())
        })
        .collect();

    pre_intervals.par_sort_unstable();
    let mut intervals = vec![pre_intervals[0]];
    for &(start, end) in pre_intervals.iter().skip(1) {
        let idx = intervals.len() - 1;
        if intervals[idx].1 >= start {
            intervals[idx].1 = intervals[idx].1.max(end);
        } else {
            intervals.push((start, end));
        }
    }

    intervals
        .par_iter()
        .chunks(1000)
        .map(|chunk| {
            chunk
                .iter()
                .map(|(start, end)| end - start + 1)
                .sum::<i64>()
        })
        .sum()
}

// DAY 6 PART 1
pub fn trash_compactor(input: &'static str) -> i64 {
    let mut all_ops = vec![];
    for op in input.lines().last().unwrap().split_whitespace() {
        all_ops.push(op.chars().next().unwrap());
    }

    let mut values: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|x| x.parse::<i64>().ok())
                .collect()
        })
        .collect();

    values.pop();

    let m = values.len();
    let n = values[0].len();

    let mut ans = 0;
    for j in 0..n {
        let mut curr = i64::from(all_ops[j] != '+');
        #[allow(clippy::needless_range_loop)]
        for i in 0..m {
            if all_ops[j] == '+' {
                curr += values[i][j];
            } else {
                curr *= values[i][j];
            }
        }
        ans += curr;
    }

    ans
}

// DAY 6 PART 2
pub fn trash_compactor2(input: &'static str) -> i64 {
    let mut all_chars: Vec<Vec<char>> = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    all_chars.pop();

    let mut all_ops = vec![];
    for op in input.lines().last().unwrap().split_whitespace() {
        all_ops.push(op.chars().next().unwrap());
    }

    let mut count_spaces = vec![];
    let mut count = 0;
    for letter in input.lines().last().unwrap().chars().skip(1) {
        if letter == ' ' {
            count += 1;
        } else {
            count_spaces.push(count);
            count = 0;
        }
    }
    count_spaces.push(count);

    let n = count_spaces.len();
    count_spaces[n - 1] += 1;

    let mut curr = 0;
    let mut ans = 0;
    for (&spaces, &op) in count_spaces.iter().zip(all_ops.iter()) {
        let mut total = i64::from(op != '+');
        for sub_col in 0..spaces {
            let mut num = String::new();
            for row in &all_chars {
                let potential_digit = row[(curr + sub_col) as usize];
                if potential_digit.is_ascii_digit() {
                    num.push(potential_digit);
                }
            }
            if num.is_empty() {
                continue;
            }
            if op == '+' {
                total += num.parse::<i64>().unwrap();
            } else {
                total *= num.parse::<i64>().unwrap();
            }
        }
        curr += spaces + 1;
        ans += total;
    }

    ans
}

// helper for laboratories input
fn create_grid_and_start(input: &'static str) -> (usize, Vec<Vec<bool>>) {
    let mut lines_iter = input.lines();
    let start = lines_iter
        .next()
        .unwrap()
        .chars()
        .position(|x| x == 'S')
        .unwrap();

    // skip blank line
    lines_iter.next();

    let mut grid: Vec<Vec<bool>> = vec![];
    // continue to skip the alternating blank lines
    while let (Some(line), Some(_)) = (lines_iter.next(), lines_iter.next()) {
        grid.push(line.bytes().map(|x| x == b'^').collect::<Vec<bool>>());
    }

    (start, grid)
}

// DAY 7 PART 1
pub fn laboratories(input: &'static str) -> i32 {
    let (start, grid) = create_grid_and_start(input);

    let mut ans = 0;
    let mut beams = AHashSet::new();
    beams.insert(start);
    for row in grid {
        let mut beams_to_be_added = vec![];
        let mut beams_to_be_removed = AHashSet::new();
        for (j, &val) in row.iter().enumerate() {
            if val && beams.contains(&j) {
                beams_to_be_removed.insert(j);
                beams_to_be_added.push(j - 1);
                beams_to_be_added.push(j + 1);
                ans += 1;
            }
        }

        beams.retain(|x| !beams_to_be_removed.contains(x));
        beams.extend(beams_to_be_added.iter());
    }

    ans
}

// DAY 7 PART 2
pub fn laboratories2(input: &'static str) -> i64 {
    let (start, grid) = create_grid_and_start(input);

    fn dpfn(grid: &[Vec<bool>], i: i32, j: i32, cache: &mut AHashMap<(i32, i32), i64>) -> i64 {
        if i >= grid.len() as i32 {
            return 1;
        }
        if j < 0 || j >= grid[i as usize].len() as i32 {
            return 0;
        }

        if let Some(&ans) = cache.get(&(i, j)) {
            return ans;
        }

        let best = if grid[i as usize][j as usize] {
            dpfn(grid, i + 1, j - 1, cache) + dpfn(grid, i + 1, j + 1, cache)
        } else {
            dpfn(grid, i + 1, j, cache)
        };
        cache.insert((i, j), best);
        best
    }

    dpfn(&grid, 0, start as i32, &mut AHashMap::new())
}

pub struct UnionFind {
    pub uf: Vec<usize>,
    pub rank: Vec<usize>,
    pub groups: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            uf: (0..n).collect(),
            rank: vec![1; n],
            groups: n,
        }
    }

    pub fn find_parent(&mut self, mut a: usize) -> usize {
        while self.uf[a] != a {
            // skips straight to grandparent
            self.uf[a] = self.uf[self.uf[a]];
            a = self.uf[a];
        }
        a
    }

    pub fn union_parents(&mut self, a: usize, b: usize) -> bool {
        let p1 = self.find_parent(a);
        let p2 = self.find_parent(b);
        if p1 == p2 {
            return false;
        }

        self.groups -= 1;

        if self.rank[p1] > self.rank[p2] {
            self.uf[p2] = p1;
            self.rank[p1] += self.rank[p2];
        } else {
            self.uf[p1] = p2;
            self.rank[p2] += self.rank[p1];
        }

        true
    }
}

const fn straight_line_dist(p1: (i32, i32, i32), p2: (i32, i32, i32)) -> i64 {
    let x = (p2.0 as i64 - p1.0 as i64) as i128;
    let y = (p2.1 as i64 - p1.1 as i64) as i128;
    let z = (p2.2 as i64 - p1.2 as i64) as i128;
    let a = x * x + y * y + z * z;

    (a as u128).isqrt() as i64
}

// helper for parsing playground input
fn create_coords(input: &'static str) -> Vec<(i32, i32, i32)> {
    input
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

fn create_shortest_dist_heap(
    coords: &[(i32, i32, i32)],
) -> BinaryHeap<(std::cmp::Reverse<i64>, (usize, usize))> {
    let mut shortest_distances = BinaryHeap::new();
    for (i, &p1) in coords.iter().enumerate() {
        for (j, &p2) in coords.iter().enumerate().skip(i + 1) {
            shortest_distances.push((std::cmp::Reverse(straight_line_dist(p1, p2)), (i, j)));
        }
    }

    shortest_distances
}

// DAY 8 PART 1
pub fn playground(input: &'static str, k: i32) -> i64 {
    let coords = create_coords(input);
    let mut shortest_distances = create_shortest_dist_heap(&coords);

    let mut uf = UnionFind::new(coords.len());
    let mut count = 0;
    while count < k
        && let Some((_, (i, j))) = shortest_distances.pop()
    {
        uf.union_parents(i, j);
        count += 1;
    }

    let mut v = uf.rank;
    v.par_sort_unstable();
    let n = v.len();

    (v[n - 1] * v[n - 2] * v[n - 3]) as i64
}

// DAY 8 PART 2
pub fn playground2(input: &'static str) -> i64 {
    let coords = create_coords(input);
    let mut shortest_distances = create_shortest_dist_heap(&coords);

    let mut uf = UnionFind::new(coords.len());
    while let Some((_, (i, j))) = shortest_distances.pop() {
        uf.union_parents(i, j);
        if uf.groups == 1 {
            return coords[i].0 as i64 * coords[j].0 as i64;
        }
    }

    -1
}

// helper for movie theater to parse input
fn create_tiles(input: &'static str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|line| {
            let mut line_iter = line.split(',');
            (
                line_iter.next().unwrap().parse::<i32>().unwrap(),
                line_iter.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect()
}

// DAY 9 PART 1
pub fn movie_theater(input: &'static str) -> i64 {
    let tiles = create_tiles(input);
    let mut ans = 0;
    for (i, &(p1x, p1y)) in tiles.iter().enumerate() {
        for &(p2x, p2y) in tiles.iter().skip(i + 1) {
            ans = ans.max(((p2x - p1x).abs() + 1) as i64 * ((p2y - p1y).abs() + 1) as i64);
        }
    }
    ans
}

// DAY 9 PART 2
pub fn movie_theater2(input: &'static str) -> i64 {
    let tiles = create_tiles(input);

    let mut edges = vec![];
    for (&(t1x, t1y), &(t2x, t2y)) in tiles.iter().tuple_windows() {
        edges.push((t1x.min(t2x), t1y.min(t2y), t1x.max(t2x), t1y.max(t2y)));
    }

    let ((t1x, t1y), (t2x, t2y)) = (tiles[tiles.len() - 1], tiles[0]);
    edges.push((t1x.min(t2x), t1y.min(t2y), t1x.max(t2x), t1y.max(t2y)));

    let mut ans = 0;
    for (i, &(p1x, p1y)) in tiles.iter().enumerate() {
        for &(p2x, p2y) in tiles.iter().skip(i + 1) {
            let potential_ans = ((p2x - p1x).abs() + 1) as i64 * ((p2y - p1y).abs() + 1) as i64;
            if potential_ans <= ans {
                continue;
            }

            if !edges.iter().any(|&(esx, esy, elx, ely)| {
                p1x.min(p2x) < elx && p1y.min(p2y) < ely && p1x.max(p2x) > esx && p1y.max(p2y) > esy
            }) {
                ans = ans.max(potential_ans);
            }
        }
    }
    ans
}

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

// DAY 10 PART 1
pub fn factory(input: &'static str) -> u16 {
    let mut targets = vec![];
    let mut buttons = vec![];
    for line in input.lines() {
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

// // DAY 10 PART 2
// pub fn factory2(input: &'static str) -> u16 {
//     let mut buttons = vec![];
//     let mut requirements = vec![];
//     for line in input.lines() {
//         let mut parts = line.split_ascii_whitespace();
//         parts.next();

//         let mut btns = vec![];
//         let remaining: Vec<&str> = parts.collect();
//         for section in remaining.iter().take(remaining.len() - 1) {
//             let s: Vec<char> = section.chars().collect();
//             let s: String = s[1..s.len() - 1].iter().collect();
//             btns.push(
//                 s.split(',')
//                     .fold(0u16, |acc, x| acc | (1 << x.parse::<u16>().unwrap())),
//             );
//         }
//         buttons.push(btns);

//         let s: Vec<char> = remaining.last().unwrap().chars().collect();
//         let s: String = s[1..s.len() - 1].iter().collect();
//         requirements.push(
//             s.split(',')
//                 .map(|x| x.parse::<u16>().unwrap())
//                 .collect::<Vec<u16>>(),
//         );
//     }

//     let mut ans = 0;
//     for (reqs, btns) in requirements.iter().zip(buttons.iter()) {
//         // add a visited memo to cut paths
//         ans += 1; //factory_dfs(btns, target);
//     }

//     ans
// }

#[allow(dead_code)]
const fn str_to_u32(s: &str) -> u32 {
    assert!(s.len() <= 4);
    let v = s.as_bytes();
    let mut ans = 0;
    let mut i = 0;
    while i < v.len() {
        ans |= (v[i] as u32) << (i * 8);
        i += 1;
    }
    ans
}

const fn str_to_u16(s: &str) -> u16 {
    assert!(s.len() <= 3);
    let v = s.as_bytes();
    //assert!(v.iter().all(u8::is_ascii_lowercase));

    let mut ans = 0;
    let mut i = 0;
    while i < v.len() as u8 {
        ans = ans * 26 + (v[i as usize] - b'a') as u16;
        i += 1;
    }
    ans
}

fn create_adj_list(input: &'static str) -> AHashMap<u16, SmallVec<[u16; 21]>> {
    // run without smallvec and get largest size to set as smallvec size
    let mut adj_list: AHashMap<u16, SmallVec<[u16; 21]>> = AHashMap::new();
    for line in input.lines() {
        let mut lines_iter = line.split(':');
        let key = lines_iter.next().unwrap();
        let mut values = vec![];
        for value in lines_iter.next().unwrap().split_whitespace() {
            values.push(str_to_u16(value));
        }

        adj_list
            .entry(str_to_u16(key))
            .or_default()
            .extend(values.into_iter());
    }
    adj_list
}

// DAY 11 PART 1
pub fn reactor(input: &'static str) -> i32 {
    let adj_list = create_adj_list(input);

    fn dp(
        adj_list: &AHashMap<u16, SmallVec<[u16; 21]>>,
        output: u16,
        val: u16,
        cache: &mut AHashMap<u16, i32>,
    ) -> i32 {
        if val == output {
            return 1;
        }

        if let Some(&ans) = cache.get(&val) {
            return ans;
        }

        let mut ans = 0;
        for &adj in &adj_list[&val] {
            ans += dp(adj_list, output, adj, cache);
        }

        cache.insert(val, ans);
        ans
    }

    dp(
        &adj_list,
        str_to_u16("out"),
        str_to_u16("you"),
        &mut AHashMap::new(),
    )
}

// DAY 11 PART 2
pub fn reactor2(input: &'static str) -> i64 {
    let adj_list = create_adj_list(input);

    #[allow(clippy::too_many_arguments)]
    fn dp(
        adj_list: &AHashMap<u16, SmallVec<[u16; 21]>>,
        output: u16,
        v1: u16,
        v2: u16,
        val: u16,
        has_v1: bool,
        has_v2: bool,
        cache: &mut AHashMap<(u16, bool, bool), i64>,
    ) -> i64 {
        if val == output {
            if has_v1 && has_v2 {
                return 1;
            }
            return 0;
        }

        if let Some(&ans) = cache.get(&(val, has_v1, has_v2)) {
            return ans;
        }

        let mut ans = 0;
        for &adj in &adj_list[&val] {
            ans += dp(
                adj_list,
                output,
                v1,
                v2,
                adj,
                has_v1 || val == v1,
                has_v2 || val == v2,
                cache,
            );
        }

        cache.insert((val, has_v1, has_v2), ans);
        ans
    }

    dp(
        &adj_list,
        str_to_u16("out"),
        str_to_u16("dac"),
        str_to_u16("fft"),
        str_to_u16("svr"),
        false,
        false,
        &mut AHashMap::new(),
    )
}

fn main() {
    let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .without_time()
        .init();

    assert!(reactor2(include_str!("../inputs/d11t2.txt")) == 2);
    info!("{:?}", reactor2(include_str!("../inputs/d11.txt")));
}
