use shared;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Lines};
use std::num::ParseIntError;

fn main() {
    let (intervals, ingerdients) = match read_input("input") {
        Err(error) => {
            println!("Error occured reading day 5 input: {}", error.to_string());
            return;
        }
        Ok(iterator) => iterator,
    };
    let (part1, part2) = calculate_answers(intervals, ingerdients);
    println!("\tDay 3\nPart 1: {}\nPart 2: {}", part1, part2);
}

fn calculate_answers(intervals: Vec<(i64, i64)>, ingredients: Vec<i64>) -> (i64, i64) {
    let intervals = merge_all_intervals(intervals);
    let mut part1 = 0;
    for ingredient in ingredients {
        for interval in intervals.iter() {
            if item_in_interval(ingredient, *interval) {
                part1 += 1;
                break;
            }
        }
    }
    let part2 = intervals.iter().map(|x| x.1 - x.0 + 1).sum();
    (part1, part2)
}

fn merge_all_intervals(intervals: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut old_intervals: Vec<(i64, i64)> = vec![];
    let mut new_intervals: Vec<(i64, i64)> = vec![];
    for mut interval in intervals {
        std::mem::swap(&mut new_intervals, &mut old_intervals);
        new_intervals.clear();
        for old_interval in old_intervals.iter() {
            if (intervals_overlap(interval, *old_interval)) {
                interval = merge_two_intervals(interval, *old_interval)
            } else {
                new_intervals.push(*old_interval);
            }
        }
        new_intervals.push(interval);
    }
    new_intervals
}

fn intervals_overlap(interval1: (i64, i64), interval2: (i64, i64)) -> bool {
    return item_in_interval(interval1.0, interval2)
        || item_in_interval(interval1.1, interval2)
        || item_in_interval(interval2.0, interval1)
        || item_in_interval(interval2.1, interval1)
}

fn merge_two_intervals(interval1: (i64, i64), interval2: (i64, i64)) -> (i64, i64) {
    return (
        std::cmp::min(interval1.0, interval2.0),
        std::cmp::max(interval1.1, interval2.1),
    );
}
fn item_in_interval(item: i64, interval: (i64, i64)) -> bool { item >= interval.0 && item <= interval.1 }

fn read_input<P>(filename: P) -> Result<(Vec<(i64, i64)>, Vec<i64>), Box<dyn Error>>
where
    P: AsRef<std::path::Path>,
{
    let mut line_iterator = shared::get_lines_from_file(filename)?;
    let mut_ref: &mut Lines<BufReader<File>> = &mut line_iterator;
    let intervals: Vec<(i64, i64)> = mut_ref
        .take_while(|x| x.is_ok() && x.as_ref().unwrap().len() > 0)
        .map(Result::unwrap)
        .map(parse_interval)
        .collect::<Result<Vec<(i64, i64)>, Box<dyn Error>>>()?;
    let ingredients = mut_ref
        .take_while(|x| x.as_ref().is_ok() && x.as_ref().unwrap().len() > 0)
        .map(Result::unwrap)
        .map(|x| x.as_str().parse())
        .collect::<Result<Vec<i64>, ParseIntError>>()?;
    Ok((intervals, ingredients))
}

fn parse_interval(interval_string: String) -> Result<(i64, i64), Box<dyn Error>> {
    let x = interval_string
        .split('-')
        .map(str::parse)
        .collect::<Result<Vec<i64>, ParseIntError>>()?;
    if x.len() != 2 {
        return Err("Invalid interval format in input file".into());
    }
    Ok((x[0], x[1]))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_parse() {
        let (intervals, ingredients) = match read_input("test") {
            Err(error) => {
                println!("Error reading test data: {}", error.to_string());
                panic!();
            }
            Ok(input_data) => input_data,
        };
        assert_eq!(intervals, vec![(3, 5), (10, 14), (16, 20), (12, 18)]);
        assert_eq!(ingredients, vec![1, 5, 8, 11, 17, 32]);
    }
    #[test]
    fn test_combine_intervals() {
        let (intervals, _ingredients) = read_input("test").unwrap();
        let new_intervals = merge_all_intervals(intervals);
        assert_eq!(new_intervals, vec![(3, 5), (10, 20)]);
    }
    #[test]
    fn test_part_one() {
        let (intervals, ingredients) = read_input("test").unwrap();
        let (part1, _part2) = calculate_answers(intervals, ingredients);
        assert_eq!(part1, 3);
    }
    #[test]
    fn test_part_two() {
        let (intervals, ingredients) = read_input("test").unwrap();
        let (_part1, part2) = calculate_answers(intervals, ingredients);
        assert_eq!(part2, 14);
    }
}
