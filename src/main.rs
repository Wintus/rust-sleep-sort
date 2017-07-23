use std::{env, thread};
use std::time::Duration;
use std::sync::mpsc::channel;

fn main() {
    let (tx, rx) = channel();

    let ns: Vec<_> = env::args()
        .skip(1)
        .map(|x| x.parse().unwrap_or(0))
        .collect();

    let threads: Vec<_> = ns.into_iter().map(|n| {
        let tx = tx.clone();

        thread::spawn(move || {
            let d = Duration::from_millis(n * 10);
            thread::sleep(d);

            tx.send(n).unwrap();
        })
    }).collect();

    let ns: Vec<_> = rx.iter().take(threads.len()).collect();
    println!("{:?}", ns);
}
