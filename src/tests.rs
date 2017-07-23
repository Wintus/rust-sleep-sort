#![allow(unused_imports)]

use super::*;
// TODO: benchmark

#[test]
fn test_case_0() {
    let ns = vec![5, 8, 3, 9, 6, 0, 1, 13, 7];
    let mut sorted = ns.clone();
    sorted.sort();
    assert_eq!(sorted, sleep_sort(ns));
}

#[test]
fn test_case_1() {
    let ns = vec![7, 8, 3, 9, 6, 6, 3, 7];
    let mut sorted = ns.clone();
    sorted.sort();
    assert_eq!(sorted, sleep_sort(ns));
}

#[test]
fn test_case_2() {
    let ns = vec![0, 0, 1, 1, 2, 7, 8, 3, 9, 6, 6, 3, 7];
    let mut sorted = ns.clone();
    sorted.sort();
    assert_eq!(sorted, sleep_sort(ns));
}

#[test]
fn test_log() {
    let max: u32 = 30;

    let mut ns: Vec<u32> = (0..max).map(|n| {
        // logX 使うと、 0 -> -inf で 1 と差がつかない
        let log = (n as f32).ln_1p() * SCALE;
        log.round() as u32
    }).collect();

    let first = ns.remove(0);
    let sorted: bool = ns.into_iter().fold((true, first), |(sorted, prev), n| {
        let b = sorted && prev < n;
        (b, n)
    }).0;

    assert!(sorted);
}
