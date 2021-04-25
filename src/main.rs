use std::collections::HashMap;
use std::io;
use std::io::Read;

use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
}

fn main() -> Result<(), Error> {
    let stdin = io::stdin();
    let mut inputs = String::new();
    stdin.lock().read_to_string(&mut inputs)?;

    println!(
        "{:?}",
        sum_of_groups(&inputs.split("\n\n").collect::<Vec<_>>())
    );

    Ok(())
}

fn sum_of_groups(input: &[&str]) -> usize {
    return input
        .iter()
        .map(|group| {
            group
                .chars()
                .filter(|char| ('a'..='z').contains(char))
                .fold(HashMap::<char, usize>::new(), |mut map, char| {
                    map.entry(char).and_modify(|x| *x += 1).or_insert(1);
                    map
                })
                .iter()
                .filter(|(_, count)| **count == group.lines().count())
                .count()
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use crate::sum_of_groups;

    #[test]
    fn no_input_is_none() {
        assert_eq!(0, sum_of_groups(&Vec::new()));
    }

    #[test]
    fn one_group() {
        assert_eq!(3, sum_of_groups(&["abc\nabc"]));
        assert_eq!(0, sum_of_groups(&["a\nb\nc"]));
    }

    #[test]
    fn two_groups() {
        assert_eq!(3, sum_of_groups(&["abz\nabz\nabz\nabz", "a\nb\nz"]));
    }

    #[test]
    fn example_from_page() {
        assert_eq!(
            6,
            sum_of_groups(&["abc", "a\nb\nc", "ab\nac", "a\na\na\na", "b"])
        );
    }
}
