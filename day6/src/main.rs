fn main() {
    let (lines, operators, ranges) = match parse_input("input") {
        Err(error) => {
            println!("Error occured reading day 5 input: {}", error.to_string());
            return;
        }
        Ok(input_data) => input_data,
    };
    let (part1, part2) = calculate_answers(lines, operators, ranges);
    println!("\tDay 6\nPart 1: {}\nPart 2: {}", part1, part2);
}
fn calculate_answers(
    operand_lines: Vec<String>,
    operators: Vec<char>,
    ranges: Vec<std::ops::Range<usize>>,
) -> (i64, i64) {
    let mut part1 = 0;
    let mut part2 = 0;
    // Part 1
    for (operator, range) in std::iter::zip(operators.iter(), ranges.iter()) {
        let part_one_operands = get_part_one_operands(range, &operand_lines);
        let part_two_operands = get_part_two_operands(range, &operand_lines);
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
    operand_lines: &'a Vec<String>,
) -> Box<dyn Iterator<Item = i64> + 'a> {
    let problem_operand_iter = operand_lines
        .iter()
        .map(|x| std::str::from_utf8(&x.as_bytes()[range.clone()]).unwrap().trim())
        .map(str::parse::<i64>)
        .map(|x| x.unwrap_or(0));
    Box::new(problem_operand_iter)
}
fn get_part_two_operands(range: &std::ops::Range<usize>, operand_lines: &Vec<String>) -> Box<dyn Iterator<Item = i64>> {
    let mut operands = Vec::new();
    for idx in range.clone() {
        operands.push(
            std::str::from_utf8(
                operand_lines
                    .iter()
                    .map(|x| x.as_bytes()[idx])
                    .collect::<Vec<_>>()
                    .as_slice(),
            )
            .unwrap()
            .trim()
            .parse::<i64>()
            .unwrap(),
        );
    }
    Box::new(operands.into_iter())
}
fn parse_input<P>(
    filename: P,
) -> Result<(Vec<String>, Vec<char>, Vec<std::ops::Range<usize>>), Box<dyn std::error::Error>>
where
    P: AsRef<std::path::Path>,
{
    let file_data = std::fs::read_to_string(filename)?;
    let mut lines = file_data.lines().filter(|x| x.len() > 0).collect::<Vec<&str>>();
    let last_line = lines.pop().ok_or("Input file should not be empty")?;
    lines
        .iter()
        .all(|line| line.len() == last_line.len())
        .then(|| ())
        .ok_or("Input lines are not matching length")?;
    let (regions, operators) = parse_operator_line(last_line);
    let owned_lines = lines.into_iter().map(String::from).collect();
    Ok((owned_lines, operators, regions))
}
fn parse_operator_line(op_line: &str) -> (Vec<std::ops::Range<usize>>, Vec<char>) {
    // Assumes the line is non empty
    let mut regions = Vec::new();
    let mut operators = Vec::new();
    let mut last_idx = 0;
    operators.push(op_line.chars().next().unwrap());
    for (idx, ch) in op_line.chars().enumerate().skip(1) {
        if ch != ' ' {
            operators.push(ch);
            regions.push(last_idx..(idx - 1));
            last_idx = idx;
        };
    }
    regions.push(last_idx..op_line.len());
    (regions, operators)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let (lines, operators, regions) = parse_input("test").expect("Input file should be at './input'");
        assert_eq!(operators, vec!['*', '+', '*', '+']);
        assert_eq!(lines, vec!["123 328  51 64 ", " 45 64  387 23 ", "  6 98  215 314"]);
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
