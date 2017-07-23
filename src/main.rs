extern crate sleep_sort;

use sleep_sort::sleep_sort;
use std::env;

fn main() {
    let ns: Vec<_> = env::args()
        .skip(1)
        .map(|x| x.parse().unwrap_or(0))
        .collect();

    let ns: Vec<_> = sleep_sort(ns);
    println!("{:?}", ns);
    let sorted =
        ns.into_iter()
          .fold((true, 0), |(sorted, last_n), n| (sorted && last_n <= n, n))
            .0;
    println!("sorted: {}", sorted);
}
