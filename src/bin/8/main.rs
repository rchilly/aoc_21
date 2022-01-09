use std::collections::HashMap;
use std::str::FromStr;

const ZERO: &str = "abcefg";
const ONE: &str = "cf";
const TWO: &str = "acdeg";
const THREE: &str = "acdfg";
const FOUR: &str = "bcdf";
const FIVE: &str = "abdfg";
const SIX: &str = "abdefg";
const SEVEN: &str = "acf";
const EIGHT: &str = "abcdefg";
const NINE: &str = "abcdfg";

fn contains_chars_of(s: &str, of: &str) -> bool {
    for c in of.chars() {
        if !s.contains(c) {
            return false;
        }
    }

    true
}

struct SignalGroup {
    zero_to_nine: HashMap<&'static str, String>,
    output: Vec<String>,
}

impl SignalGroup {
    fn new(mut digits: Vec<String>, output: Vec<String>) -> Result<SignalGroup, String> {
        let mut zero_to_nine = HashMap::new();

        for unique in [ONE, FOUR, SEVEN, EIGHT] {
            let digit = digits
                .iter()
                .find(|d| d.len() == unique.len())
                .ok_or("unique length digit not found")?;

            zero_to_nine.insert(unique, digit.clone());
        }

        let seven = zero_to_nine.get(SEVEN).unwrap();

        let three = digits
            .iter()
            .find(|d| d.len() == THREE.len() && contains_chars_of(d, seven))
            .ok_or("three not found")?;

        zero_to_nine.insert(THREE, three.clone());

        let nine = digits
            .iter()
            .find(|d| contains_chars_of(d, three) && d.len() == three.len() + 1)
            .ok_or("nine not found")?;

        zero_to_nine.insert(NINE, nine.clone());

        let five = digits
            .iter()
            .find(|d| {
                let three = zero_to_nine.get(THREE).unwrap();
                let nine = zero_to_nine.get(NINE).unwrap();
                d.len() == FIVE.len() && contains_chars_of(nine, d) && *d != three
            })
            .ok_or("five not found")?;

        zero_to_nine.insert(FIVE, five.clone());

        // 1, 3, 4, 5, 7, 8, 9

        digits.retain(|d| !zero_to_nine.values().any(|v| v == d));

        // At this point, 0, 2 and 6 remain.

        let zero = digits
            .iter()
            .find(|d| {
                let seven = zero_to_nine.get(SEVEN).unwrap();
                d.len() == ZERO.len() && contains_chars_of(d, seven)
            })
            .ok_or("zero not found")?;

        zero_to_nine.insert(ZERO, zero.clone());

        let six = digits
            .iter()
            .find(|d| {
                let five = zero_to_nine.get(FIVE).unwrap();
                d.len() == SIX.len() && contains_chars_of(d, five)
            })
            .ok_or("six not found")?;

        let two = digits
            .iter()
            .find(|d| d.len() == TWO.len())
            .ok_or("two not found")?;

        zero_to_nine.insert(SIX, six.clone());
        zero_to_nine.insert(TWO, two.clone());

        Ok(SignalGroup {
            zero_to_nine,
            output,
        })
    }

    pub fn output_as_digits(&self) -> [&'static str; 4] {
        let mut decoded = [""; 4];
        for (i, o) in self.output.iter().enumerate() {
            for (k, v) in self.zero_to_nine.iter() {
                if v.len() == o.len() && contains_chars_of(v, o) {
                    decoded[i] = k;
                    break;
                }
            }
        }

        decoded
    }

    pub fn output_as_number(&self) -> Result<usize, std::num::ParseIntError> {
        let mut number = String::with_capacity(4);
        for o in &self.output {
            for (k, v) in self.zero_to_nine.iter() {
                if v.len() == o.len() && contains_chars_of(v, o) {
                    match *k {
                        ZERO => number.push('0'),
                        ONE => number.push('1'),
                        TWO => number.push('2'),
                        THREE => number.push('3'),
                        FOUR => number.push('4'),
                        FIVE => number.push('5'),
                        SIX => number.push('6'),
                        SEVEN => number.push('7'),
                        EIGHT => number.push('8'),
                        NINE => number.push('9'),
                        _ => (),
                    }

                    break;
                }
            }
        }

        number.parse::<usize>()
    }
}

impl FromStr for SignalGroup {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits: Vec<String> = Vec::with_capacity(10);
        let mut output: Vec<String> = Vec::with_capacity(4);

        let mut reached_output = false;
        for sub in s.split_ascii_whitespace() {
            if sub == "|" {
                reached_output = true;
                continue;
            }

            if reached_output {
                output.push(sub.to_owned())
            } else {
                digits.push(sub.to_owned())
            }
        }

        if digits.len() != 10 {
            return Err(format!(
                "expected 10 space-delimited digits before '|', got {}",
                digits.len()
            ));
        }

        if output.len() != 4 {
            return Err(format!(
                "expected 4 space-delimited digits after '|', got {}",
                output.len()
            ));
        }

        SignalGroup::new(digits, output)
    }
}

fn main() {
    let input = match advent_21::read_input(8) {
        Err(why) => panic!("failed to read input: {}", why),
        Ok(v) => v,
    };

    let mut groups: Vec<SignalGroup> = Vec::with_capacity(input.len());
    for line in input.into_iter() {
        groups.push(SignalGroup::from_str(&line).unwrap());
    }

    let mut tally = 0;
    for group in &groups {
        let decoded = group.output_as_digits();
        for d in decoded {
            match d {
                ONE | FOUR | SEVEN | EIGHT => tally += 1,
                _ => (),
            }
        }
    }

    println!("got {} 1/4/7/8s", tally);

    let mut sum = 0;
    for group in &groups {
        sum += group.output_as_number().unwrap();
    }

    println!("got {} sum of outputs", sum);
}
