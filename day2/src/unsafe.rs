use std::path::Path;

type Int = i64;
fn main() {
    let start = std::time::Instant::now();
    let interval_vec = read_input("input");
    let file_parsed = start.elapsed();
    let (part1, part2) = calculate_answers(interval_vec);
    let solved = start.elapsed();
    print!(
        "\tDay 2\nPart 1: {}\nPart 2: {}\nTime calculating:\t\t{}\nTotal (incl reading/parsing):\t{}\n",
        part1,
        part2,
        (solved - file_parsed).as_secs_f64(),
        solved.as_secs_f64(),
    );
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
    let first_sequence = first_sequence_seed / power_of_ten(target_digit_count - sequence_digit_count);

    let difference_between_repeated_sequences = repeat_one_with_leading_zeros(repetitions, sequence_digit_count - 1);
    let mut repeated_sequence = repeat_num(first_sequence, repetitions);
    if repeated_sequence < lower_bound {
        //First repetition was less than LB, increase by 1 so it's greater than LB
        repeated_sequence += difference_between_repeated_sequences;
    };
    let relevant_upper_bound = std::cmp::min(upper_bound + 1, power_of_ten(target_digit_count));
    while repeated_sequence < relevant_upper_bound {
        // If spltting with primes, the only case that can be double counted are multiples of 1, 11, 111... etc.
        // But we do want these values counted once, so check if this is the first iteration
        invalid_id_sum += repeated_sequence;
        repeated_sequence += difference_between_repeated_sequences;
    }
    let potential_duplicate_factor = get_duplicate_factor(target_digit_count);
    if !is_first_iteration {
        let mut duplicate = potential_duplicate_factor;
        while duplicate < lower_bound {
            duplicate += potential_duplicate_factor
        }
        while duplicate <= relevant_upper_bound {
            invalid_id_sum -= duplicate;
            duplicate += potential_duplicate_factor;
        }
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
// As powers of ten are used a lot across the problem, a LUT provides good speedup
const POWER_OF_TEN_LOOKUP: [Int; 13] = [
    Int::pow(10, 0),
    Int::pow(10, 1),
    Int::pow(10, 2),
    Int::pow(10, 3),
    Int::pow(10, 4),
    Int::pow(10, 5),
    Int::pow(10, 6),
    Int::pow(10, 7),
    Int::pow(10, 8),
    Int::pow(10, 9),
    Int::pow(10, 10),
    Int::pow(10, 11),
    Int::pow(10, 12),
];
const fn power_of_ten(x: u32) -> Int {
    POWER_OF_TEN_LOOKUP[x as usize]
    //Int::pow(10, x)
}
// A lookup could be used here, but this isn't called very often.
fn get_duplicate_factor(num_digits: u32) -> Int { (0..num_digits).map(power_of_ten).sum() }
fn repeat_num(num: Int, repetitions: u32) -> Int {
    let digits = digit_count(num);
    (0..repetitions).map(|r| num * power_of_ten(digits * r)).sum()
}
fn repeat_one_with_leading_zeros(repetitions: u32, leading_zeros: u32) -> Int {
    let digits = 1 + leading_zeros;
    (0..repetitions).map(|r| power_of_ten(digits * r)).sum()
}

fn read_input<P>(filename: P) -> Vec<(Int, Int)>
where
    P: AsRef<Path>,
{
    let file_string = std::fs::read_to_string(filename).unwrap();
    let rv = file_string.split(',').map(read_interval).collect();
    rv
}
fn read_interval(interval_string: &str) -> (Int, Int) {
    let mut interval_vec = interval_string.split('-').map(|x| x.trim().parse().unwrap());
    (interval_vec.next().unwrap(), interval_vec.next().unwrap())
}
