mod input;

fn main() {
    println!("aoc20-02");

    for (ii, i) in input::INPUT.iter().enumerate() {
        for (ij, j) in input::INPUT[ii..].iter().enumerate() {
            for k in input::INPUT[ij..].iter() {
                if (i + j + k) == 2020 {
                    let r = *i as u64 * *j as u64 * *k as u64;
                    println!("{}", r);
                    return;
                }
            }
        }
    }
}
