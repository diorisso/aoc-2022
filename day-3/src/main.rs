use std::error::Error;

fn get_priority(letter: char) -> usize {
    let mut alphabet = ('a'..='z').chain('A'..='Z');

    alphabet
        .position(|i| i == letter)
        .unwrap() + 1
}

fn main() -> Result<(), Box<dyn Error>> {
    // part 1
    const INPUT: &str = include_str!("../input.txt");

    let part1_result: usize = INPUT.lines()
        .map(|rucksack| {
            let (compartment_a, compartment_b) = rucksack.split_at(rucksack.len() / 2);

            let index = compartment_a
                .find(|char| compartment_b.contains(char))
                .unwrap();

            rucksack.chars().nth(index).unwrap()
        })
        .map(|char| get_priority(char))
        .sum::<usize>();

    println!("part 1 result: {part1_result}");

    Ok(())
}
