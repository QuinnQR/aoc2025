use std::error::Error;
use std::num::ParseIntError;
use std::path::Path;

type Int = i64;
fn main() {
    let interval_vec = match read_input("input") {
        Err(error) => {
            println!("Error reading day 2 input: {}", error.to_string());
            return
        }
        Ok(v) => v,
    };
    let (part1, part2) = calculate_answers(interval_vec);
    println!("\tDay 2\nPart 1: {}\nPart 2: {}", part1, part2);
}
fn calculate_answers(intervals: Vec<(Int, Int)>) -> (Int, Int) {
    let mut part1 = 0;
    for (low, high) in intervals {
        let start_digits = digit_count(low);
        let mut seed_digits;
        let mut seed;

        if start_digits % 2 == 0 {
            seed_digits = start_digits / 2;
            seed = low / pow10(seed_digits);
            if repeat_num(seed, seed_digits) < low {
                seed += 1
            };
        } else {
            seed_digits = start_digits / 2 + 1;
            seed = pow10(seed_digits - 1);
        }
        let mut repeated = repeat_num(seed, seed_digits);
        let mut upper_power_10 = pow10(seed_digits);
        while repeated <= high {
            part1 += repeated;
            seed += 1;
            if seed == upper_power_10 {
                upper_power_10 *= 10;
                seed_digits += 1;
            }
            repeated = repeat_num(seed, seed_digits);
        }
    }
    (part1, 0)
}
fn digit_count(x: Int) -> u32 {
    let mut log = 0;
    let mut mult = 1;
    while mult <= x {
        mult *= 10;
        log += 1;
    }
    log
}
fn pow10(x: u32) -> Int {
    let ten: Int = 10;
    ten.pow(x)
}
fn repeat_num(num: Int, digits: u32) -> Int { num + pow10(digits) * num }

fn read_input<P>(filename: P) -> Result<Vec<(Int, Int)>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let input_string = std::fs::read_to_string(filename)?;
    input_string.split(',').map(read_interval).collect()
}
fn read_interval(interval_string: &str) -> Result<(Int, Int), Box<dyn Error>> {
    let interval_vec = interval_string
        .split('-')
        .map(str::trim)
        .map(str::parse::<Int>)
        .collect::<Result<Vec<Int>, ParseIntError>>()?;
    if interval_vec.len() != 2 || interval_vec[0] > interval_vec[1] {
        Err("Invalid interval format".into())
    } else {
        Ok((interval_vec[0], interval_vec[1]))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_parse() {
        match read_input("test") {
            Err(error) => {
                panic!("Error occurred reading test input: {}", error.to_string());
            }
            Ok(vector) => {
                assert_eq!(
                    vector,
                    vec![
                        (11, 22),
                        (95, 115),
                        (998, 1012),
                        (1188511880, 1188511890),
                        (222220, 222224),
                        (1698522, 1698528),
                        (446443, 446449),
                        (38593856, 38593862),
                        (565653, 565659),
                        (824824821, 824824827),
                        (2121212118, 2121212124)
                    ],
                    "Input mismatch with test data"
                );
            }
        };
    }
    #[test]
    fn test_example_part1() {
        let (part1, _part2) = calculate_answers(read_input("test").unwrap());
        assert_eq!(part1, 1227775554);
    }
    #[test]
    fn test_digit_count() {
        assert_eq!(digit_count(10), 2);
        assert_eq!(digit_count(99), 2);
        assert_eq!(digit_count(100), 3);
        assert_eq!(digit_count(999), 3);
        assert_eq!(digit_count(1000), 4);
        assert_eq!(digit_count(9999), 4);
    }
    #[test]
    fn test_repeat_num() {
        assert_eq!(repeat_num(100, digit_count(100)), 100100);
        assert_eq!(repeat_num(1000, digit_count(1000)), 10001000);
        assert_eq!(repeat_num(1234, digit_count(1234)), 12341234)
    }
}
