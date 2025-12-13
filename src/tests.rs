#![cfg(test)]
use ::function_name::named;
use proptest::prelude::*;

use crate::*;
#[test]
#[named]
fn d1() {
    let t = std::time::Instant::now();
    assert!(secret_entrance(include_str!("../inputs/d1t1.txt")) == 3);
    assert!(secret_entrance(include_str!("../inputs/d1.txt")) == 1147);

    assert!(secret_entrance2("L50\nR100") == 2);
    assert!(secret_entrance2(include_str!("../inputs/d1t1.txt")) == 16);
    assert!(secret_entrance2(include_str!("../inputs/d1.txt")) == 6789);
    println!("{} {:?}", function_name!(), t.elapsed());
}

#[test]
#[named]
fn d2() {
    let t = std::time::Instant::now();
    assert!(gift_shop(include_str!("../inputs/d2t1.txt")) == 1_227_775_554);
    assert!(gift_shop(include_str!("../inputs/d2.txt")) == 23_534_117_921);

    assert!(gift_shop2(include_str!("../inputs/d2t1.txt")) == 4_174_379_265);
    assert!(gift_shop2(include_str!("../inputs/d2.txt")) == 31_755_323_497);
    println!("{} {:?}", function_name!(), t.elapsed());
}

#[test]
#[named]
fn d3() {
    let t = std::time::Instant::now();
    assert!(lobby(include_str!("../inputs/d3t1.txt")) == 357);
    assert!(lobby(include_str!("../inputs/d3.txt")) == 17031);

    assert!(lobby2_top_down(include_str!("../inputs/d3t1.txt")) == 3_121_910_778_619);
    assert!(lobby2_top_down(include_str!("../inputs/d3.txt")) == 168_575_096_286_051);

    assert!(lobby2(include_str!("../inputs/d3t1.txt")) == 3_121_910_778_619);
    assert!(lobby2(include_str!("../inputs/d3.txt")) == 168_575_096_286_051);
    println!("{} {:?}", function_name!(), t.elapsed());
}

#[test]
#[named]
fn d4() {
    let t = std::time::Instant::now();
    assert!(printing_department(include_str!("../inputs/d4t1.txt")) == 13);
    assert!(printing_department(include_str!("../inputs/d4.txt")) == 1551);

    assert!(printing_department2(include_str!("../inputs/d4t1.txt")) == 43);
    assert!(printing_department2(include_str!("../inputs/d4.txt")) == 9784);
    println!("{} {:?}", function_name!(), t.elapsed());
}

#[test]
#[named]
fn d5() {
    let t = std::time::Instant::now();
    assert!(cafeteria(include_str!("../inputs/d5t1.txt")) == 3);
    assert!(cafeteria(include_str!("../inputs/d5.txt")) == 698);

    assert!(cafeteria2(include_str!("../inputs/d5t1.txt")) == 14);
    assert!(cafeteria2(include_str!("../inputs/d5.txt")) == 352_807_801_032_167);
    println!("{} {:?}", function_name!(), t.elapsed());
}

#[test]
#[named]
fn d6() {
    let t = std::time::Instant::now();
    assert!(trash_compactor(include_str!("../inputs/d6t1.txt")) == 4_277_556);
    assert!(trash_compactor(include_str!("../inputs/d6.txt")) == 5_335_495_999_141);

    assert!(trash_compactor2(include_str!("../inputs/d6t1.txt")) == 3_263_827);
    assert!(trash_compactor2(include_str!("../inputs/d6.txt")) == 10_142_723_156_431);
    println!("{} {:?}", function_name!(), t.elapsed());
}

#[test]
#[named]
fn d7() {
    let t = std::time::Instant::now();
    assert!(laboratories(include_str!("../inputs/d7t1.txt")) == 21);
    assert!(laboratories(include_str!("../inputs/d7.txt")) == 1533);

    assert!(laboratories2(include_str!("../inputs/d7t1.txt")) == 40);
    assert!(laboratories2(include_str!("../inputs/d7.txt")) == 10_733_529_153_890);
    println!("{} {:?}", function_name!(), t.elapsed());
}

#[test]
#[named]
fn d8() {
    let t = std::time::Instant::now();
    assert!(playground(include_str!("../inputs/d8t1.txt"), 10) == 40);
    assert!(playground(include_str!("../inputs/d8.txt"), 1000) == 47040);

    assert!(playground2(include_str!("../inputs/d8t1.txt")) == 25272);
    assert!(playground2(include_str!("../inputs/d8.txt")) == 4_884_971_896);
    println!("{} {:?}", function_name!(), t.elapsed());
}

#[test]
#[named]
fn d9() {
    let t = std::time::Instant::now();
    assert!(movie_theater(include_str!("../inputs/d9t1.txt")) == 50);
    assert!(movie_theater(include_str!("../inputs/d9.txt")) == 4_781_235_324);

    assert!(movie_theater2(include_str!("../inputs/d9t1.txt")) == 24);
    assert!(movie_theater2(include_str!("../inputs/d9.txt")) == 1_566_935_900);
    println!("{} {:?}", function_name!(), t.elapsed());
}

#[test]
#[named]
fn d10() {
    let t = std::time::Instant::now();
    assert!(factory(include_str!("../inputs/d10t1.txt")) == 7);
    assert!(factory(include_str!("../inputs/d10.txt")) == 422);

    assert!(factory2(include_str!("../inputs/d10t1.txt")) == 33);
    assert!(factory2(include_str!("../inputs/d10.txt")) == 16361);
    println!("{} {:?}", function_name!(), t.elapsed());
}

#[test]
#[named]
fn d11() {
    let t = std::time::Instant::now();
    assert!(reactor(include_str!("../inputs/d11t1.txt")) == 5);
    assert!(reactor(include_str!("../inputs/d11.txt")) == 613);

    assert!(reactor2(include_str!("../inputs/d11t2.txt")) == 2);
    assert!(reactor2(include_str!("../inputs/d11.txt")) == 372_918_445_876_116);
    println!("{} {:?}", function_name!(), t.elapsed());
}

#[test]
#[named]
fn d12() {
    let t = std::time::Instant::now();
    assert!(christmas_tree_farm(include_str!("../inputs/d12.txt")) == 599);

    println!("{} {:?}", function_name!(), t.elapsed());
}

#[test]
fn test_matrix_rotation() {
    let m = [[true, true, true], [false, false, true], [true, true, true]];
    let mut tm = m;
    rotate_3x3(&mut tm);
    rotate_3x3(&mut tm);
    rotate_3x3(&mut tm);
    rotate_3x3(&mut tm);

    assert!(tm == m);
}

proptest! {
    #[test]
    fn test_straight_line_dist((x1, y1, z1) in (i32::MIN..=i32::MAX, i32::MIN..=i32::MAX, i32::MIN..=i32::MAX), (x2, y2, z2) in (i32::MIN..=i32::MAX, i32::MIN..=i32::MAX, i32::MIN..=i32::MAX)) {
        prop_assert!(straight_line_dist((x1, y1, z1), (x2, y2, z2)) >= 0);
    }

    #[test]
    fn test_str_to_u32_uniqueness(i1 in "[a-z]{3}", i2 in "[a-z]{3}") {
        prop_assume!(i1 != i2);

        let a1 = str_to_u32(&i1);
        let a2 = str_to_u32(&i2);
        let a3 = str_to_u32(&i1.to_ascii_uppercase());

        prop_assert!(a1 != a2);
        prop_assert!(a1 != a3);

        prop_assert!(a1 < 1 << 24);
        prop_assert!(a2 < 1 << 24);
        prop_assert!(a3 < 1 << 24);
    }

    #[test]
    fn test_str_to_u8_uniqueness(i1 in "[a-z]{3}", i2 in "[a-z]{3}") {
        prop_assume!(i1 != i2);

        let a1 = str_to_u16(&i1);
        let a2 = str_to_u16(&i2);

        prop_assert!(a1 != a2);
    }
}
