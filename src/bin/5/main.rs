mod grid;

use grid::{Grid, Line};

fn main() {
    let input = match advent_21::read_input(5) {
        Err(why) => panic!("failed to read input: {}", why),
        Ok(v) => v,
    };

    let mut lines: Vec<grid::Line> = vec![];
    let (mut max_x, mut max_y) = (0, 0);
    for line in input {
        let pairs: Vec<_> = line.split(" -> ").collect();
        assert_eq!(pairs.len(), 2);

        let (start, end) = (
            pairs[0].parse::<grid::Point>().unwrap(),
            pairs[1].parse::<grid::Point>().unwrap(),
        );

        max_x = greater(max_x, greater(start.x, end.x));
        max_y = greater(max_y, greater(start.y, end.y));

        match Line::new(start, end) {
            Ok(l) => lines.push(l),
            Err(why) => println!("failed to construct line: {}", why),
        }
    }

    let mut grid = Grid::new(max_x + 1, max_y + 1);
    for l in lines {
        grid.add(l).unwrap();
    }

    println!("grid has {} intersections", grid.intersections());
}

fn greater(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        b
    }
}
