use std::str::Chars;

fn main() {
    let (operand_lines, operators, problem_ranges) = match parse_input("input") {
        Err(error) => {
            println!("Error occured reading day 6 input: {}", error.to_string());
            return;
        }
        Ok(input_data) => input_data,
    };
    let (part1, part2) = calculate_answers(operand_lines, operators, problem_ranges);
    println!("\tDay 6\nPart 1: {}\nPart 2: {}", part1, part2);
}
fn calculate_answers(
    operand_lines: Vec<Vec<char>>,
    operators: Vec<char>,
    ranges: Vec<std::ops::Range<usize>>,
) -> (i64, i64) {
    let mut part1 = 0;
    let mut part2 = 0;
    // Part 1
    for (operator, problem_range) in std::iter::zip(operators.iter(), ranges.iter()) {
        let part_one_operands = get_part_one_operands(problem_range, &operand_lines);
        let part_two_operands = get_part_two_operands(problem_range, &operand_lines);
        let part_one_answer = match operator {
            '+' => part_one_operands.fold(0_i64, |lhs, rhs| lhs + rhs),
            '*' => part_one_operands.fold(1_i64, |lhs, rhs| lhs * rhs),
            _ => 0,
        };
        let part_two_answer = match operator {
            '+' => part_two_operands.into_iter().fold(0_i64, |lhs, rhs| lhs + rhs),
            '*' => part_two_operands.into_iter().fold(1_i64, |lhs, rhs| lhs * rhs),
            _ => 0,
        };
        part2 += part_two_answer;
        part1 += part_one_answer;
    }
    // Part 2
    (part1, part2)
}
fn get_part_one_operands<'a>(
    range: &'a std::ops::Range<usize>,
    operand_lines: &'a Vec<Vec<char>>,
) -> Box<dyn Iterator<Item = i64> + 'a> {
    // Need String to stay alive long enough to use &str
    Box::new(
        operand_lines
            .iter()
            .map(|x| x[range.clone()].into_iter().collect::<String>())
            .map(parse_operand_string),
    )
}
fn get_part_two_operands(
    range: &std::ops::Range<usize>,
    operand_lines: &Vec<Vec<char>>,
) -> Box<dyn Iterator<Item = i64>> {
    let mut operands = Vec::new();
    // Unlikely to be more than this, might be less.
    operands.reserve(4);
    for idx in range.clone() {
        operands.push(parse_operand_string(
            operand_lines.iter().map(|x| x[idx]).collect::<String>(),
        ));
    }
    Box::new(operands.into_iter())
}
fn parse_operand_string(operand_string: String) -> i64 { operand_string.as_str().trim().parse::<i64>().unwrap_or(0) }
fn parse_input<P>(
    filename: P,
) -> Result<(Vec<Vec<char>>, Vec<char>, Vec<std::ops::Range<usize>>), Box<dyn std::error::Error>>
where
    P: AsRef<std::path::Path>,
{
    let file_data = std::fs::read_to_string(filename)?;
    let mut line_vec = file_data.lines().filter(|x| x.len() > 0).collect::<Vec<&str>>();
    let last_line = line_vec.pop().ok_or("Input file should not be empty")?;
    line_vec
        .iter()
        .all(|line| line.len() == last_line.len())
        .then(|| ())
        .ok_or("Input lines are not matching length")?; // Make sure all lines are the same length
    let (problem_regions, operators) = parse_operator_line(last_line);
    let char_line_vec = line_vec.into_iter().map(str::chars).map(Chars::collect).collect();
    Ok((char_line_vec, operators, problem_regions))
}
fn parse_operator_line(operator_line: &str) -> (Vec<std::ops::Range<usize>>, Vec<char>) {
    // Assumes the line is non empty
    let mut problem_regions = Vec::new();
    let mut operators = Vec::new();
    let mut last_operator_idx = 0;
    operators.push(operator_line.chars().next().unwrap());
    for (idx, ch) in operator_line.chars().enumerate().skip(1) {
        if ch != ' ' {
            operators.push(ch);
            problem_regions.push(last_operator_idx..(idx - 1));
            last_operator_idx = idx;
        };
    }
    problem_regions.push(last_operator_idx..operator_line.len());
    (problem_regions, operators)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let (lines, operators, regions) = parse_input("test").expect("Input file should be at './input'");
        assert_eq!(operators, vec!['*', '+', '*', '+']);
        assert_eq!(
            lines,
            vec![
                "123 328  51 64 ".chars().collect::<Vec<_>>(),
                " 45 64  387 23 ".chars().collect::<Vec<_>>(),
                "  6 98  215 314".chars().collect::<Vec<_>>()
            ]
        );
        assert_eq!(regions, vec![0..3, 4..7, 8..11, 12..15]);
    }
    #[test]
    fn test_part_one() {
        let (lines, operators, regions) = parse_input("test").expect("Input file should be at './input'");
        let (part1, _part2) = calculate_answers(lines, operators, regions);
        assert_eq!(part1, 4277556);
    }
    #[test]
    fn test_part_two() {
        let (lines, operators, regions) = parse_input("test").expect("Input file should be at './input'");
        let (_part1, part2) = calculate_answers(lines, operators, regions);
        assert_eq!(part2, 3263827);
    }
}
