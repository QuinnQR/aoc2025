use shared::get_lines_from_file;
use std::{error::Error, path::Path};
type Int = i64;
fn main() {
    let line_iterator = match read_input("input") {
        Err(error) => {
            println!("Error occured reading day 3 input: {}", error.to_string());
            return;
        }
        Ok(iterator) => iterator,
    };
    let (part1, part2) = calculate_answers(line_iterator);
    println!("\tDay 3\nPart 1:  {}\nPart 2:  {}", part1, part2,);
}
fn calculate_answers(line_iterator: Box<dyn Iterator<Item = String>>) -> (Int, Int) {
    let mut part1: Int = 0;
    let mut part2: Int = 0;

    for line in line_iterator {
        let mut part_two_digits: [Int; 12] = [0; 12];
        let mut part_one_digits: [Int; 2] = [0; 2];
        // Stores the digit of the best
        for (next_digit_idx, next_digit_char) in line.chars().enumerate() {
            let next_digit_value = next_digit_char as Int - '0' as Int;
            // If we're close to the end, don't replace early digits, start from this idx:
            let start_index_for_replace = 11 - (line.len() - next_digit_idx - 1).min(11);
            for index in start_index_for_replace..12 {
                if next_digit_value > part_two_digits[index] {
                    part_two_digits[index] = next_digit_value;
                    part_two_digits[(index + 1)..12].fill(0);
                    break;
                }
            }
            // If at the end of a line, don't replace the first digit for part 1.
            if next_digit_value > part_one_digits[0] && next_digit_idx != line.len() - 1 {
                part_one_digits[0] = next_digit_value;
                part_one_digits[1] = 0;
            } else if next_digit_value > part_one_digits[1] {
                part_one_digits[1] = next_digit_value;
            }
        }
        // Convert digit arrays into an integer
        part1 += part_one_digits.into_iter().reduce(|lhs, rhs| lhs * 10 + rhs).unwrap();
        part2 += part_two_digits.into_iter().reduce(|lhs, rhs| lhs * 10 + rhs).unwrap();
    }
    (part1, part2)
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
