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

    // True average turns out to be adjacent to the best position.
    // Trying to represent this less hackily than not rounding to
    // calculate that average.

    let mut dev = 2;
    let mut least = usize::MAX;
    let mut pos = usize::MAX;
    loop {
        let mut candidates: Vec<usize> = Vec::with_capacity(dev * 2 + 1);
        for p in mean_pos - dev..=mean_pos + dev {
            let c = sum_diffs_to_target(&positions, asc_diff, p);
            candidates.push(c);
            if c < least {
                least = c;
                pos = p;
            }
        }

        // Confirm least as low point of parabola.
        let len = candidates.len();
        if least < candidates[0] && least < candidates[len - 1] {
            break;
        }

        dev += 1;
    }

    println!(
        "will cost {} fuel to align at mean-ish position {} (mean = {})",
        least, pos, mean_pos,
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

    let mean: f32 = sum as f32 / numbers.len() as f32;
    mean.round() as usize
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
        let numbers = &[16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(get_mean(numbers), 5);
    }

    #[test]
    fn sum_diffs_to_target_works() {
        let numbers = &[16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(sum_diffs_to_target(numbers, raw_diff, 2), 37);
        assert_eq!(sum_diffs_to_target(numbers, asc_diff, 5), 168);
    }
}
