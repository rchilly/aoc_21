use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

pub const ZERO: &str = "abcefg";
pub const ONE: &str = "cf";
pub const TWO: &str = "acdeg";
pub const THREE: &str = "acdfg";
pub const FOUR: &str = "bcdf";
pub const FIVE: &str = "abdfg";
pub const SIX: &str = "abdefg";
pub const SEVEN: &str = "acf";
pub const EIGHT: &str = "abcdefg";
pub const NINE: &str = "abcdfg";

fn contains_chars_of(s: &str, of: &str) -> bool {
    for c in of.chars() {
        if !s.contains(c) {
            return false;
        }
    }

    true
}

pub struct SignalGroup {
    zero_to_nine: HashMap<&'static str, String>,
    output: Vec<String>,
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

impl SignalGroup {
    // This function only accessible indirectly via the FromStr
    // trait implementation, for simplicity.
    fn new(mut digits: Vec<String>, output: Vec<String>) -> Result<SignalGroup, String> {
        let mut zero_to_nine = HashMap::new();

        for unique in [ONE, FOUR, SEVEN, EIGHT] {
            let digit = digits
                .iter()
                .find(|d| d.len() == unique.len())
                .ok_or("unique length digit not found")?;

            zero_to_nine.insert(unique, digit.clone());
        }

        let three = digits
            .iter()
            .find(|d| {
                let seven = zero_to_nine.get(SEVEN).unwrap();
                d.len() == THREE.len() && contains_chars_of(d, seven)
            })
            .ok_or("three not found")?;

        let nine = digits
            .iter()
            .find(|d| d.len() == NINE.len() && contains_chars_of(d, three))
            .ok_or("nine not found")?;

        zero_to_nine.insert(THREE, three.clone());
        zero_to_nine.insert(NINE, nine.clone());

        // Remove 1, 3, 4, 7, 8, 9, so that only 0, 2, 5 and 6 remain.
        digits.retain(|d| !zero_to_nine.values().any(|v| v == d));

        let zero = digits
            .iter()
            .find(|d| {
                let seven = zero_to_nine.get(SEVEN).unwrap();
                d.len() == ZERO.len() && contains_chars_of(d, seven)
            })
            .ok_or("zero not found")?;

        let five = digits
            .iter()
            .find(|d| {
                let nine = zero_to_nine.get(NINE).unwrap();
                d.len() == FIVE.len() && contains_chars_of(nine, d)
            })
            .ok_or("five not found")?;

        let two = digits
            .iter()
            .find(|d| d.len() == TWO.len() && *d != five)
            .ok_or("two not found")?;

        let six = digits
            .iter()
            .find(|d| d.len() == SIX.len() && contains_chars_of(d, five))
            .ok_or("six not found")?;

        zero_to_nine.insert(ZERO, zero.clone());
        zero_to_nine.insert(TWO, two.clone());
        zero_to_nine.insert(FIVE, five.clone());
        zero_to_nine.insert(SIX, six.clone());

        Ok(SignalGroup {
            zero_to_nine,
            output,
        })
    }

    pub fn output_as_digits(&self) -> [&str; 4] {
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

    pub fn output_as_number(&self) -> Result<usize, ParseIntError> {
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
