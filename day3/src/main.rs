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
    println!("\tDay 3\nPart 1: {}\nPart 2: {}", part1, part2);
}
fn calculate_answers(line_iterator: Box<dyn Iterator<Item = String>>) -> (Int, Int) {
    let mut part1 = 0;
    let part2 = 0;
    for line in line_iterator {
        let mut biggest_so_far = 0;
        let mut current_second = 0;
        for char in line.chars() {
            let num = char as Int;
            if num > biggest_so_far {
                current_second = -biggest_so_far;
                biggest_so_far = num;
            } else if num > current_second {
                current_second = num;
            }
        }
        part1 += if current_second < 0 {
            10 * (-current_second - ('0' as Int)) + biggest_so_far - ('0' as Int)
        } else {
            10 * (biggest_so_far - ('0' as Int)) + current_second - ('0' as Int)
        }
    }
    (part1, part2)
}
fn read_input<P>(filename: P) -> Result<Box<dyn Iterator<Item = String>>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    Ok(Box::new(
        get_lines_from_file(filename)?.map_while(|x| x.ok().filter(|x| x.len() > 1)),
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
}
