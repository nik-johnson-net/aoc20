mod input;

use std::collections::HashSet;

fn main() {
    println!("aoc20-01");

    let mut numbers = HashSet::new();

    for i in &input::INPUT {
        let search = 2020 - i;
        if let Some(r) = numbers.get(&search) {
            println!("{}", (*r as u64) * (*i as u64));
            break;
        } else {
            numbers.insert(*i);
        }
    }
}
