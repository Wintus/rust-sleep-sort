use std::thread;
use std::time::Duration;
use std::sync::mpsc::channel;

pub fn sleep_sort(ns: Vec<u32>) -> Vec<u32> {
    // transmit & receive, x is nonsense
    let (tx, rx) = channel();

    let threads: Vec<_> = ns.into_iter().map(|n| {
        let tx = tx.clone();

        thread::spawn(move || {
            let d = Duration::from_millis(n as u64 * 10);
            thread::sleep(d);

            tx.send(n).unwrap();
        })
    }).collect();

    rx.iter()
      .take(threads.len())
      .collect()
}
