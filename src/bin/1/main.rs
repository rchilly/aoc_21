fn main() {
    let input = match advent_21::read_input(1) {
        Err(why) => panic!("failed to read input: {}", why),
        Ok(v) => v,
    };

    let mut depths: Vec<usize> = Vec::new();

    // Implicitly calls .into_iter() on the vector, which moves i.e.
    // consumes it. Any reference to it after here won't compile.
    for line in input.into_iter() {
        match line.parse::<usize>() {
            Err(why) => panic!("couldn't convert line '{}' to number: {}", line, why),
            Ok(number) => depths.push(number),
        }
    }

    println!("got {} depth measurements from file", depths.len());
    println!(
        "got {} depth measurements greater than the ones before them",
        count_increasing_numbers(&depths)
    );

    let windows = get_rolling_windows(&depths, 3);
    println!("got {} depth windows", windows.len());
    println!(
        "got {} depth windows greater than the ones before them",
        count_increasing_numbers(&windows)
    );
}

// Iterates through numbers, counting each one that's greater than the
// one right before it.
fn count_increasing_numbers(numbers: &[usize]) -> usize {
    let length = numbers.len();
    if length == 0 {
        return 0;
    }

    // Iterator has to be mutable to call next(), which borrows a mutable reference to
    // itself and changes the state of the underlying struct.
    //
    // Vectors provide the .into_iter() method to iterate by moving i.e. taking ownership
    // of each element in the underlying collection, thus consuming it. But here we're
    // dealing with a slice, which is just a reference to a collection, so we can't move
    // the collection it references.
    let mut numbers_iter = numbers.iter();

    // Manually take first element so we don't need a special condition in the loop
    // below for skipping the compare with no previous value on hand.
    let mut previous = match numbers_iter.next() {
        Some(number) => number,
        None => panic!("first element was none, expected some"),
    };

    let mut count: usize = 0;

    for number in numbers_iter {
        if number > previous {
            count += 1;
        }

        previous = number;
    }

    count
}

struct Window {
    count: usize,
    sum: usize,
}

// Iterates through numbers, grouping them into rolling windows of the provided size,
// and returns a new collection of those windows as the sums of their component numbers.
fn get_rolling_windows(numbers: &[usize], window_size: usize) -> Vec<usize> {
    let length = numbers.len();
    if length == 0 {
        return vec![];
    }

    let mut windows: Vec<usize> = vec![];
    let mut accs: Vec<Window> = Vec::with_capacity(window_size);

    // Same as .iter() since this is a slice.
    //
    // Use as many accumulators as the window size to keep running
    // sums as we iterate. We push those sums to the return value
    // as windows fill up.
    for number in numbers {
        if accs.len() < window_size {
            accs.push(Window { count: 0, sum: 0 });
        }

        for acc in accs.iter_mut() {
            (*acc).count += 1;
            (*acc).sum += number;

            if acc.count == window_size {
                windows.push(acc.sum);
                (*acc).count = 0;
                (*acc).sum = 0;
            }
        }
    }

    println!("collected {} windows", windows.len());
    println!("  window 1: {}", windows[0]);
    println!("  window 2: {}", windows[1]);
    println!("  ...");
    println!(
        "  window {}: {}",
        windows.len() - 1,
        windows[windows.len() - 2]
    );
    println!("  window {}: {}", windows.len(), windows[windows.len() - 1]);

    windows
}
