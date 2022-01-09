use std::collections::HashMap;
use std::option::Option;

const MAX_DAYS: usize = 8;

#[derive(Copy, Clone)]
struct Fish(u8);

impl Fish {
    // Age one day and possibly spawn a new fish.
    fn age(&mut self) -> Option<Fish> {
        if self.0 == 0 {
            self.0 = 6;
            Some(Fish(8))
        } else {
            self.0 -= 1;
            None
        }
    }
}

struct School(HashMap<usize, usize>);

impl School {
    pub fn new(fish: &[Fish]) -> School {
        let mut school = HashMap::with_capacity(MAX_DAYS + 1);

        for f in fish {
            school
                .entry(f.0 as usize)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        School(school)
    }

    pub fn age(&mut self) {
        let at_zero = *self.0.get(&0).unwrap_or(&0);

        for day in 1..=MAX_DAYS {
            let at_day = *self.0.get(&day).unwrap_or(&0);
            self.0.insert(day - 1, at_day);
        }

        self.0.insert(MAX_DAYS, at_zero);
        self.0
            .entry(MAX_DAYS - 2)
            .and_modify(|count| *count += at_zero)
            .or_insert(at_zero);
    }

    pub fn population(&self) -> usize {
        let mut count = 0;
        for n in self.0.values() {
            count += *n;
        }
        count
    }
}

fn main() {
    let input = match advent_21::read_input(6) {
        Err(why) => panic!("failed to read input: {}", why),
        Ok(v) => v,
    };

    let mut fish: Vec<Fish> = vec![];
    let ages = input.first().unwrap();
    for a in ages.split(',') {
        fish.push(Fish(a.parse::<u8>().unwrap()))
    }

    println!(
        "after {} days, there are {} fish",
        80,
        count_fish_after_days(&fish, 80),
    );

    println!(
        "after {} days, there are {} fish",
        256,
        count_school_after_days(&fish, 256),
    );
}

fn count_fish_after_days(fish: &[Fish], days: usize) -> usize {
    let mut all_fish = fish.to_vec();
    let mut new_fish: Vec<Fish> = vec![];
    for _ in 0..days {
        new_fish.clear();

        for f in all_fish.iter_mut() {
            if let Some(new) = f.age() {
                new_fish.push(new);
            }
        }

        all_fish.extend_from_slice(&new_fish);
    }

    all_fish.len()
}

fn count_school_after_days(fish: &[Fish], days: usize) -> usize {
    let mut school = School::new(fish);
    for _ in 0..days {
        school.age();
    }

    school.population()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn age_school_works() {
        let mut school = School::new(&[Fish(5), Fish(2), Fish(2), Fish(1), Fish(0)]);

        assert_eq!(school.0[&5], 1);
        assert_eq!(school.0[&2], 2);
        assert_eq!(school.0[&1], 1);
        assert_eq!(school.0[&0], 1);
        assert_eq!(school.population(), 5);

        school.age();

        assert_eq!(school.0[&8], 1);
        assert_eq!(school.0[&6], 1);
        assert_eq!(school.0[&4], 1);
        assert_eq!(school.0[&1], 2);
        assert_eq!(school.0[&0], 1);
        assert_eq!(school.population(), 6);

        school.age();

        assert_eq!(school.0[&8], 1);
        assert_eq!(school.0[&7], 1);
        assert_eq!(school.0[&6], 1);
        assert_eq!(school.0[&5], 1);
        assert_eq!(school.0[&3], 1);
        assert_eq!(school.0[&0], 2);
        assert_eq!(school.population(), 7);

        school.age();

        assert_eq!(school.0[&8], 2);
        assert_eq!(school.0[&7], 1);
        assert_eq!(school.0[&6], 3);
        assert_eq!(school.0[&5], 1);
        assert_eq!(school.0[&4], 1);
        assert_eq!(school.0[&2], 1);
        assert_eq!(school.population(), 9);
    }
}
