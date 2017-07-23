extern crate sleep_sort;

use sleep_sort::sleep_sort;
use std::env;

fn main() {
    let ns: Vec<_> = env::args()
        .skip(1)
        .map(|x| x.parse().unwrap_or(0))
        .collect();

    let ns = sleep_sort(ns);
    println!("{:?}", ns);
}
