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

    for (lower_bound, upper_bound) in intervals {
        // Used to seed first valid repeatable sequence s.t. rep(sequence) >= LB   - changes if more digits are added
        let mut first_sequence_seed = lower_bound;
        for target_digit_count in digit_count(lower_bound)..digit_count(upper_bound) + 1 {
            // Used to stop duplicate values being added multiple times
            let mut is_first_iteration = true;
            for &repetitions in primes.iter().take_while(|x| **x <= target_digit_count) {
                if target_digit_count % (repetitions) != 0 {
                    continue;
                }
                let invalid_id_sum = find_n_digit_repeats_in_interval(
                    target_digit_count,
                    repetitions,
                    first_sequence_seed,
                    (lower_bound, upper_bound),
                    is_first_iteration,
                );
                part2 += invalid_id_sum;
                if repetitions == 2 {
                    part1 += invalid_id_sum;
                }
                is_first_iteration = false;
            }
            first_sequence_seed = power_of_ten(target_digit_count);
        }
    }

    (part1, part2)
}

fn find_n_digit_repeats_in_interval(
    target_digit_count: u32,
    repetitions: u32,
    first_sequence_seed: Int,
    (lower_bound, upper_bound): (Int, Int),
    is_first_iteration: bool,
) -> Int {
    let mut invalid_id_sum = 0;

    let sequence_digit_count = target_digit_count / repetitions;
    let sequence_max = power_of_ten(sequence_digit_count);
    let potential_duplicate_factor = get_duplicate_factor(sequence_digit_count);

    let mut sequence = first_sequence_seed / power_of_ten(target_digit_count - sequence_digit_count);
    if repeat_num(sequence, repetitions) < lower_bound {
        //First repetition was less than LB, increase by 1 so it's greater than LB
        sequence += 1;
    };
    let mut repeated_sequence = repeat_num(sequence, repetitions);
    while sequence < sequence_max && repeated_sequence <= upper_bound {
        // If spltting with primes, the only case that can be double counted are multiples of 1, 11, 111... etc.
        // But we do want these values counted once, so check if this is the first iteration
        if sequence % potential_duplicate_factor != 0 || is_first_iteration {
            invalid_id_sum += repeated_sequence;
        }
        sequence += 1;
        repeated_sequence = repeat_num(sequence, repetitions);
    }
    invalid_id_sum
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
fn power_of_ten(x: u32) -> Int { Int::pow(10, x) }
fn get_duplicate_factor(num_digits: u32) -> Int { (0..num_digits).map(power_of_ten).sum() }
fn repeat_num(num: Int, repetitions: u32) -> Int {
    let digits = digit_count(num);
    (0..repetitions).map(|r| num * power_of_ten(digits * r)).sum()
}

fn read_input<P>(filename: P) -> Result<Vec<(Int, Int)>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    std::fs::read_to_string(filename)?
        .split(',')
        .map(read_interval)
        .collect()
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
        assert_eq!(digit_count(1), 1);
        assert_eq!(digit_count(9), 1);
        assert_eq!(digit_count(10), 2);
        assert_eq!(digit_count(99), 2);
        assert_eq!(digit_count(100), 3);
        assert_eq!(digit_count(999), 3);
        assert_eq!(digit_count(1000), 4);
        assert_eq!(digit_count(5000), 4);
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
