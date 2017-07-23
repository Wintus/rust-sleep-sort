use std::{env, thread};
use std::time::Duration;
use std::sync::{Arc, Mutex};

fn main() {
    let sorted = Arc::new(Mutex::new(Vec::new()));

    let ns: Vec<_> = env::args()
        .skip(1)
        .map(|x| x.parse().unwrap_or(0))
        .collect();

    let threads: Vec<_> = ns.into_iter().map(|n| {
        let sorted = sorted.clone();

        thread::spawn(move || {
            let d = Duration::from_millis(n * 10);
            thread::sleep(d);

            let mut sorted = sorted.lock().unwrap();
            sorted.push(n);
        })
    }).collect();

    for thread in threads {
        let _ = thread.join();
    }

    println!("{:?}", sorted);
}
