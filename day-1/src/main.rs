use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    // part 1
    let input = include_str!("../input.txt");

    let max: String = input.split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
        .to_string();

    println!("part 1 result: {}", max);

    // part 2
    let mut top_three_sum: Vec<u32> = input.split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    top_three_sum.sort_by(|a, b| b.cmp(a));

    println!("part 2 result: {}", top_three_sum.iter().take(3).sum::<u32>().to_string());

    Ok(())
}
