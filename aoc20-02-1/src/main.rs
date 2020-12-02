mod input;

struct Password<'a> {
    required_char: char,
    range_start: usize,
    range_end: usize,
    password: &'a str,
}

impl<'a> Password<'a> {
    pub fn new(line: &'a str) -> Password<'a> {
        let mut parts = line.split(" ");
        let range_str = parts.next().unwrap();
        let required_char = parts.next().unwrap();
        let password = parts.next().unwrap();

        let mut range_iter = range_str.split("-");
        let range_start: usize = range_iter.next().unwrap().parse().expect("NaN");
        let range_end: usize = range_iter.next().unwrap().parse().expect("NaN");

        Password {
            required_char: required_char.chars().nth(0).unwrap(),
            range_start: range_start,
            range_end: range_end,
            password: password,
        }
    }

    pub fn valid(&self) -> bool {
        let count =
            self.password
                .chars()
                .fold(0, |a, c| if c == self.required_char { a + 1 } else { a });
        count >= self.range_start && count <= self.range_end
    }

    pub fn valid2(&self) -> bool {
        let first = self.password.chars().nth(self.range_start - 1).unwrap();
        let second = self.password.chars().nth(self.range_end - 1).unwrap();

        (first == self.required_char) ^ (second == self.required_char)
    }
}

fn solve<T: AsRef<str>>(input: T) -> usize {
    input
        .as_ref()
        .lines()
        .map(|l| Password::new(l).valid())
        .filter(|i| *i)
        .count()
}

fn solve2<T: AsRef<str>>(input: T) -> usize {
    input
        .as_ref()
        .lines()
        .map(|l| Password::new(l).valid2())
        .filter(|i| *i)
        .count()
}

fn main() {
    println!("aoc20-02-1");
    println!("{}", solve(input::INPUT));
    println!("{}", solve2(input::INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given() {
        let sample = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        assert_eq!(solve(sample), 2);
    }

    #[test]
    fn given2() {
        let sample = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        assert_eq!(solve2(sample), 1);
    }
}
