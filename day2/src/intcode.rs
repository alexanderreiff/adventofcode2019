use std::ops::{Add, Mul};

const INSTRUCTION_LEN: usize = 4;

pub struct Intcode {
    state: Vec<u32>,
}

impl Intcode {
    pub fn new(state: Vec<u32>) -> Self {
        Self { state }
    }

    pub fn exec(mut self) -> Vec<u32> {
        let mut pointer: usize = 0;
        loop {
            let result = self.update_state(pointer);
            match result {
                Ok(next_pointer) => pointer = next_pointer,
                Err(_) => return self.state,
            }
        }
    }

    fn update_state(&mut self, pointer: usize) -> Result<usize, ()> {
        let next_pointer = pointer + INSTRUCTION_LEN;
        let instruction = {
            let instructions = self
                .state
                .get(pointer..next_pointer)
                .or_else(|| self.state.get(pointer..=pointer));
            match parse_instructions(instructions) {
                Some(value) => value,
                None => return Err(()),
            }
        };
        instruction.exec(&mut self.state);
        Ok(next_pointer)
    }
}

struct Instruction {
    op: fn(u32, u32) -> u32,
    in_pointer1: usize,
    in_pointer2: usize,
    out_pointer: usize,
}

impl Instruction {
    pub fn exec(&self, state: &mut Vec<u32>) {
        let op = self.op;
        state[self.out_pointer] = op(state[self.in_pointer1], state[self.in_pointer2])
    }
}

fn parse_instructions(instructions: Option<&[u32]>) -> Option<Instruction> {
    let instructions = instructions?;
    let op = match instructions[0] {
        1 => u32::add,
        2 => u32::mul,
        99 => return None,
        _ => panic!("Unsupported opcode"),
    };
    let in_pointer1 = instructions[1] as usize;
    let in_pointer2 = instructions[2] as usize;
    let out_pointer = instructions[3] as usize;

    Some(Instruction {
        op,
        in_pointer1,
        in_pointer2,
        out_pointer,
    })
}

#[cfg(test)]
mod intcode_tests {
    use super::*;

    #[test]
    fn it_sets_the_correct_states() {
        let test1 = Intcode::new(vec![1, 0, 0, 0, 99]);
        assert_eq!(test1.exec(), vec![2, 0, 0, 0, 99]);
        let test2 = Intcode::new(vec![2, 3, 0, 3, 99]);
        assert_eq!(test2.exec(), vec![2, 3, 0, 6, 99]);
        let test3 = Intcode::new(vec![2, 4, 4, 5, 99, 0]);
        assert_eq!(test3.exec(), vec![2, 4, 4, 5, 99, 9801]);
        let test4 = Intcode::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        assert_eq!(test4.exec(), vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    #[should_panic]
    fn it_panics_with_unknown_opcodes() {
        let test = Intcode::new(vec![1, 2, 3, 4, 99]);
        test.exec();
    }
}
