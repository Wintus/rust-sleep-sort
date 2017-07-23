use std::{env, thread};
use std::time::Duration;

fn main() {
    let ns: Vec<_> = env::args()
        .skip(1)
        .map(|x| x.parse().unwrap_or(0))
        .collect();
    let threads: Vec<_> = ns.into_iter().map(|n| {
        thread::spawn(move || {
            let d = Duration::from_millis(n);
            thread::sleep(d);
            println!("{}", n);
        })
    }).collect();
    for thread in threads {
        let _ = thread.join();
    }
}
