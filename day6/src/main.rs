use std::num::ParseIntError;

fn main() {
    let (operands, operators) = match parse_input("input") {
        Err(error) => {
            println!("Error occured reading day 5 input: {}", error.to_string());
            return;
        }
        Ok(input_data) => input_data,
    };
    let (part1, part2) = calculate_answers(transpose(operands).unwrap(), operators);
    println!("\tDay 6\nPart 1: {}\nPart 2: {}", part1, part2);
}
fn calculate_answers(operands: Vec<Vec<i64>>, operators: Vec<char>) -> (i64, i64) {
    let mut part1 = 0;
    for (operand_vec, operator) in std::iter::zip(operands.iter(), operators.iter()) {
        match operator {
            '*' => part1 += operand_vec.iter().fold(1_i64, |lhs, rhs| lhs * rhs),
            '+' => part1 += operand_vec.iter().fold(0_i64, |lhs, rhs| lhs + rhs),
            _ => (),
        }
    }
    (part1, 0)
}

fn parse_input<P>(filename: P) -> Result<(Vec<Vec<i64>>, Vec<char>), Box<dyn std::error::Error>>
where
    P: AsRef<std::path::Path>,
{
    let data = std::fs::read_to_string(filename)?;
    let mut lines = data.trim().lines();
    let mut_iterator = &mut lines;
    let operators = mut_iterator
        .rev()
        .take(1)
        .next()
        .expect("File should not be empty after .trim() is called")
        .split(" ")
        .filter(|string| string.len() > 0)
        .map(|x| x.chars().next().unwrap())
        .collect::<Vec<char>>();
    let operands = mut_iterator
        .map(|line| {
            line.split(' ')
                .filter(|x| x.len() > 0)
                .map(str::parse)
                .collect::<Result<Vec<_>, ParseIntError>>()
        })
        .collect::<Result<Vec<_>, ParseIntError>>()?;
    Ok((operands, operators))
}
fn transpose<T>(md_vector: Vec<Vec<T>>) -> Result<Vec<Vec<T>>, Box<dyn std::error::Error>>
where
    T: Copy,
{
    let mut transposed_vector: Vec<Vec<T>> = vec![];
    transposed_vector.resize(md_vector[0].len(), Vec::<T>::new());
    if md_vector.iter().any(|col| col.len() != md_vector[0].len()) {
        return Err("Cannot transpose jagged array".into())
    };
    md_vector
        .iter()
        .map(|x| x.iter())
        .fold(&mut transposed_vector, |vectors, iterator| {
            std::iter::zip(vectors.iter_mut(), iterator)
                .map(|(vec, val)| vec.push(*val))
                .for_each(drop);
            vectors
        });
    Ok(transposed_vector)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let (operands, operators) = parse_input("test").expect("Test should be stored in the file ./test");
        assert_eq!(operators, vec!['*', '+', '*', '+']);
        assert_eq!(
            operands,
            vec![vec![123, 328, 51, 64], vec![45, 64, 387, 23], vec![6, 98, 215, 314]]
        );
    }
    #[test]
    fn test_transpose() {
        let (operands, _operators) = parse_input("test").expect("Test should be stored in the file ./test");
        assert_eq!(
            transpose(operands).expect("Test operands should form a non-jagged Vec"),
            vec![
                vec![123, 45, 6],
                vec![328, 64, 98],
                vec![51, 387, 215],
                vec![64, 23, 314]
            ]
        );
        match transpose(vec![vec![123, 456, 789], vec![123, 456].to_vec()]) {
            Err(_error) => (),
            Ok(_some_val) => panic!("Transpose with jagged array should cause an error"),
        };
    }
    #[test]
    fn test_part_one() {
        let (operands, operators) = parse_input("test").expect("Test should be stored in the file ./test");
        let (part1, _part2) = calculate_answers(transpose(operands).expect("input should not be jagged"), operators);
        assert_eq!(part1, 4277556);
    }
}
