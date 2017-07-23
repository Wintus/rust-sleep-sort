#![allow(unused_imports)]

use super::*;
// TODO: benchmark

#[test]
fn test_case_0() {
    let ns = vec![5, 8, 3, 9, 6, 1, 13, 7];
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
