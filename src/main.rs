use std::collections::HashSet;
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
        .map(|item| {
            item.chars()
                .filter(|char| ('a'..='z').contains(char))
                .collect::<HashSet<char>>()
                .len()
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
        assert_eq!(3, sum_of_groups(&["abc"]));
        assert_eq!(3, sum_of_groups(&["a\nb\nc"]));
    }
    #[test]
    fn two_groups() {
        assert_eq!(6, sum_of_groups(&["abz", "a\nb\nz"]));
    }
}
