fn main() {
    let layout = match read_input("input") {
        Err(error) => {
            println!("Error occured reading day 4 input: {}", error.to_string());
            return;
        }
        Ok(input_data) => input_data,
    };
    let (part1, part2) = calculate_answers(layout);
    println!("\tDay 4\nPart 1: {}\nPart 2: {}", part1, part2);
}
fn calculate_answers(layout: Vec<Vec<bool>>) -> (i32, i32) {
    let mut new_layout;
    let mut num_removed;
    (new_layout, num_removed) = remove_rolls(layout);
    let part1 = num_removed;
    let mut part2 = part1;
    while num_removed > 0 {
        (new_layout, num_removed) = remove_rolls(new_layout);
        part2 += num_removed;
    }
    (part1, part2)
}
fn remove_rolls(layout: Vec<Vec<bool>>) -> (Vec<Vec<bool>>, i32) {
    // Can't modify in place as this is also used for part 1.
    // Likely would be faster if modified in place instead
    // (If part 1 was given its own function)
    let mut new_layout = layout.clone();
    let mut num_removed = 0;
    for row in 1..(layout.len() - 1) {
        for col in 1..(layout[row].len() - 1) {
            if layout[row][col] {
                let surroundings = layout[row][col - 1..col + 2]
                    .iter()
                    .chain(layout[row - 1][col - 1..col + 2].iter())
                    .chain(layout[row + 1][col - 1..col + 2].iter())
                    .filter(|&&x| x)
                    .count();
                if surroundings < 5 {
                    num_removed += 1;
                    new_layout[row][col] = false;
                }
            }
        }
    }
    (new_layout, num_removed)
}
fn read_input<P>(filename: P) -> Result<Vec<Vec<bool>>, Box<dyn std::error::Error>>
where
    P: AsRef<std::path::Path>,
{
    let mut layout = std::fs::read_to_string(filename)?
        .trim()
        .lines()
        .map(|line| {
            let mut line_vec = vec![false];
            line_vec.extend(
                line.chars()
                    .map(|character| if character == '@' { true } else { false }),
            );
            line_vec.push(false);
            line_vec
        })
        .collect::<Vec<_>>();
    let line_length = layout[0].len();
    layout.insert(0, std::iter::repeat(false).take(line_length).collect());
    layout.push(std::iter::repeat(false).take(line_length).collect());
    Ok(layout)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_parse() {
        let data = match read_input("test") {
            Err(error) => {
                println!("Error reading test input: {}", error.to_string());
                panic!();
            }
            Ok(data) => data,
        };
        assert!(data.len() > 2);
        assert_eq!(data[0], [false; 12]);
        assert_eq!(data[data.len() - 1], [false; 12]);
        assert_eq!(
            data[1],
            vec![
                false, false, false, true, true, false, true, true, true, true, false, false
            ]
        );
        assert_eq!(
            data[data.len() - 2],
            vec![
                false, true, false, true, false, true, true, true, false, true, false, false
            ]
        );
    }
    #[test]
    fn test_part_one() {
        let data = read_input("test").unwrap();
        let (part1, _part2) = calculate_answers(data);
        assert_eq!(part1, 13);
    }
    #[test]
    fn test_part_two() {
        let data = read_input("test").unwrap();
        let (_part1, part2) = calculate_answers(data);
        assert_eq!(part2, 43);
    }
}
