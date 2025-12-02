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
    let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
    let mut part1 = 0;
    let mut part2 = 0;
    for (low, high) in intervals {
        let mut seed = low;
        for target_digits in digit_count(low)..digit_count(high) + 1 {
            let mut is_first_num_splits = true;
            for &num_repeats in primes.iter().take_while(|x| **x <= target_digits) {
                if target_digits % (num_repeats) != 0 {
                    continue;
                }
                let sequence_digits = target_digits / num_repeats;
                let sequence_max = pow10(sequence_digits);
                let mut sequence = seed / pow10(target_digits - sequence_digits);

                if repeat_num(sequence, num_repeats) < low {
                    sequence += 1;
                };
                let danger_val = get_danger_val(sequence_digits);
                let mut repeated = repeat_num(sequence, num_repeats);
                while sequence < sequence_max && repeated <= high {
                    // If spltting with primes, the only case that can be double counted are multiples of 1, 11, 111... etc.
                    // But we do want these values counted once, so check if this is the first
                    if sequence % danger_val != 0 || is_first_num_splits {
                        part2 += repeated;
                    }
                    if num_repeats == 2 {
                        part1 += repeated;
                    }
                    sequence += 1;
                    repeated = repeat_num(sequence, num_repeats);
                }
                is_first_num_splits = false;
            }
            seed = pow10(target_digits);
        }
    }

    (part1, part2)
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
fn get_danger_val(num_digits: u32) -> Int {
    let mut result = 0;
    for d in 0..num_digits {
        result += pow10(d);
    }
    result
}
fn repeat_num(num: Int, num_times: u32) -> Int {
    let digits = digit_count(num);
    let mut repeated = 0;
    for i in 0..num_times {
        repeated += num * pow10(digits * i);
    }
    repeated
}

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
    fn test_example_part2() {
        let (_part1, part2) = calculate_answers(read_input("test").unwrap());
        assert_eq!(part2, 4174379265);
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
        assert_eq!(repeat_num(100, 2), 100100);
        assert_eq!(repeat_num(1000, 2), 10001000);
        assert_eq!(repeat_num(1234, 2), 12341234);
        assert_eq!(repeat_num(1234, 4), 1234123412341234)
    }
}
