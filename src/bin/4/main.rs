mod bingo;

fn main() {
    let input = match advent_21::read_input(4) {
        Err(why) => panic!("failed to read input: {}", why),
        Ok(v) => v,
    };

    let mut lines = input.into_iter();
    let mut draws: Vec<u8> = vec![];
    if let Some(l) = lines.next() {
        for s in l.split(',') {
            let number = s.parse::<u8>().unwrap();
            draws.push(number);
        }
    }

    if draws.is_empty() {
        panic!("expected comma separated numbers drawn for bingo");
    }

    let mut boards: Vec<bingo::Board> = vec![];
    let mut numbers: Vec<u8> = vec![];
    for line in lines {
        if line.is_empty() && !numbers.is_empty() {
            let board = bingo::Board::new(&numbers).unwrap();
            boards.push(board);
            numbers.clear();
        }

        for n in line.split_ascii_whitespace() {
            let number = n.parse::<u8>().unwrap();
            numbers.push(number);
        }
    }

    if boards.is_empty() {
        panic!("expected one or more deserialized bingo boards");
    }

    let mut winners: Vec<(bingo::Board, u8)> = vec![];
    for drawn in &draws {
        for board in boards.iter_mut() {
            if let bingo::Outcome::Bingo = board.play(*drawn) {
                winners.push((board.clone(), *drawn))
            }
        }

        boards.retain(|b| !b.bingo);
    }

    let (first, drawn) = winners.first().unwrap();
    println!(
        "first winning board scores {} when {} drawn! {}",
        first.score(*drawn),
        drawn,
        first,
    );

    let (last, drawn) = winners.last().unwrap();
    println!(
        "last winning board scores {} when {} drawn! {}",
        last.score(*drawn),
        drawn,
        last,
    );
}
