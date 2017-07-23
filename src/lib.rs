use std::thread;
use std::time::Duration;
use std::sync::mpsc::channel;

type F = f32;
type MS = u64;

const SCALE: F = 20.0; // for accuracy

pub fn sleep_sort(ns: Vec<u32>) -> Vec<u32> {
    // transmit & receive, x is nonsense
    let (tx, rx) = channel();

    // 並列睡眠
    let count = ns.into_iter().map(|n| {
        let tx = tx.clone();

        thread::spawn(move || {
            let ms = (n as F).log2() * SCALE;
            let d = Duration::from_millis(ms as MS);
            thread::sleep(d);

            tx.send(n).unwrap();
        })
    }).count(); // count で消費

    rx.iter().take(count).collect()
}

mod tests;
