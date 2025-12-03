use shared::get_lines_from_file;
use std::{error::Error, path::Path};
type Int = i64;
fn main() {
    let start = std::time::Instant::now();
    let line_iterator = match read_input("input") {
        Err(error) => {
            println!("Error occured reading day 3 input: {}", error.to_string());
            return;
        }
        Ok(iterator) => iterator,
    };
    let (part1, part2) = calculate_answers(line_iterator);
    let elapsed = start.elapsed();
    println!(
        "\tDay 3\nPart 1: {}\nPart 2: {}\nTime:   {}",
        part1,
        part2,
        elapsed.as_micros()
    );
}
fn calculate_answers(line_iterator: Box<dyn Iterator<Item = String>>) -> (Int, Int) {
    line_iterator
        .map(get_line_result)
        .reduce(|lhs, rhs| (lhs.0 + rhs.0, lhs.1 + rhs.1))
        .unwrap()
}

fn get_line_result(line: String) -> (Int, Int) {
    let mut part_one_digits: [Int; 2] = [0; 2];
    let mut part_two_digits: [Int; 12] = [0; 12];
    // Stores the digit of the best
    for (next_digit_idx, next_digit_char) in line.chars().enumerate() {
        let next_digit_value = next_digit_char as Int - '0' as Int;
        // Part 1:
        if next_digit_value > part_one_digits[0] && next_digit_idx != line.len() - 1 {
            // If at the end of a line, don't replace the first digit for part 1.
            part_one_digits[0] = next_digit_value;
            part_one_digits[1] = 0;
        } else if next_digit_value > part_one_digits[1] {
            part_one_digits[1] = next_digit_value;
        }

        // Part 2:
        // If we're close to the end of the line, don't replace early digits (as there aren't
        // enough digits left to fill the number) To ensure this, only replace digits after this index:
        let first_replacable_index = 11 - (line.len() - next_digit_idx - 1).min(11);
        for index in first_replacable_index..12 {
            if next_digit_value > part_two_digits[index] {
                part_two_digits[index] = next_digit_value;
                // Inserting a number here invalidates the less significant digits, set them to zero
                part_two_digits[(index + 1)..12].fill(0);
                break;
            }
        }
    }
    // Convert digit arrays into integer tuple and return
    (
        part_one_digits.into_iter().reduce(|lhs, rhs| lhs * 10 + rhs).unwrap(),
        part_two_digits.into_iter().reduce(|lhs, rhs| lhs * 10 + rhs).unwrap(),
    )
}

fn read_input<P>(filename: P) -> Result<Box<dyn Iterator<Item = String>>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    Ok(Box::new(
        get_lines_from_file(filename)?.map_while(|x| x.ok().filter(|x| x.len() > 11)),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_input_parse() {
        let mut test_iterator = match read_input("test") {
            Err(error) => panic!("Error occured reading test input: {}", error.to_string()),
            Ok(iterator) => iterator,
        };
        assert_eq!(test_iterator.next(), Some(String::from("987654321111111")));
        assert_eq!(test_iterator.next(), Some(String::from("811111111111119")));
        assert_eq!(test_iterator.next(), Some(String::from("234234234234278")));
        assert_eq!(test_iterator.next(), Some(String::from("818181911112111")));
        assert_eq!(test_iterator.next(), None);
    }
    #[test]
    fn test_part1() {
        let test_iterator = read_input("test").unwrap();
        let (part1, _part2) = calculate_answers(test_iterator);
        assert_eq!(part1, 357);
    }
    #[test]
    fn test_part2() {
        let test_iterator = read_input("test").unwrap();
        let (_part1, part2) = calculate_answers(test_iterator);
        assert_eq!(part2, 3121910778619);
    }
}
