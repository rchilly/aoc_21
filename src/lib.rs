use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_input(day: u8) -> std::io::Result<Vec<String>> {
    let fmt = format!("data/{}.txt", day);
    let path = Path::new(&fmt);

    let input = File::open(&path)?;

    let mut lines: Vec<String> = Vec::new();

    for l in io::BufReader::new(input).lines() {
        let line = l?;
        lines.push(line);
    }

    Ok(lines)
}
