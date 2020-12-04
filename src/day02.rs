/*
--- Day 2: Password Philosophy ---
Your flight departs in a few days from the coastal airport; 
the easiest way down to the coast from here is via toboggan.

The shopkeeper at the North Pole Toboggan Rental Shop is having 
a bad day. "Something's wrong with our computers; we can't log in!" 
You ask if you can take a look.

Their password database seems to be a little corrupted: some of the 
passwords wouldn't have been allowed by the Official Toboggan 
Corporate Policy that was in effect when they were chosen.

To try to debug the problem, they have created a list (your puzzle input) 
of passwords (according to the corrupted database) and the corporate 
policy when that password was set.

For example, suppose you have the following list:

1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
Each line gives the password policy and then the password. The password 
policy indicates the lowest and highest number of times a given letter 
must appear for the password to be valid. For example, 1-3 a means that 
the password must contain a at least 1 time and at most 3 times.

In the above example, 2 passwords are valid. The middle password, cdefg, 
is not; it contains no instances of b, but needs at least 1. The first 
and third passwords are valid: they contain one a or nine c, both within 
the limits of their respective policies.

How many passwords are valid according to their policies?
*/

pub struct Rule {
    min: u8,
    max: u8,
    letter: char
}

impl Rule {
    pub fn parse(raw: &str) -> Rule {
        let split_raw: Vec<&str> = raw.split(" ").collect();
        // my use of unwrap() here is safe given the input in data/day02.txt
        let range = split_raw.get(0).unwrap();
        let letters = split_raw.get(1).unwrap();

        let split_range: Vec<&str> = range.split("-").collect();
        let min_str: &str = split_range.get(0).unwrap();
        let max_str: &str = split_range.get(1).unwrap();
        let min: u8 = min_str.parse().unwrap();
        let max: u8 = max_str.parse().unwrap();
        let letter: char = letters.chars().last().unwrap();
        // data/day02.txt has the property that the this will always be Some(c)
        return Rule { min, max, letter };
    }
    
    pub fn valid(self: &Rule, password: &str) -> bool {
        let mut letter_freq: u8 = 0;

        for c in password.chars() {
            if c == self.letter {
                letter_freq += 1;
            }
        }

        return letter_freq >= self.min && letter_freq <= self.max;
    }
}

#[test]
fn parse_rule_test() {
    let rule = Rule::parse("2-3 t");
    assert_eq!(2, rule.min);
    assert_eq!(3, rule.max);
    assert_eq!('t', rule.letter);
}


fn valid_password(raw_rule: &str, password: &str) -> bool {
    let rule = Rule::parse(raw_rule);
    return rule.valid(password);
}

#[test]
fn valid_password_test() {
    assert_eq!(true, valid_password("1-3 a", "abcde"));
    assert_eq!(false, valid_password("1-3 b", "cdefg"));
    assert_eq!(true, valid_password("2-9 c", "ccccccccc"));
}

/* --- Part Two ---
While it appears you validated the passwords correctly, they don't seem to be what the Official Toboggan Corporate Authentication System is expecting.

The shopkeeper suddenly realizes that he just accidentally explained the password policy rules from his old job at the sled rental place down the street! The Official Toboggan Corporate Policy actually works a little differently.

Each policy actually describes two positions in the password, where 1 means the first character, 2 means the second character, and so on. (Be careful; Toboggan Corporate Policies have no concept of "index zero"!) Exactly one of these positions must contain the given letter. Other occurrences of the letter are irrelevant for the purposes of policy enforcement.

Given the same example list from above:

1-3 a: abcde is valid: position 1 contains a and position 3 does not.
1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
How many passwords are valid according to the new interpretation of the policies? 

*/

impl Rule {
    pub fn valid_part2(self: &Rule, password: &str) -> bool {
        let n: usize = (self.min as usize) - 1;
        let m: usize = (self.max as usize) - 1;
        if let Some(c) = password.chars().nth(n) {
            if c == self.letter {
                if let Some(d) = password.chars().nth(m) {
                    if d != self.letter {
                        return true;
                    }
                }
            } else {
                if let Some(d) = password.chars().nth(m) {
                    if d == self.letter {
                        return true;
                    }
                }
            }
        }

        return false;
    }
}

pub fn valid_password_part2(raw_rule: &str, password: &str) -> bool {
    let rule = Rule::parse(raw_rule);
    return rule.valid_part2(password);
}

#[test]
fn valid_password_part_2_test() {
    assert_eq!(true, valid_password_part2("1-3 a", "abcde"));
    assert_eq!(false, valid_password_part2("1-3 b", "cdefg"));
    assert_eq!(false, valid_password_part2("2-9 c", "ccccccccc"));
}