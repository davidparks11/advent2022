use crate::solution::Solution;

pub fn part1(input: Vec<String>) -> Solution {
    let mut max = 0;
    let mut current = 0;

    let mut iter = input.iter().peekable();
    while let Some(line) = iter.next() {
        let calories = line.parse::<i32>().unwrap_or_default();
        current += calories;
        //check if next line is a new elve's calories
        if calories == 0 || iter.peek().is_none() {
            if current > max {
                max = current
            }
            current = 0; //reset current calorie count
        }
    }
    Solution::from(max)
}

pub fn part2(input: Vec<String>) -> Solution {
    let mut top_three = vec![0, 0, 0];
    let mut current = 0;

    let mut iter = input.iter().peekable();
    while let Some(line) = iter.next() {
        let calories = line.parse::<i32>().unwrap_or_default();
        current += calories;
        if calories == 0 || iter.peek().is_none() {
            if current > top_three[0] {
                top_three[2] = top_three[1];
                top_three[1] = top_three[0];
                top_three[0] = current;
            } else if current > top_three[1] {
                top_three[2] = top_three[1];
                top_three[1] = current;
            } else if current > top_three[2] {
                top_three[2] = current;
            }
            current = 0; //reset current calorie count
        }
    }

    Solution::from(top_three[0] + top_three[1] + top_three[2])
}
