use shared;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Lines};
use std::path::Path;

fn main() {
    let input_data: Vec<i32> = match read_input("input") {
        Err(err) => {
            println!("Error reading day 1 input: {}", err.to_string());
            return;
        }
        Ok(parsed_input) => parsed_input,
    };
    let (part1, part2) = calculate_answers(input_data);
    println!("\tDay 1\nPart 1: {}\nPart 2: {}", part1, part2);
}
fn calculate_answers(moves: Vec<i32>) -> (i32, i32) {
    // moves[i] < 0 is a left rotation, moves[i] > 0 is a right rotation
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;
    let mut dial_position: i32 = 50;

    for dial_move in moves.into_iter() {
        dial_position += dial_move;

        // Move dial_position to 50 (without passing a 0, in general case) and calculate the new dial_move
        // As this normalised_move is centred at 50, its easy to work out how many 0s are crossed
        let normalised_move = (dial_move + 50 - dial_position.rem_euclid(100)).abs();
        part2 += (normalised_move + 49) / 100;

        if dial_position % 100 == 0 {
            part1 += 1;
            part2 += 1;
            // If the dial is at 0, it moves right to 50. If the original position was to the left then
            // this adds an extra crossing of 0. to account for this, subtract 1 if the dial turned right
            if dial_move > 0 {
                part2 -= 1;
            }
        }
    }
    (part1, part2)
}

fn read_input<P>(filename: P) -> Result<Vec<i32>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    Ok(parse_lines(shared::get_lines_from_file(filename)?)?)
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
