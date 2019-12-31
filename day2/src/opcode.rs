use std::ops::{Add, Mul};

const OP_LEN: usize = 4;

pub struct Opcode {
    state: Vec<u32>,
}

impl Opcode {
    pub fn new(state: Vec<u32>) -> Self {
        Self { state }
    }

    pub fn exec(mut self) -> Vec<u32> {
        let mut counter: usize = 0;
        loop {
            let result = self.update_state(counter);
            match result {
                Ok(next_counter) => counter = next_counter,
                Err(_) => return self.state,
            }
        }
    }

    fn update_state(&mut self, counter: usize) -> Result<usize, ()> {
        let next_counter = counter + OP_LEN;
        let op_set = {
            let instructions = self
                .state
                .get(counter..next_counter)
                .or_else(|| self.state.get(counter..=counter));
            match parse_instruction_set(instructions) {
                Some(value) => value,
                None => return Err(()),
            }
        };
        op_set.exec(&mut self.state);
        Ok(next_counter)
    }
}

struct OpSet {
    op: fn(u32, u32) -> u32,
    in_position1: usize,
    in_position2: usize,
    out_position: usize,
}

impl OpSet {
    pub fn exec(&self, state: &mut Vec<u32>) {
        let op = self.op;
        state[self.out_position] = op(state[self.in_position1], state[self.in_position2])
    }
}

fn parse_instruction_set(instructions: Option<&[u32]>) -> Option<OpSet> {
    let instructions = instructions?;
    let op = match instructions[0] {
        1 => u32::add,
        2 => u32::mul,
        99 => return None,
        _ => panic!("Unsupported opcode"),
    };
    let in_position1 = instructions[1] as usize;
    let in_position2 = instructions[2] as usize;
    let out_position = instructions[3] as usize;

    Some(OpSet {
        op,
        in_position1,
        in_position2,
        out_position,
    })
}

#[cfg(test)]
mod opcode_tests {
    use super::*;

    #[test]
    fn it_sets_the_correct_states() {
        let test1 = Opcode::new(vec![1, 0, 0, 0, 99]);
        assert_eq!(test1.exec(), vec![2, 0, 0, 0, 99]);
        let test2 = Opcode::new(vec![2, 3, 0, 3, 99]);
        assert_eq!(test2.exec(), vec![2, 3, 0, 6, 99]);
        let test3 = Opcode::new(vec![2, 4, 4, 5, 99, 0]);
        assert_eq!(test3.exec(), vec![2, 4, 4, 5, 99, 9801]);
        let test4 = Opcode::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        assert_eq!(test4.exec(), vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    #[should_panic]
    fn it_panics_with_unknown_opcodes() {
        let test = Opcode::new(vec![1, 2, 3, 4, 99]);
        test.exec();
    }
}
