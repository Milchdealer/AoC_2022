use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn main() {
    let alphabet: HashMap<char, u32> = (b'A'..=b'z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .enumerate()
        .map(|(i, c)| (u32::try_from(i + 1).unwrap(), c))
        .map(|(i, c)| if i <= 26 { (c, i + 26) } else { (c, i - 26) })
        .collect();

    let content = fs::read_to_string("input").expect("Should have been able to read the file");

    let result: u32 = content
        .split("\n")
        .map(|line| {
            let len = line.len() / 2;
            (&line[..len], &line[len..])
        })
        .map(|(left, right)| {
            left.chars()
                .filter(|c| right.contains(*c))
                .unique()
                .collect::<Vec<char>>()
        })
        .map(|items| {
            items
                .iter()
                .map(|i: &char| alphabet.get(i).unwrap())
                .sum::<u32>()
        })
        .sum();
    println!("{:?}", result);

    let result: u32 = content
        .split("\n")
        .chunks(3)
        .into_iter()
        .map(|mut group| {
            let one = group.next().unwrap_or_default();
            let two = group.next().unwrap_or_default();
            let three = group.next().unwrap_or_default();

            one.chars()
                .filter(|c| two.contains(*c))
                .filter(|c| three.contains(*c))
                .filter(|c| *c != '\r')
                .unique()
                .map(|c| alphabet.get(&c).unwrap())
                .sum::<u32>()
        })
        .sum();
    println!("{:?}", result);
}
