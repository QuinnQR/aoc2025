
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
pub fn get_lines_from_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename.as_ref())?;
    Ok(io::BufReader::new(file).lines())
}
