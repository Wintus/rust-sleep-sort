use std::thread;
use std::time::Duration;
use std::sync::mpsc::channel;

type F = f32;
type U = u32;

const SCALE: F = 1_000_000_000.0; // in seconds order

pub fn sleep_sort(ns: Vec<U>) -> Vec<U> {
    // transmit & receive, x is nonsense
    let (tx, rx) = channel();

    // 並列睡眠
    let count = ns.into_iter().map(|n| {
        let tx = tx.clone();

        thread::spawn(move || {
            let ns = (n as F).ln_1p() * SCALE;
            let d = Duration::new(0, ns as U);
            thread::sleep(d);

            tx.send(n).unwrap();
        })
    }).count(); // count で消費

    rx.iter().take(count).collect()
}

mod tests;
