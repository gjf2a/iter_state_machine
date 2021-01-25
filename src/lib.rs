#![cfg_attr(not(test), no_std)]

mod examples;

pub enum Instruction<T> {
    PrintStr(&'static str),
    PrintInt(isize),
    PrintFloat(f64),
    Input(&'static str, fn(&mut T, &str) -> Option<&'static str>),
    Update(fn(&mut T))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::examples::Average;

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
                    Instruction::Input(_, f) => assert_eq!(None, f(&mut avg, input.next().unwrap())),
                    Instruction::Update(f) => f(&mut avg)
                }
            }
        }
    }
}
