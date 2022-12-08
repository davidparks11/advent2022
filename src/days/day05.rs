use crate::solution::Solution;

pub fn part1(input: Vec<String>) -> Solution {
    let mut stack = SupplyStack::new(&input);
    for instruction in input.iter().skip(stack.instruction_index) {
        stack.process_move(instruction, false);
    }
    Solution::Str(stack.get_top_crate())
}

pub fn part2(input: Vec<String>) -> Solution {
    let mut stack = SupplyStack::new(&input);
    for instruction in input.iter().skip(stack.instruction_index) {
        stack.process_move(instruction, true);
    }
    Solution::Str(stack.get_top_crate())
}

#[derive(Debug)]
struct SupplyStack {
    stack: Vec<Vec<char>>,
    instruction_index: usize,
}

const COLUMN_WIDTH: usize = 4;
const BLANK: char = ' ';
impl SupplyStack {
    //new creates a new SupplyStack by first finding the empty ling between the starting
    //cargo state, and the move instructions. A vector of vector of chars is made to hold
    //all crates. The crates are then added from bottom to top, and the instruction index
    // stored in SupplyStack
    pub fn new(input: &Vec<String>) -> SupplyStack {
        let mut crate_floor_index = 0;
        for (pos, line) in input.iter().enumerate() {
            if line == "" {
                crate_floor_index = pos - 1;
                break;
            }
        }

        let starting_state = &input.as_slice()[0..crate_floor_index];
        let column_count = (starting_state.last().unwrap().len() + 1) / COLUMN_WIDTH;
        let mut columns: Vec<Vec<char>> = Vec::new();
        for _ in 0..column_count {
            columns.push(Vec::new());
        }

        for line in starting_state.iter().rev() {
            for current_column in 0..column_count {
                match SupplyStack::get_crate_index(current_column, line) {
                    Some(c) => columns.get_mut(current_column).unwrap().push(c),
                    None => (),
                }
            }
        }

        let s = SupplyStack {
            stack: columns,
            instruction_index: crate_floor_index + 2,
        };

        s
    }

    //move input is always in format "move x from y to z". Numbers are parse after
    //splitting the instruction by " ". Any that are not okay are filter out and the
    //remaining vector holds x, y, and z.
    fn process_move(&mut self, move_instruction: &String, in_order: bool) {
        let vals: Vec<usize> = move_instruction
            .split(" ")
            .filter_map(|m| m.parse().ok())
            .collect();
        let crate_count = vals.get(0).expect("instruction should have target");
        let src = vals.get(1).expect("source should have target");
        let dst = vals.get(2).expect("destination should have target");

        self.move_crates(in_order, *crate_count, *src - 1, *dst - 1);
    }

    //moves crate_count items from src to dst either in order or reverse order depending on in_order
    fn move_crates(&mut self, in_order: bool, crate_count: usize, src: usize, dst: usize) {
        let len = self.stack.get(src).unwrap().len();
        let mut crates = self
            .stack
            .get_mut(src)
            .unwrap()
            .split_off(len - crate_count);
        if !in_order {
            crates.reverse();
        }
        self.stack.get_mut(dst).unwrap().append(&mut crates);
    }

    /**
     * Was part of part1 - generalized to move_crates in part2
     */
    // fn move_crate(&mut self, src: usize, dst: usize) {
    //     match self.stack.get(src) {
    //         None => {
    //             return;
    //         }
    //         Some(_) => {}
    //     }
    //     let val = self.stack.get_mut(src).unwrap().pop().unwrap();
    //     self.stack.get_mut(dst).unwrap().push(val);
    // }

    //get_crate_index is a helper method to return a crate for a given column index. Since a 
    //column may not contain a crate for the given location, it is returned as an Option<char>
    fn get_crate_index(crate_num: usize, crate_line: &String) -> Option<char> {
        let c: char = *crate_line.as_bytes().get(crate_num * 4 + 1).unwrap() as char;
        if c == BLANK {
            return None;
        }
        Some(c)
    }

    //get_top_crate reads off the top of each column and produces a string from the crate.
    fn get_top_crate(&self) -> String {
        let mut top_row: Vec<char> = Vec::new();
        for col in self.stack.iter() {
            top_row.push(*col.last().unwrap());
        }

        top_row.iter().collect::<String>()
    }
}
