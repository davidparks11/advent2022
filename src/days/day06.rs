use std::collections::HashMap;

use crate::solution::Solution;

pub fn part1(input: Vec<String>) -> Solution {
    Solution::Usize(find_start_packet_marker_index(input.get(0).unwrap(), 4))
}

pub fn part2(input: Vec<String>) -> Solution {
    Solution::Usize(find_start_packet_marker_index(input.get(0).unwrap(), 14))
}

fn find_start_packet_marker_index(data_stream: &String, marker_size: usize) -> usize {
    let mut char_count: HashMap<char, u8> = HashMap::new();
    let mut char_queue: Vec<char> = Vec::new();

    data_stream.chars().take(marker_size - 1).for_each(|c| {
        match char_count.get(&c) {
            None => char_count.insert(c, 1),
            Some(count) => char_count.insert(c, count + 1),
        };
        char_queue.push(c);
    });

    for (pos, c) in data_stream.chars().skip(marker_size - 1).enumerate() {
        //check if map contains char, if not, check that map has size of marker - 1 which
        //indicates marker number of unique elements
        if char_count.get(&c).is_none() && char_count.len() == marker_size - 1 {
            return pos + marker_size;
        }

        //decrement of remove exiting value
        let exit_char = char_queue.get(0).unwrap();
        let exit_char_count = char_count.get(exit_char).unwrap();
        if *exit_char_count == 1 {
            char_count.remove(exit_char);
        } else {
            char_count.insert(*exit_char, *exit_char_count - 1);
        }

        //drop first values in buffer
        char_queue.drain(0..1);

        //add or increment char count
        match char_count.get(&c) {
            None => char_count.insert(c, 1),
            Some(count) => char_count.insert(c, count + 1),
        };

        //add currrent char
        char_queue.push(c);
    }
    0
}
