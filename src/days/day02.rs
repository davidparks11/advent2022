use crate::solution::Solution;

enum Scores {
    Rock = 1,
    Paper,
    Scissors,
}

pub fn part1(input: Vec<String>) -> Solution {
    let score = input
        .iter()
        .fold(0, |acc, game| acc + rock_paper_scissors(parse_game(game)));
    Solution::from(score)
}

pub fn part2(input: Vec<String>) -> Solution {
    let score = input.iter().fold(0, |acc, game| {
        acc + rock_paper_scissors_directed(parse_game(game))
    });
    Solution::from(score)
}

fn parse_game(game: &String) -> (char, char) {
    (
        game.chars().nth(0).expect("good input"),
        game.chars().nth(2).expect("good input"),
    )
}

fn rock_paper_scissors(sides: (char, char)) -> i32 {
    match sides {
        //first player rock
        ('A', _) => match sides.1 {
            'X' => Scores::Scissors as i32 + 0,
            'Y' => Scores::Rock as i32 + 3,
            'Z' => Scores::Paper as i32 + 6,
            _ => panic!("bad input!"),
        },
        //first player paper
        ('B', _) => match sides.1 {
            'X' => Scores::Rock as i32 + 0,
            'Y' => Scores::Paper as i32 + 3,
            'Z' => Scores::Scissors as i32 + 6,
            _ => panic!("bad input!"),
        },
        //first player scissor
        ('C', _) => match sides.1 {
            'X' => Scores::Paper as i32 + 0,
            'Y' => Scores::Scissors as i32 + 3,
            'Z' => Scores::Rock as i32 + 6,
            _ => panic!("bad input!"),
        },
        _ => panic!("bad input!"),
    }
}

fn rock_paper_scissors_directed(sides: (char, char)) -> i32 {
    match sides {
        //first player rock
        ('A', _) => match sides.1 {
            'X' => Scores::Rock as i32 + 3,
            'Y' => Scores::Paper as i32 + 6,
            'Z' => Scores::Scissors as i32 + 0,
            _ => panic!("bad input!"),
        },
        //first player paper
        ('B', _) => match sides.1 {
            'X' => Scores::Rock as i32 + 0,
            'Y' => Scores::Paper as i32 + 3,
            'Z' => Scores::Scissors as i32 + 6,
            _ => panic!("bad input!"),
        },
        //first player scissor
        ('C', _) => match sides.1 {
            'X' => Scores::Rock as i32 + 6,
            'Y' => Scores::Paper as i32 + 0,
            'Z' => Scores::Scissors as i32 + 3,
            _ => panic!("bad input!"),
        },
        _ => panic!("bad input!"),
    }
}
