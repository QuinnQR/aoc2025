use std::{error::Error, path::Path};
type Int = i64;
pub fn main() {
    let start = std::time::Instant::now();
    let input = read_input("input").unwrap();
    let parsed = start.elapsed();
    let (part1, part2) = calculate_answers(input);
    let elapsed = start.elapsed();
    println!(
        "\tDay 3 Unsafe\nPart 1: {}\nPart 2: {}\nTime:   {}\nIO:     {}\nCalc:   {}",
        part1,
        part2,
        elapsed.as_micros(),
        parsed.as_micros(),
        (elapsed - parsed).as_micros()
    );
}
fn calculate_answers(raw_data: Vec<u8>) -> (Int, Int) {
    raw_data
        .as_chunks::<101>()
        .0
        .into_iter()
        .map(get_line_result::<101>)
        .reduce(|lhs, rhs| (lhs.0 + rhs.0, lhs.1 + rhs.1))
        .unwrap()
}

fn get_line_result<const LINE_SIZE: usize>(line: &[u8; LINE_SIZE]) -> (Int, Int) {
    let mut part_two_digits: [Int; 12] = [0; 12];
    let mut part_one_digits: [Int; 2] = [0; 2];
    // Stores the digit of the best
    for (next_digit_idx, next_digit_char) in line.into_iter().rev().skip(1).rev().enumerate() {
        let next_digit_value = *next_digit_char as Int - '0' as Int;
        // If we're close to the end, don't replace early digits, start from this idx:
        let start_index_for_replace = 11 - (line.len() - next_digit_idx - 2).min(11);
        for index in start_index_for_replace..12 {
            if next_digit_value > part_two_digits[index] {
                part_two_digits[index] = next_digit_value;
                part_two_digits[(index + 1)..12].fill(0);
                break;
            }
        }
        // If at the end of a line, don't replace the first digit for part 1.
        if next_digit_value > part_one_digits[0] && next_digit_idx != line.len() - 2 {
            part_one_digits[0] = next_digit_value;
            part_one_digits[1] = 0;
        } else if next_digit_value > part_one_digits[1] {
            part_one_digits[1] = next_digit_value;
        }
    }
    // Convert digit arrays into an integer
    return (
        part_one_digits.into_iter().reduce(|lhs, rhs| lhs * 10 + rhs).unwrap(),
        part_two_digits.into_iter().reduce(|lhs, rhs| lhs * 10 + rhs).unwrap(),
    );
}

fn read_input<P>(filename: P) -> Result<Vec<u8>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    Ok(std::fs::read_to_string(filename)?.into_bytes())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_alt() {
        let test_iterator = read_input("test").unwrap();
        let (part1, part2) = test_iterator
            .as_chunks::<16>()
            .0
            .into_iter()
            .map(get_line_result::<16>)
            .reduce(|lhs, rhs| (lhs.0 + rhs.0, lhs.1 + rhs.1))
            .unwrap();
        assert_eq!(part1, 357);
        assert_eq!(part2, 3121910778619);
    }
}
