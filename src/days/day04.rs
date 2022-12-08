use crate::solution::Solution;

pub fn part1(input: Vec<String>) -> Solution {
    let pair_count = input.iter().fold(0, |acc, line| {
        let (lhs, rhs) = parse_pairs(line);
        if lhs.sym_contains(rhs) {
            return acc + 1;
        }
        acc
    });
    Solution::I32(pair_count)
}

pub fn part2(input: Vec<String>) -> Solution {
    let pair_count = input.iter().fold(0, |acc, line| {
        let (lhs, rhs) = parse_pairs(line);
        if lhs.overlaps(rhs) {
            return acc + 1;
        }
        acc
    });
    Solution::I32(pair_count)
}

fn parse_pairs(input: &String) -> (Range, Range) {
    let sides: Vec<String> = input.split(',').map(|s| s.to_string()).collect();
    let lhs = sides.get(0).expect("input should have left range");
    let rhs = sides.get(1).expect("input should have right range");
    (Range::new(lhs), Range::new(rhs))
}

#[derive(Debug)]
struct Range {
    lhs: u8,
    rhs: u8,
}

impl Range {
    fn new(range: &str) -> Range {
        let sides: Vec<&str> = range.split("-").collect();
        Range { 
            lhs: sides.get(0).unwrap().parse().expect("num should be parsable u8"),
         rhs: sides.get(1).unwrap().parse().expect("num should be parsable u8")
         }
    }

    //check wether one of the range contains the other
    fn sym_contains(&self, other: Range) -> bool {
        if other.lhs <= self.lhs {
            if other.rhs >= self.rhs {
                return true;
            }
        } 
        if self.lhs <= other.lhs {
            return self.rhs >= other.rhs;
        }
        false
    }

    fn overlaps(&self, other: Range) -> bool {
        if other.lhs <= self.lhs {
            return other.rhs >= self.lhs;
        }
        self.rhs >= other.lhs
    }
}
