// All numbers in input are 12-bit.
// We'll use closest size u16 to hold their values.
const BITS: usize = 12;

fn main() {
    let input = match advent_21::read_input(3) {
        Err(why) => panic!("failed to read input: {}", why),
        Ok(v) => v,
    };

    let mut numbers: Vec<u16> = Vec::new();

    for line in input.into_iter() {
        match u16::from_str_radix(&line, 2) {
            Err(_) => panic!("couldn't convert line '{}' to u16", line),
            Ok(number) => numbers.push(number),
        };
    }

    println!(
        "power consumption: {:?}",
        calculate_power_consumption(&numbers, true).unwrap_or_default()
    );

    println!(
        "life support rating: {:?}",
        calculate_life_support_rating(&numbers, true).unwrap_or_default()
    );
}

fn calculate_power_consumption(numbers: &[u16], debug: bool) -> Result<usize, String> {
    let gamma_rate = find_most_common_bits(numbers)?;

    // Must remove the leading 1111's in positions 16-13
    // after the NOT, since we just want a 12-bit number.
    let epsilon_rate = !gamma_rate ^ (0b1111 << BITS);

    // The 012 chunk means make the {:b} display width 12
    // and pad with 0's if needed. Note reuse of positional
    // arguments.
    if debug {
        println!(
            "gamma = {0} ({0:012b}), epsilon = {1} ({1:012b})",
            gamma_rate, epsilon_rate,
        );
    }

    Ok(epsilon_rate as usize * gamma_rate as usize)
}

// Tries to find the most common bits at each index of the numbers
// in numbers and reports them as the single number they represent
// in sequence. A tie at any index is an error.
fn find_most_common_bits(numbers: &[u16]) -> Result<u16, String> {
    let mut tallies = [0i32; BITS];

    for number in numbers {
        // AND each bit in each 12-bit number to check
        // if it's set and adjust the tally for that
        // bit index accordingly.
        for (i, tally) in tallies.iter_mut().enumerate() {
            let bit_index = 0b1 << (BITS - (i + 1));
            match number & bit_index {
                0 => *tally -= 1,
                _ => *tally += 1,
            }
        }
    }

    let mut most_common_str = String::new();
    for (i, t) in tallies.iter().enumerate() {
        if *t == 0 {
            return Err(format!(
                "got tie, expected clear winner between 0 vs. 1 for bit position {}",
                i
            ));
        }

        if *t > 0 {
            most_common_str.push('1');
        } else {
            most_common_str.push('0');
        }
    }

    u16::from_str_radix(&most_common_str, 2).map_err(|e| e.to_string())
}

fn calculate_life_support_rating(numbers: &[u16], debug: bool) -> Result<usize, String> {
    // I love that ? works here. Thanks to the implementation of Into<usize> for u16.
    let o2_gen_rate = find_one_by_bit_criteria(numbers, BitCriteria::MostCommonOrOne)?;
    let co2_scrub_rate = find_one_by_bit_criteria(numbers, BitCriteria::LeastCommonOrZero)?;

    if debug {
        println!(
            "o2 = {0} ({0:012b}), co2 = {1} ({1:012b})",
            o2_gen_rate, co2_scrub_rate,
        );
    }

    Ok(o2_gen_rate as usize * co2_scrub_rate as usize)
}

// Criteria for filtering down a list of numbers by evaluating
// one bit index at a time.
enum BitCriteria {
    // Keep all numbers with most common bit at index, or with
    // value 1 in case of tie.
    MostCommonOrOne,
    // Keep all numbers with least common bit at index, or with
    // value 0 in case of tie.
    LeastCommonOrZero,
}

fn find_one_by_bit_criteria(numbers: &[u16], critera: BitCriteria) -> Result<u16, String> {
    let mut copy = numbers.to_vec();

    for n in 0..BITS {
        if copy.len() == 1 {
            break;
        }

        let mut tally = 0;
        let bit_index = 0b1 << (BITS - 1 - n);
        for number in copy.iter() {
            match number & bit_index {
                0 => tally -= 1,
                _ => tally += 1,
            }
        }

        let keep_zero = match critera {
            BitCriteria::LeastCommonOrZero => tally >= 0,
            BitCriteria::MostCommonOrOne => tally < 0,
        };

        if keep_zero {
            copy.retain(|number: &u16| (number & bit_index) == 0);
        } else {
            copy.retain(|number: &u16| (number & bit_index) > 0);
        }
    }

    if copy.len() != 1 {
        return Err(format! {"{} numbers remained after filtering by criteria", copy.len()});
    }

    Ok(copy.pop().unwrap())
}
