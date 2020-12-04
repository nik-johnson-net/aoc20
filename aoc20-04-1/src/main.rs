extern crate hex;

mod input;

#[derive(Default)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    pub fn new<T: AsRef<[(String, String)]>>(tokens: T) -> Passport {
        let mut p = Passport::default();

        for (token, value) in tokens.as_ref() {
            match token.as_ref() {
                "byr" => p.byr = Some(value.to_owned()),
                "iyr" => p.iyr = Some(value.to_owned()),
                "eyr" => p.eyr = Some(value.to_owned()),
                "hgt" => p.hgt = Some(value.to_owned()),
                "hcl" => p.hcl = Some(value.to_owned()),
                "ecl" => p.ecl = Some(value.to_owned()),
                "pid" => p.pid = Some(value.to_owned()),
                "cid" => p.cid = Some(value.to_owned()),
                &_ => panic!("eeee"),
            }
        }

        p
    }

    pub fn valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    pub fn valid2(&self) -> Result<(), String> {
        if let Some(byr) = &self.byr {
            if byr.len() != 4 {
                return Err(format!("byr != 4"));
            }
            let n: usize = byr.parse().unwrap();
            if n < 1920 || n > 2002 {
                return Err(format!("byr {} outside range", n));
            }
        } else {
            return Err(format!("byr not found"));
        }

        if let Some(iyr) = &self.iyr {
            if iyr.len() != 4 {
                return Err(format!("iyr != 4"));
            }
            let n: usize = iyr.parse().unwrap();
            if n < 2010 || n > 2020 {
                return Err(format!("iyr {} outside range", iyr));
            }
        } else {
            return Err(format!("iyr missing"));
        }

        if let Some(eyr) = &self.eyr {
            if eyr.len() != 4 {
                return Err(format!("eyr != 4"));
            }
            let n: usize = eyr.parse().unwrap();
            if n < 2020 || n > 2030 {
                return Err(format!("eyr {} outside range", n));
            }
        } else {
            return Err(format!("eyr missing"));
        }

        if let Some(hgt) = &self.hgt {
            match &hgt[hgt.len() - 2..] {
                "cm" => {
                    let n: usize = hgt[0..hgt.len() - 2].parse().unwrap();
                    if n < 150 || n > 193 {
                        return Err(format!("hgt {} outside range", hgt));
                    }
                    ()
                }
                "in" => {
                    let n: usize = hgt[0..hgt.len() - 2].parse().unwrap();
                    if n < 59 || n > 76 {
                        return Err(format!("hgt {} outside range (in)", hgt));
                    }
                    ()
                }
                &_ => return Err(format!("hgt {} invalid", hgt)),
            }
        } else {
            return Err(format!("hgt missing"));
        }

        if let Some(hcl) = &self.hcl {
            if hcl.len() != 7 || hcl.chars().nth(0).unwrap() != '#' {
                return Err(format!("hcl invalid {}", hcl));
            }
            let hexcode = &hcl[1..hcl.len()];
            if hex::decode(hexcode).is_err() {
                return Err(format!("hcl {} not hex", hexcode));
            }
        } else {
            return Err(format!("hcl missing"));
        }

        if let Some(ecl) = &self.ecl {
            if !vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl.as_str()) {
                return Err(format!("ecl {} incorrect", ecl));
            }
        } else {
            return Err(format!("ecl missing"));
        }

        if let Some(pid) = &self.pid {
            if pid.len() != 9 {
                return Err(format!("pid != 9"));
            }
        } else {
            return Err(format!("pid missing"));
        }

        Ok(())
    }
}

fn solve<T: AsRef<str>>(input: T) -> usize {
    input
        .as_ref()
        .split("\n\n")
        .map(|x| {
            let tokens: Vec<(String, String)> = x
                .split("\n")
                .flat_map(|xx| {
                    xx.split(" ").map(|y| {
                        let mut z = y.split(":");
                        (z.next().unwrap().to_owned(), z.next().unwrap().to_owned())
                    })
                })
                .collect();
            Passport::new(&tokens).valid()
        })
        .filter(|x| *x)
        .count()
}

fn solve2<T: AsRef<str>>(input: T) -> usize {
    input
        .as_ref()
        .split("\n\n")
        .map(|x| {
            let tokens: Vec<(String, String)> = x
                .split("\n")
                .flat_map(|xx| {
                    xx.split(" ").map(|y| {
                        let mut z = y.split(":");
                        (z.next().unwrap().to_owned(), z.next().unwrap().to_owned())
                    })
                })
                .collect();
            match Passport::new(&tokens).valid2() {
                Ok(_) => true,
                Err(s) => {
                    println!("{}", s);
                    false
                }
            }
        })
        .filter(|x| *x)
        .count()
}

fn main() {
    println!("aoc20-04-1");

    println!("{}", solve(input::INPUT));
    println!("{}", solve2(input::INPUT));
}
