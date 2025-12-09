fn main() {
    let layout = match parse_input("input") {
        Err(error) => {
            println!("Error occured reading day 7 input: {}", error.to_string());
            return;
        }
        Ok(input_data) => input_data,
    };
    let (part1, part2) = calculate_answers(layout);
    println!("\tDay 7\nPart 1: {}\nPart 2: {}", part1, part2);
}
fn calculate_answers(layout: Layout) -> (i64, i64) {
    let mut part1 = 0;
    let mut beam_counts: Vec<i64> = Vec::new();
    beam_counts.resize(layout.right_index + 1, 0);
    beam_counts[layout.start_index] = 1;
    for splitter_line in layout.splitter_indices {
        for splitter_position in splitter_line {
            if beam_counts[splitter_position] != 0 {
                part1 += 1;
                if splitter_position != 0 {
                    beam_counts[splitter_position - 1] += beam_counts[splitter_position];
                }
                if splitter_position != layout.right_index {
                    beam_counts[splitter_position + 1] += beam_counts[splitter_position];
                }
                beam_counts[splitter_position] = 0;
            }
        }
    }
    let part2 = beam_counts.into_iter().fold(0, |x, y| x + y);
    (part1, part2)
}
struct Layout {
    // I don't think the input can ever split out of bounds, but right_col can be used to make sure
    right_index: usize,
    start_index: usize,
    splitter_indices: Vec<Vec<usize>>,
}
fn parse_input<P>(filename: P) -> Result<Layout, Box<dyn std::error::Error>>
where
    P: AsRef<std::path::Path>,
{
    let file_string = std::fs::read_to_string(filename)?;
    let mut lines = file_string.trim().lines();
    let first_line = lines
        .next()
        .ok_or::<Box<dyn std::error::Error>>("Input should not be empty".into())?
        .chars()
        .collect::<Vec<_>>();
    let start_col = first_line.iter().position(|character| character == &'S').unwrap_or(0);
    let right_col = first_line.len() - 1;
    let splitter_cols = lines.map(parse_splitter_line).collect::<Vec<Vec<usize>>>();
    Ok(Layout {
        right_index: right_col,
        start_index: start_col,
        splitter_indices: splitter_cols,
    })
}
fn parse_splitter_line(line: &str) -> Vec<usize> {
    let mut splitter_locations = Vec::new();
    for (idx, ch) in line.chars().enumerate() {
        if ch == '^' {
            splitter_locations.push(idx);
        }
    }
    splitter_locations
}

#[cfg(test)]
mod tests {
    use crate::{calculate_answers, parse_input};

    #[test]
    fn test_parse_input() {
        let layout = parse_input("test").expect("Test input should be stored at ./test");
        assert_eq!(layout.right_index, 14);
        assert_eq!(layout.start_index, 7);
        assert_eq!(layout.splitter_indices.len(), 15);
        assert_eq!(layout.splitter_indices[1], vec![7]);
        assert_eq!(layout.splitter_indices[2], vec![]);
        assert_eq!(layout.splitter_indices[3], vec![6, 8]);
    }
    #[test]
    fn test_part_one() {
        let (part1, _part2) = calculate_answers(parse_input("test").expect("Test input should be stored at ./test"));
        assert_eq!(part1, 21);
    }
    #[test]
    fn test_part_two() {
        let (_part1, part2) = calculate_answers(parse_input("test").expect("Test input should be stored at ./test"));
        assert_eq!(part2, 40);
    }
}
