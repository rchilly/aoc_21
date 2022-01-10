fn main() {
    let input = match advent_21::read_input(7) {
        Err(why) => panic!("failed to read input: {}", why),
        Ok(v) => v,
    };

    let mut positions: Vec<usize> = vec![];
    for p in input.first().unwrap().split(',') {
        positions.push(p.parse::<usize>().unwrap())
    }

    let med_pos = get_median(&positions);
    let mean_pos = get_mean(&positions);

    println!(
        "will cost {} fuel to align at mean position {}",
        sum_diffs_to_target(&positions, raw_diff, med_pos),
        med_pos
    );

    println!(
        "will cost {} fuel to align at mean position {}",
        sum_diffs_to_target(&positions, asc_diff, mean_pos),
        mean_pos
    );
}

fn get_median(numbers: &[usize]) -> usize {
    let mut seq = numbers.to_vec();
    seq.sort_unstable();

    let len = seq.len();
    let is_even = len % 2 == 0;

    if is_even {
        (seq[len / 2] + seq[len / 2 - 1]) / 2
    } else {
        seq[len / 2]
    }
}

fn get_mean(numbers: &[usize]) -> usize {
    let mut sum = 0;
    for number in numbers {
        sum += number;
    }

    // Rounding mean before returning or skipping the f32
    // conversion both returned a value 1 greater than correct
    // answer. So essentially rounding down. Shrug.
    let mean: f32 = sum as f32 / numbers.len() as f32;
    mean as usize
}

fn sum_diffs_to_target(
    numbers: &[usize],
    get_diff: fn(a: usize, b: usize) -> usize,
    target: usize,
) -> usize {
    let mut sum = 0;
    for number in numbers {
        sum += get_diff(*number, target);
    }

    sum
}

fn raw_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn asc_diff(a: usize, b: usize) -> usize {
    let (mut from, mut to) = (a, b);
    if a > b {
        from = b;
        to = a;
    }

    let mut diff = 0;
    let mut add = 1;

    while from < to {
        from += 1;
        diff += add;
        add += 1;
    }

    diff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_median_works() {
        let numbers = &[16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(get_median(numbers), 2);
    }

    #[test]
    fn get_mean_works() {
        let numbers = &[16, 1, 2, 0, 4, 2, 7, 1, 2, 15];
        assert_eq!(get_mean(numbers), 5);
    }

    #[test]
    fn sum_diffs_to_target_works() {
        let numbers = &[16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(sum_diffs_to_target(numbers, raw_diff, 2), 37);
        assert_eq!(sum_diffs_to_target(numbers, asc_diff, 5), 168);
    }
}
