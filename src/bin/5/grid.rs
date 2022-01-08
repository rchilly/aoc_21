use std::fmt::{Display, Write};
use std::str::FromStr;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut x_str, mut y_str) = ("", "");

        for (n, coord) in s.split(',').enumerate() {
            match n {
                0 => x_str = coord,
                1 => y_str = coord,
                _ => {
                    return Err(
                        "got more than comma-delimited pair of coordinate strings".to_string()
                    )
                }
            }
        }

        let x = x_str.parse::<usize>();
        let y = y_str.parse::<usize>();
        if x.is_err() || y.is_err() {
            return Err("comma-delimited strings not numbers".to_string());
        }

        Ok(Point {
            x: x.unwrap(),
            y: y.unwrap(),
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Line {
    start: Point,
    end: Point,
    slope: Slope,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Slope {
    Vertical,
    Horizontal,
    DiagonalPos,
    DiagonalNeg,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Result<Line, String> {
        // If we're dealing with a left-running horizontal line or
        // down-running vertical line, normalize them in the opposite
        // direction for convenience of plotting points along them.
        let (mut _start, mut _end) = (start, end);
        if start.x > end.x || (start.x == end.x && start.y > end.y) {
            _start = end;
            _end = start;
        }

        let slope: Slope;
        if _start.x == _end.x {
            slope = Slope::Vertical;
        } else if _start.y == _end.y {
            slope = Slope::Horizontal;
        } else {
            match (_end.y as isize - _start.y as isize) / (_end.x as isize - _start.x as isize) {
                1 => slope = Slope::DiagonalPos,
                -1 => slope = Slope::DiagonalNeg,
                m => {
                    return Err(format!(
                        "got slope value {} for diagonal line, only +/- 1 accepted",
                        m
                    ))
                }
            }
        }

        Ok(Line {
            start: _start,
            end: _end,
            slope,
        })
    }

    pub fn slope(&self) -> Slope {
        self.slope
    }

    pub fn points(&self) -> Vec<Point> {
        let mut points = vec![];
        let mut current = self.start;

        while current != self.end {
            points.push(current);

            match self.slope {
                Slope::Vertical => current.y += 1,
                Slope::Horizontal => current.x += 1,
                Slope::DiagonalNeg => {
                    current.x += 1;
                    current.y -= 1;
                }
                Slope::DiagonalPos => {
                    current.x += 1;
                    current.y += 1;
                }
            }
        }

        points.push(self.end);

        points
    }
}

pub struct Grid {
    rows: Vec<Vec<usize>>,
    w: usize,
    h: usize,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for line in self.rows.iter().rev() {
            for point in line {
                write!(s, "{:<3}", point).unwrap();
            }

            writeln!(s).unwrap();
        }

        f.write_str(&s)
    }
}

impl Grid {
    pub fn new(w: usize, h: usize) -> Grid {
        let mut rows = Vec::with_capacity(h);
        for _ in 0..h {
            rows.push(vec![0; w]);
        }

        Grid { rows, w, h }
    }

    pub fn add(&mut self, l: Line) -> Result<(), String> {
        if l.end.x >= self.w {
            return Err(format!("line {:?} overflows grid width", l));
        }

        if l.start.y >= self.h || l.end.y >= self.h {
            return Err(format!("line {:?} overflows grid height", l));
        }

        for p in l.points() {
            self.rows[p.y][p.x] += 1;
        }

        Ok(())
    }

    pub fn intersections(&self) -> usize {
        let mut count = 0;
        for row in &self.rows {
            for point in row {
                if *point > 1 {
                    count += 1
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal_works() {
        let (start, end) = (Point { x: 2, y: 3 }, Point { x: 7, y: 3 });
        let line = Line::new(start, end).unwrap();
        assert_eq!(line.start, start);
        assert_eq!(line.end, end);
        assert_eq!(line.slope, Slope::Horizontal);
        assert_eq!(
            line.points(),
            vec![
                Point { x: 2, y: 3 },
                Point { x: 3, y: 3 },
                Point { x: 4, y: 3 },
                Point { x: 5, y: 3 },
                Point { x: 6, y: 3 },
                Point { x: 7, y: 3 },
            ]
        )
    }

    #[test]
    fn point_swap_works() {
        let (start, end) = (Point { x: 2, y: 3 }, Point { x: 7, y: 3 });
        let line = Line::new(end, start).unwrap(); // Flipped.
        assert_eq!(line.start, start);
        assert_eq!(line.end, end);
        assert_eq!(line.slope, Slope::Horizontal);
        assert_eq!(
            line.points(),
            vec![
                Point { x: 2, y: 3 },
                Point { x: 3, y: 3 },
                Point { x: 4, y: 3 },
                Point { x: 5, y: 3 },
                Point { x: 6, y: 3 },
                Point { x: 7, y: 3 },
            ]
        )
    }

    #[test]
    fn vertical_works() {
        let (start, end) = (Point { x: 2, y: 3 }, Point { x: 2, y: 10 });
        let line = Line::new(start, end).unwrap();
        assert_eq!(line.start, start);
        assert_eq!(line.end, end);
        assert_eq!(line.slope, Slope::Vertical);
        assert_eq!(
            line.points(),
            vec![
                Point { x: 2, y: 3 },
                Point { x: 2, y: 4 },
                Point { x: 2, y: 5 },
                Point { x: 2, y: 6 },
                Point { x: 2, y: 7 },
                Point { x: 2, y: 8 },
                Point { x: 2, y: 9 },
                Point { x: 2, y: 10 },
            ]
        )
    }

    #[test]
    fn diagonal_pos_works() {
        let (start, end) = (Point { x: 2, y: 3 }, Point { x: 6, y: 7 });
        let line = Line::new(start, end).unwrap();
        assert_eq!(line.start, start);
        assert_eq!(line.end, end);
        assert_eq!(line.slope, Slope::DiagonalPos);
        assert_eq!(
            line.points(),
            vec![
                Point { x: 2, y: 3 },
                Point { x: 3, y: 4 },
                Point { x: 4, y: 5 },
                Point { x: 5, y: 6 },
                Point { x: 6, y: 7 },
            ]
        )
    }

    #[test]
    fn diagonal_neg_works() {
        let (start, end) = (Point { x: 8, y: 10 }, Point { x: 10, y: 8 });
        let line = Line::new(start, end).unwrap();
        assert_eq!(line.start, start);
        assert_eq!(line.end, end);
        assert_eq!(line.slope, Slope::DiagonalNeg);
        assert_eq!(
            line.points(),
            vec![
                Point { x: 8, y: 10 },
                Point { x: 9, y: 9 },
                Point { x: 10, y: 8 },
            ]
        )
    }

    #[test]
    #[should_panic]
    fn other_slopes_panic() {
        let (start, end) = (Point { x: 10, y: 10 }, Point { x: 11, y: 12 });
        let _ = Line::new(start, end).unwrap();
    }

    #[test]
    fn intersections_works() {
        /*
        x x x X X
        . . . X .
        . x X X .
        . x . x .
        x . . x .
        */

        let l1 = Line::new(Point { x: 0, y: 0 }, Point { x: 4, y: 4 }).unwrap();
        let l2 = Line::new(Point { x: 1, y: 2 }, Point { x: 3, y: 2 }).unwrap();
        let l3 = Line::new(Point { x: 3, y: 4 }, Point { x: 3, y: 0 }).unwrap();
        let l4 = Line::new(Point { x: 0, y: 4 }, Point { x: 4, y: 4 }).unwrap();

        let mut grid: Grid = Grid::new(5, 5);
        grid.add(l1).unwrap();
        grid.add(l2).unwrap();
        grid.add(l3).unwrap();
        grid.add(l4).unwrap();

        println!("{}", grid);

        assert_eq!(grid.intersections(), 5);
    }
}
