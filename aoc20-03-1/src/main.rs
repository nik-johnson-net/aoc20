mod input;

struct Mountain {
    width: usize,
    mountain: Vec<String>,
}

impl Mountain {
    pub fn new<T: AsRef<str>>(mtn: T) -> Mountain {
        let parsed: Vec<String> = mtn.as_ref().lines().map(|s| s.to_owned()).collect();
        Mountain {
            width: parsed.first().expect("non-zero mountain").len(),
            mountain: parsed,
        }
    }

    pub fn traverse_tree_count(&self, right: usize, down: usize) -> usize {
        self.mountain
            .iter()
            .enumerate()
            .filter(|(idx, _x)| idx % down == 0)
            .map(|(_, x)| x)
            .enumerate()
            .map(|(idx, x)| {
                let position = (idx * right) % self.width;
                x.chars().nth(position).unwrap()
            })
            .filter(|x| *x == '#')
            .count()
    }
}

fn main() {
    println!("Hello, world!");
    let mtn = Mountain::new(input::INPUT);
    println!("{}", mtn.traverse_tree_count(3, 1));

    let part2 = vec![
        mtn.traverse_tree_count(1, 1),
        mtn.traverse_tree_count(3, 1),
        mtn.traverse_tree_count(5, 1),
        mtn.traverse_tree_count(7, 1),
        mtn.traverse_tree_count(1, 2),
    ]
    .iter()
    .fold(1, |a, x| a * x);
    println!("{}", part2);
}
