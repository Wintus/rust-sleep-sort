use std::{env, thread};

fn main() {
    let ns: Vec<u32> = env::args()
        .skip(1)
        .map(|x| x.parse().unwrap_or(0))
        .collect();
    println!("{:?}", ns);
}
