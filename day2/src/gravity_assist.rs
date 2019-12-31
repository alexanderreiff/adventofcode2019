use crate::intcode::*;
use std::cmp::min;

pub fn input_pair_for_output(state: &[u32], output: u32) -> Option<(u32, u32)> {
    let bounds = min(state.len(), 100) as u32;
    for noun in 0..bounds {
        for verb in 0..bounds {
            let init_state = reset_state(state, noun, verb);
            let intcode = Intcode::new(init_state);
            let new_state = intcode.exec();
            if new_state[0] == output {
                return Some((noun, verb));
            }
        }
    }
    None
}

fn reset_state(state: &[u32], noun: u32, verb: u32) -> Vec<u32> {
    let mut new_state = state.to_owned();
    new_state[1] = noun;
    new_state[2] = verb;
    new_state
}

#[cfg(test)]
mod gravity_assist_tests {
    use super::*;

    #[test]
    fn it_finds_the_expected_pair_for_given_output() {
        let state = vec![2, 1, 0, 0, 99, 10, 0, 3];
        let pair = input_pair_for_output(&state, 30);
        assert_eq!(pair, Some((5, 7)));
    }

    #[test]
    fn it_returns_none_if_no_pair_is_found() {
        let state = vec![2, 1, 0, 3, 99];
        let pair = input_pair_for_output(&state, 30);
        assert_eq!(pair, None);
    }
}
