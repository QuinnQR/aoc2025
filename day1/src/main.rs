use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

fn main() {
    let input: Vec<i32> = match read_input("input") {
        Err(err) => {
            println!("Error reading day 1 input: {}", err.to_string());
            return;
        }
        Ok(parsed_input) => parsed_input,
    };
    let (part1, part2) = calculate_answers(input);
    println!("\tDay 1\nPart 1: {}\nPart 2: {}", part1, part2);
}
fn calculate_answers(moves: Vec<i32>) -> (i32, i32) {
    // moves[i] < 0 is a left rotation, moves[i] > 0 is a right rotation
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;
    let mut dial_position: i32 = 50;

    for diff in moves.into_iter() {
        // Move dial_position to 50 (without passing a 0, in general case) and calculate the new diff
        dial_position += diff;

        // As this normalised_diff is centred at 50, its easy to work out how many 0s are crossed
        let normalised_diff = (diff + 50 - dial_position.rem_euclid(100)).abs();
        part2 += (normalised_diff + 49) / 100;

        if dial_position % 100 == 0 {
            part1 += 1;
            // Theres an edge case that occurs if diff > 0 (then prev_dial_pos < dial_pos) and
            // dial pos % 100 == 0. Moving dial to 50 adds an extra crossing of 0. So we check
            // if this edge case has occurred before incrementing here.
            if diff < 0 {
                part2 += 1;
            }
        }
    }
    (part1, part2)
}

fn read_input<P>(filename: P) -> Result<Vec<i32>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    Ok(parse_lines(get_lines_from_file(filename)?)?)
}
fn parse_lines(lines: Lines<BufReader<File>>) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut result: Vec<i32> = Vec::new();
    result.reserve(5000);

    for line in lines.map_while(Result::ok) {
        // Avoid phantom new lines at end of input
        if line.len() == 0 {
            break
        };
        if line.starts_with('L') {
            // Left rotations are negative
            result.push(-line[1..].parse::<i32>()?);
        } else if line.starts_with('R') {
            // Right rotations are positive
            result.push(line[1..].parse::<i32>()?);
        } else {
            let error: Box<dyn Error> =
                String::from("Unexpected characters/format while parsing AoC day 1 input").into();
            return Err(error);
        }
    }
    Ok(result)
}
fn get_lines_from_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename.as_ref())?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_example() {
        let test_input = match read_input("test") {
            Ok(input) => input,
            Err(err) => {
                println!("Error reading test input for day 1:\n{}", err.to_string());
                panic!();
            }
        };
        let (part1, part2) = calculate_answers(test_input);
        assert_eq!(part1, 3, "Part 1 should be 3 for the test data");
        assert_eq!(part2, 6, "Part 2 should be 6 for the test data");
    }
}
