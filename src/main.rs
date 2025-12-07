#![feature(const_trait_impl, const_range)]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[allow(unused_imports)]
#[macro_use]
extern crate static_assertions;
use std::collections::{HashMap, HashSet};

use ahash::RandomState;
use rayon::prelude::*;
mod tests;

pub fn combination_lock_code(input: &'static str) -> i32 {
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

pub fn pair_lists(input: &str) -> i32 {
    let mut list1 = vec![];
    let mut list2 = HashMap::new();
    for line in input.lines() {
        let nums: Vec<&str> = line.split_whitespace().collect();
        list1.push(nums[0].parse::<i32>().unwrap());
        *list2.entry(nums[1].parse::<i32>().unwrap()).or_insert(0) += 1;
    }

    let mut ans = 0;
    for num in list1 {
        if let Some(&val) = list2.get(&num) {
            ans += num * val;
        }
    }
    ans
}

pub fn corrupted_memory(input: &str) -> i32 {
    let mut ans = 0;
    let mut handle_mul = |mul: &str| {
        if let Some(vals) = mul.split(')').collect::<Vec<&str>>().first() {
            let nums = vals.split(',').collect::<Vec<&str>>();
            if let Some(n1) = nums.first()
                && let Some(n2) = nums.get(1)
                && let Ok(v1) = n1.parse::<i32>()
                && let Ok(v2) = n2.parse::<i32>()
            {
                ans += v1 * v2;
            }
        }
    };
    for fn_do in input.split("do()").skip(1) {
        let muls = fn_do.split("mul(").collect::<Vec<&str>>();
        if muls[0].contains("don't()") {
            continue;
        }
        let mut next = true;
        for mul in muls {
            if !next {
                break;
            }
            if mul.contains("don't()") {
                next = false;
            }
            handle_mul(mul);
        }
    }

    if let Some(mul) = input.split("mul(").collect::<Vec<&str>>().get(1) {
        handle_mul(mul);
    }

    ans
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

pub fn count_xmas(input: &str) -> i32 {
    let mut grid = vec![];
    for line in input.lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }

    let m = grid.len();
    let n = grid[0].len();
    let xmas = ['X', 'M', 'A', 'S'];
    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            for (di, dj) in DIRS_8 {
                let mut found = true;
                for k in 0..4 {
                    let (ni, nj) = (i as i32 + di * k, j as i32 + dj * k);
                    if (0..m as i32).contains(&ni)
                        && (0..n as i32).contains(&nj)
                        && grid[ni as usize][nj as usize] == xmas[k as usize]
                    {
                        continue;
                    }
                    found = false;
                    break;
                }
                if found {
                    ans += 1;
                }
            }
        }
    }

    ans
}

pub fn count_xmas2(input: &str) -> i32 {
    let mut grid = vec![];
    for line in input.lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }

    let m = grid.len();
    let n = grid[0].len();
    let mas = ['M', 'A', 'S'];
    let sam = ['S', 'A', 'M'];

    let mut ans = 0;
    for i in 1..m - 1 {
        for j in 1..n - 1 {
            let pos = [grid[i - 1][j - 1], grid[i][j], grid[i + 1][j + 1]];
            let neg = [grid[i + 1][j - 1], grid[i][j], grid[i - 1][j + 1]];

            if (pos == sam || pos == mas) && (neg == sam || neg == mas) {
                ans += 1;
            }
        }
    }

    ans
}

pub fn count_invalid_ids(input: &str) -> i64 {
    input
        .split(',')
        .par_bridge()
        .map(|range_str| {
            let mut range = range_str.split('-').map(|x| x.parse::<i64>().unwrap());

            let mut total = 0;
            for num in range.next().unwrap()..=range.next().unwrap() {
                let str_num: Vec<char> = num.to_string().chars().collect();
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

pub fn count_invalid_ids2(input: &str) -> i64 {
    input
        .split(',')
        .par_bridge()
        .map(|range_str| {
            let mut range = range_str.split('-').map(|x| x.parse::<i64>().unwrap());

            let mut total = 0;
            for num in range.next().unwrap()..=range.next().unwrap() {
                let str_num: Vec<char> = num.to_string().chars().collect();
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

pub fn print_queue(input: &str) -> i32 {
    let parts: Vec<&str> = input.split("\r\n\r\n").collect();
    let (p1, p2) = (parts[0], parts[1]);
    let mut comes_before: HashMap<i32, HashSet<i32, RandomState>, RandomState> = HashMap::default();
    for line in p1.lines() {
        let s: Vec<&str> = line.split('|').collect();
        let (n1, n2) = (s[0].parse::<i32>().unwrap(), s[1].parse::<i32>().unwrap());
        comes_before.entry(n1).or_default().insert(n2);
    }

    let mut ans = 0;
    for line in p2.lines() {
        let nums: Vec<i32> = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
        let mut all = true;
        let mut found: HashSet<i32, RandomState> = HashSet::default();
        for &num in &nums {
            if let Some(hs) = comes_before.get(&num)
                && found.intersection(hs).next().is_some()
            {
                all = false;
                break;
            }

            found.insert(num);
        }
        if all {
            ans += nums[nums.len() / 2];
        }
    }
    ans
}

// pub fn print_queue2(input: &str) -> i32 {
//     let parts: Vec<&str> = input.split("\r\n\r\n").collect();
//     let (p1, p2) = (parts[0], parts[1]);
//     let mut adj_list: HashMap<
//         i32,
//         (HashSet<i32, RandomState>, HashSet<i32, RandomState>),
//         RandomState,
//     > = HashMap::default();
//     for line in p1.lines() {
//         let s: Vec<&str> = line.split('|').collect();
//         let (n1, n2) = (s[0].parse::<i32>().unwrap(), s[1].parse::<i32>().unwrap());
//         adj_list.entry(n2).or_default().0.insert(n1);
//         adj_list.entry(n1).or_default().1.insert(n2);
//     }

//     let mut bad_nums = vec![];
//     for line in p2.lines() {
//         let nums: Vec<i32> = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
//         let mut all = true;
//         let mut found: HashSet<i32, RandomState> = HashSet::default();
//         for &num in &nums {
//             if let Some((_, after)) = adj_list.get(&num)
//                 && found.intersection(after).next().is_some()
//             {
//                 all = false;
//                 break;
//             }

//             found.insert(num);
//         }
//         if !all {
//             bad_nums.push(nums);
//         }
//     }
//     let mut ans = 0;

//     for mut nums in bad_nums {
//         let mut visited: HashSet<i32, RandomState> = HashSet::default();
//         let mut st = nums[0];

//     }
//     ans
// }

pub fn count_distinct_positions(input: &str) -> i32 {
    let mut map = vec![];
    let mut curr: (i32, i32) = (0, 0);

    let mut dir_idx = 0;
    let dirs = ['^', '>', 'v', '<'];
    let dirs_hm: HashMap<char, (i32, i32)> =
        HashMap::from([('v', (1, 0)), ('>', (0, 1)), ('<', (0, -1)), ('^', (-1, 0))]);
    for (r, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (c, val) in line.chars().enumerate() {
            if val == '.' {
                row.push(false);
            } else if val == '#' {
                row.push(true);
            } else if dirs_hm.contains_key(&val) {
                curr = (r as i32, c as i32);
                dir_idx = dirs.iter().position(|x| *x == val).unwrap();
                row.push(false);
            }
        }
        map.push(row);
    }

    let mut visited: HashSet<(i32, i32), RandomState> = HashSet::default();
    let m = map.len();
    let n = map[0].len();
    let in_bounds = |test: (i32, i32)| -> bool {
        (0..m as i32).contains(&test.0) && (0..n as i32).contains(&test.1)
    };
    let add_tuple = |t1: (i32, i32), t2: (i32, i32)| -> (i32, i32) { (t1.0 + t2.0, t1.1 + t2.1) };
    while in_bounds(curr) {
        visited.insert((curr.0, curr.1));
        let mut next = add_tuple(curr, dirs_hm[&dirs[dir_idx]]);
        while in_bounds(next) && map[next.0 as usize][next.1 as usize] {
            dir_idx = (dir_idx + 1) % dirs.len();
            next = add_tuple(curr, dirs_hm[&dirs[dir_idx]]);
        }
        curr = next;
    }

    visited.len() as i32
}

pub fn count_distinct_positions2(input: &str) -> i32 {
    let mut map = vec![];
    let mut curr: (i32, i32) = (0, 0);

    let mut dir_idx = 0;
    let dirs = ['^', '>', 'v', '<'];
    let dirs_hm: HashMap<char, (i32, i32)> =
        HashMap::from([('v', (1, 0)), ('>', (0, 1)), ('<', (0, -1)), ('^', (-1, 0))]);
    for (r, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (c, val) in line.chars().enumerate() {
            if val == '.' {
                row.push(false);
            } else if val == '#' {
                row.push(true);
            } else if dirs_hm.contains_key(&val) {
                curr = (r as i32, c as i32);
                dir_idx = dirs.iter().position(|x| *x == val).unwrap();
                row.push(false);
            }
        }
        map.push(row);
    }

    let mut visited: HashMap<(i32, i32), [bool; 4], RandomState> = HashMap::default();
    let m = map.len();
    let n = map[0].len();
    let in_bounds = |test: (i32, i32)| -> bool {
        (0..m as i32).contains(&test.0) && (0..n as i32).contains(&test.1)
    };
    let add_tuple = |t1: (i32, i32), t2: (i32, i32)| -> (i32, i32) { (t1.0 + t2.0, t1.1 + t2.1) };
    let mut ans = 0;
    while in_bounds(curr) {
        if visited.entry(curr).or_default()[dir_idx] {
            ans += 1;
        }
        let mut next = add_tuple(curr, dirs_hm[&dirs[dir_idx]]);
        while in_bounds(next) && map[next.0 as usize][next.1 as usize] {
            dir_idx = (dir_idx + 1) % dirs.len();
            next = add_tuple(curr, dirs_hm[&dirs[dir_idx]]);
        }
        curr = next;
    }

    ans
}

pub fn lobby(input: &str) -> i32 {
    let mut ans = 0;
    for line in input.lines() {
        let mut largest = i32::MIN / 100;
        let mut best = 0;
        for digit_char in line.chars() {
            if let Some(digit) = digit_char.to_digit(10) {
                best = best.max(largest * 10 + digit as i32);
                largest = largest.max(digit as i32);
            }
        }
        if largest == i32::MIN / 100 {
            continue;
        }
        ans += best;
    }
    ans
}

pub fn lobby2_top_down(input: &str) -> i64 {
    fn dp(nums: &Vec<i32>, i: usize, count: i32, cache: &mut HashMap<(usize, i32), i64>) -> i64 {
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

            dp(&nums, 0, 0, &mut HashMap::new())
        })
        .sum()
}

pub fn lobby2(input: &str) -> i64 {
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

pub fn printing_department(input: &str) -> i32 {
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

pub fn printing_department2(input: &str) -> i32 {
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

pub fn cafeteria(input: &str) -> i32 {
    let mut parts = input.split("\r\n\r\n");
    let mut pre_intervals: Vec<(i64, i64)> = parts
        .next()
        .unwrap()
        .lines()
        .par_bridge()
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

pub fn cafeteria2(input: &'static str) -> i64 {
    let mut parts = input.split("\r\n\r\n");
    let mut pre_intervals: Vec<(i64, i64)> = parts
        .next()
        .unwrap()
        .lines()
        .par_bridge()
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
        .map(|(start, end)| end - start + 1)
        .sum()
}

pub fn trash_compactor(input: &'static str) -> i64 {
    let mut values: Vec<Vec<i64>> = vec![];

    let mut all_ops = vec![];
    if let Some(ops) = input.lines().last() {
        for op in ops.split_whitespace() {
            all_ops.push(op.chars().next().unwrap());
        }
    }
    for line in input.lines() {
        values.push(
            line.split_whitespace()
                .filter_map(|x| x.parse::<i64>().ok())
                .collect(),
        );
    }
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

pub fn trash_compactor2(input: &'static str) -> i64 {
    let mut all_chars: Vec<Vec<char>> = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    all_chars.pop();

    let mut all_ops = vec![];
    if let Some(ops) = input.lines().last() {
        for op in ops.split_whitespace() {
            all_ops.push(op.chars().next().unwrap());
        }
    }

    let mut count_spaces = vec![];
    if let Some(part) = input.lines().last() {
        let mut count = 0;
        for letter in part.chars().skip(1) {
            if letter == ' ' {
                count += 1;
            } else {
                count_spaces.push(count);
                count = 0;
            }
        }
        count_spaces.push(count);
    }

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

pub fn laboratories(input: &'static str) -> i32 {
    let mut v = vec![];
    let mut lines_iter = input.lines();
    let start = lines_iter.next().map_or_else(
        || panic!(),
        |first_line| first_line.chars().position(|x| x == 'S').unwrap(),
    );
    lines_iter.next();
    while let (Some(line), Some(_)) = (lines_iter.next(), lines_iter.next()) {
        v.push(line.bytes().map(|x| x == b'^').collect::<Vec<bool>>());
    }

    let mut ans = 0;
    let mut beams: HashSet<usize, RandomState> = HashSet::default();
    beams.insert(start);
    for row in v {
        let mut beams_to_be_added = vec![];
        let mut beams_to_be_removed: HashSet<usize, RandomState> = HashSet::default();
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

pub fn laboratories2(input: &'static str) -> i64 {
    let mut v = vec![];
    let mut lines_iter = input.lines();
    let start = lines_iter.next().map_or_else(
        || panic!(),
        |first_line| first_line.chars().position(|x| x == 'S').unwrap(),
    );
    lines_iter.next();
    while let (Some(line), Some(_)) = (lines_iter.next(), lines_iter.next()) {
        v.push(line.bytes().map(|x| x == b'^').collect::<Vec<bool>>());
    }

    fn dpfn(
        grid: &[Vec<bool>],
        i: i32,
        j: i32,
        cache: &mut HashMap<(i32, i32), i64, RandomState>,
    ) -> i64 {
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

    dpfn(&v, 0, start as i32, &mut HashMap::default())
}

fn main() {
    assert!(laboratories2(include_str!("../2025/d7t1.txt")) == 40);
    println!("{:?}", laboratories2(include_str!("../2025/d7.txt")));
}
