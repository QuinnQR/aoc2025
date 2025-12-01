use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::ops::Rem;
use std::path::Path;

fn main() {
    let path = Path::new("input");
    let (part1, part2) = get_result(get_line_iterator(path).unwrap());
    println!("Day\nPart 1: {part1}\nPart 2: {part2}\n")
}

fn get_result(lines: Lines<BufReader<File>>) -> (i32, i32) {
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;
    let mut dial_position: i32 = 50;
    for line in lines.map_while(Result::ok) {
        if line.len() == 0 {
            break
        };
        let mut diff = 0;
        if line[0..1] == *"L" {
            diff = -line[1..line.len()].parse::<i32>().unwrap();
        } else if line[0..1] == *"R" {
            diff = line[1..line.len()].parse::<i32>().unwrap();
        }
        dial_position += diff;
        let normalised_diff = (diff + 50 - dial_position.rem_euclid(100)).abs();
        part2 += (normalised_diff + 49) / 100;
        if dial_position % 100 == 0 {
            part1 += 1;
            if diff < 0 {
                part2 += 1;
            }
        }
    }
    (part1, part2)
}

fn get_line_iterator<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename.as_ref())?;
    Ok(io::BufReader::new(file).lines())
}
#[cfg(test)]
mod tests {
    use super::{get_line_iterator, get_result};
    use std::path::Path;
    #[test]
    pub fn test_example() {
        let path = Path::new("test");
        let (part1, part2) = get_result(get_line_iterator(path).unwrap());
        assert_eq!(part1, 3, "Part 1 should be 3 for the test data");
        assert_eq!(part2, 6, "Part 2 should be 6 for the test data");
    }
}
