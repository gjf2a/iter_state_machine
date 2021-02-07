#![cfg_attr(not(test), no_std)]

pub mod examples;

pub trait StateMachine {
    fn receive_input(&mut self, value: &str) -> Option<&'static str>;
    fn update(&mut self);
}

#[derive(Copy,Clone)]
pub enum Instruction {
    PrintStr(&'static str),
    PrintInt(isize),
    PrintFloat(f64),
    Input(&'static str),
    Update
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::examples::{Average, Pi};

    #[test]
    fn test_average() {
        let mut avg = Average::new();
        let inputs = vec!["5", "15", "-1"];
        let mut input = inputs.iter();
        loop {
            match avg.next() {
                None => break,
                Some(instr) => match instr {
                    Instruction::PrintStr(_) => {}
                    Instruction::PrintInt(n) => assert_eq!(n, 10),
                    Instruction::PrintFloat(_) => {}
                    Instruction::Input(_) => assert_eq!(None, avg.receive_input(input.next().unwrap())),
                    Instruction::Update => avg.update()
                }
            }
        }
    }

    #[test]
    fn test_pi() {
        let mut pi = Pi::new();
        loop {
            match pi.next() {
                None => break,
                Some(instr) => match instr {
                    Instruction::PrintStr(_) => {}
                    Instruction::PrintInt(_) => {}
                    Instruction::PrintFloat(answer) => assert_eq!(answer, 3.1611986129870506),
                    Instruction::Input(_) => assert_eq!(None, pi.receive_input("0.01")),
                    Instruction::Update => pi.update()
                }
            }
        }
    }
}
