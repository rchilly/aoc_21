use std::str::FromStr;
use std::fmt;

#[derive(Debug)]
struct CommandParseError(String);

impl fmt::Display for CommandParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Forward,
}

#[derive(Debug)]
struct Command {
    dir: Direction,
    delta: u32,
}

impl Command {
    fn transform(&self, p: Position) -> Position {
        match self.dir {
            Direction::Up => Position{
                x: p.x, 
                y: p.y, 
                aim: p.aim - self.delta,
            },
            Direction::Down => Position{
                x: p.x, 
                y: p.y, 
                aim: p.aim + self.delta
            },
            Direction::Forward => Position{
                x: p.x + self.delta, 
                y: p.y + self.delta * p.aim, 
                aim: p.aim
            },
        }
    }
}

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut strs = ("", "");

        for (n, s) in s.split_ascii_whitespace().enumerate() {
            match n {
                0 => strs.0 = s,
                1 => strs.1 = s,
                _ => return Err(CommandParseError(format!("too many whitespace delimited substrings"))),
            }
        }

        let dir = strs.0.parse::<Direction>()?;
        let delta = strs.1.parse::<u32>()
            .map_err(|_| CommandParseError(String::from("invalid delta")))?;


        Ok(Command{dir, delta})
    }
}

impl FromStr for Direction {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            "forward" => Ok(Direction::Forward),
            _ => Err(CommandParseError(String::from("invalid direction"))),
        }
    }
}

#[derive(Debug, Default)]
struct Position {
    x: u32,
    y: u32,
    aim: u32,
}

fn main() {
    let input = match advent_21::read_input(2) {
        Err(why) => panic!("failed to read input: {}", why),
        Ok(v) => v,
    };

    let mut position = Position::default();
    
    for line in input.into_iter() {
        // This calls to (&str).parse() invokes Command's implementation
        // of the FromStr trait under the hood.
        //
        // Could also write as Command::from_str(&line).
        let cmd = match line.parse::<Command>() {
            Err(why) => panic!("couldn't parse string '{}' as command: {}", line, why),
            Ok(c) => c,
        };

        position = cmd.transform(position);
    }

    println!("final position is {:?}", position);
}