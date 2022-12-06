use std::collections::{HashMap, HashSet};

use crate::solution::Solution;

pub fn part1(input: Vec<String>) -> Solution {
    let invalid_types: Vec<char> = input
        .into_iter()
        .map(|sack| get_invalid_type(sack).expect("should have at least one invalid item type"))
        .collect();
    let priority_sum = invalid_types
        .into_iter()
        .fold(0, |acc, v| acc + get_priority(v));
    Solution::I32(priority_sum)
}

pub fn part2(input: Vec<String>) -> Solution {
    let common_items: Vec<char> = input
        .chunks(3)
        .map(|group| get_badge_type(group).expect("should have a shared badge type"))
        .collect();
    let badge_type_sum = common_items
        .into_iter()
        .fold(0, |acc, badge| acc + get_priority(badge));
    Solution::I32(badge_type_sum)
}

fn get_invalid_type(rucksack: String) -> Option<char> {
    //hashset to track with types have been found in the left rucksack
    let mut left_compartment: HashSet<char> = HashSet::new();

    let mut rucksack_items = rucksack.chars().into_iter();

    //add first half of items to set
    for item in rucksack_items.by_ref().take(rucksack.len() / 2) {
        left_compartment.insert(item);
    }

    //search for item that exists in set
    for item in rucksack_items.by_ref() {
        if left_compartment.contains(&item) {
            return Some(item);
        }
    }
    None
}

fn get_badge_type(rucksacks: &[String]) -> Option<char> {
    //number of times each item is found in the elves bags: no-repeats
    let mut counts_for_type: HashMap<char, u8> = HashMap::new();
    for sack in rucksacks.iter() {
        //set to track if each item has been already added to the count map
        let mut items_in_bag = HashSet::new();
        for item in sack.chars().into_iter() {
            if !items_in_bag.contains(&item) {
                items_in_bag.insert(item); //add item to local bag set to avoid counting more than once
                match counts_for_type.get(&item) {
                    //first time item has shown up for all three elves
                    None => {
                        counts_for_type.insert(item, 1);
                    }
                    //item has been found in another elve's bag
                    Some(count) => {
                        if *count == 2 {
                            return Some(item); //item has been found twice, meaning this item if their badge type
                        }
                        counts_for_type.insert(item, 2); //only possiblity is that the count is 1
                    }
                }
            }
        }
    }
    None
}

const CAPITAL_OFFSET: i32 = 27;
fn get_priority(c: char) -> i32 {
    if c <= 'Z' {
        c as i32 - 'A' as i32 + CAPITAL_OFFSET
    } else {
        c as i32 - 'a' as i32 + 1
    }
}
