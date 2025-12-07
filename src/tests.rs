#![cfg(test)]

use crate::*;
#[test]
fn d1() {
    let t = std::time::Instant::now();
    assert!(combination_lock_code("L50\nR100") == 2);
    assert!(combination_lock_code(include_str!("../2025/d1t1.txt")) == 16);
    assert!(combination_lock_code(include_str!("../2025/d1.txt")) == 6789);
    println!("d1 {:?}", t.elapsed());
}

#[test]
fn d2() {
    let t = std::time::Instant::now();
    assert!(count_invalid_ids(include_str!("../2025/d2t1.txt")) == 1_227_775_554);
    assert!(count_invalid_ids(include_str!("../2025/d2.txt")) == 23_534_117_921);

    assert!(count_invalid_ids2(include_str!("../2025/d2t1.txt")) == 4_174_379_265);
    assert!(count_invalid_ids2(include_str!("../2025/d2.txt")) == 31_755_323_497);
    println!("d2 {:?}", t.elapsed());
}

#[test]
fn d3() {
    let t = std::time::Instant::now();
    assert!(lobby(include_str!("../2025/d3t1.txt")) == 357);
    assert!(lobby(include_str!("../2025/d3.txt")) == 17031);

    assert!(lobby2_top_down(include_str!("../2025/d3t1.txt")) == 3_121_910_778_619);
    assert!(lobby2_top_down(include_str!("../2025/d3.txt")) == 168_575_096_286_051);

    assert!(lobby2(include_str!("../2025/d3t1.txt")) == 3_121_910_778_619);
    assert!(lobby2(include_str!("../2025/d3.txt")) == 168_575_096_286_051);
    println!("d3 {:?}", t.elapsed());
}

#[test]
fn d4() {
    let t = std::time::Instant::now();
    assert!(printing_department(include_str!("../2025/d4t1.txt")) == 13);
    assert!(printing_department(include_str!("../2025/d4.txt")) == 1551);

    assert!(printing_department2(include_str!("../2025/d4t1.txt")) == 43);
    assert!(printing_department2(include_str!("../2025/d4.txt")) == 9784);
    println!("d4 {:?}", t.elapsed());
}

#[test]
fn d5() {
    let t = std::time::Instant::now();
    assert!(cafeteria(include_str!("../2025/d5t1.txt")) == 3);
    assert!(cafeteria(include_str!("../2025/d5.txt")) == 698);

    assert!(cafeteria2(include_str!("../2025/d5t1.txt")) == 14);
    assert!(cafeteria2(include_str!("../2025/d5.txt")) == 352_807_801_032_167);
    println!("d5 {:?}", t.elapsed());
}

#[test]
fn d6() {
    let t = std::time::Instant::now();
    assert!(trash_compactor(include_str!("../2025/d6t1.txt")) == 4_277_556);
    assert!(trash_compactor(include_str!("../2025/d6.txt")) == 5_335_495_999_141);

    assert!(trash_compactor2(include_str!("../2025/d6t1.txt")) == 3_263_827);
    assert!(trash_compactor2(include_str!("../2025/d6.txt")) == 10_142_723_156_431);
    println!("d6 {:?}", t.elapsed());
}

#[test]
fn d7() {
    let t = std::time::Instant::now();
    assert!(laboratories(include_str!("../2025/d7t1.txt")) == 21);
    assert!(laboratories(include_str!("../2025/d7.txt")) == 1533);

    assert!(laboratories2(include_str!("../2025/d7t1.txt")) == 40);
    assert!(laboratories2(include_str!("../2025/d7.txt")) == 10_733_529_153_890);
    println!("d7 {:?}", t.elapsed());
}
