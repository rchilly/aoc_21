mod signals;

use signals::*;
use std::str::FromStr;

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
        let digits = group.output_as_digits();
        for d in digits {
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
