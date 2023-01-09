use std::error::Error;

enum Move {
    Rock,
    Paper,
    Scissors
}

impl Move {
    fn get_move(str: &str) -> Self {
        match str {
            "X" | "A" => Move::Rock,
            "Y" | "B" => Move::Paper,
            "Z" | "C" => Move::Scissors,
            _ => panic!("you have brain damage"),
        } 
    }

    fn get_part1_result(opponent: &Move, mine: &Move) -> u32 {
        let choice_count: u32 = match mine {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3
        };

        let result_count: u32 = match mine {
            Move::Rock => match opponent {
                Move::Rock => 3,
                Move::Paper => 0,
                Move::Scissors => 6
            },
            Move::Paper => match opponent {
                Move::Rock => 6,
                Move::Paper => 3,
                Move::Scissors => 0
            },
            Move::Scissors => match opponent {
                Move::Rock => 0,
                Move::Paper => 6,
                Move::Scissors => 3
            }
        };

        choice_count + result_count
    }

    fn get_part2_result(opponent: &Move, mine: &Move) -> u32 {
        let result_count: u32 = match mine {
            Move::Rock => 0,
            Move::Paper => 3,
            Move::Scissors => 6
        };

        let choice_count: u32 = match opponent {
            Move::Rock => match mine {
                Move::Rock => 3,
                Move::Paper => 1,
                Move::Scissors => 2
            },
            Move::Paper => match mine {
                Move::Rock => 1,
                Move::Paper => 2,
                Move::Scissors => 3
            },
            Move::Scissors => match mine {
                Move::Rock => 2,
                Move::Paper => 3,
                Move::Scissors => 1
            },
        };
        
        result_count + choice_count

    }
}

fn main() -> Result<(), Box<dyn Error>> {
    const INPUT: &str = include_str!("../input.txt");
    
    /*
    const INPUT: &str = "A Y
B X
C Z";
    */
    
    let part1_result: u32 = INPUT.lines()
        .map(|line| {
            let moves: Vec<Move> = line.split(" ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|str| Move::get_move(str))
                .collect::<Vec<Move>>();

            Move::get_part1_result(&moves[0], &moves[1])
        })
        .sum();

    let part2_result: u32 = INPUT.lines()
        .map(|line| {
            let moves: Vec<Move> = line.split(" ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|str| Move::get_move(str))
                .collect::<Vec<Move>>();

            Move::get_part2_result(&moves[0], &moves[1])
        })
        .sum();

    println!("part 1 result: {part1_result}");
    println!("part 2 result: {part2_result}");

    Ok(())
}
