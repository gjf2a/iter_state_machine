use crate::{Instruction, StateMachine};
use ieee754::Ieee754;

#[derive(Copy,Clone,Debug)]
pub struct Average {
    input: Option<isize>,
    total: isize,
    count: isize,
    done: bool,
    reported: bool
}

impl Average {
    pub fn new() -> Self {
        Average { total: 0, count: 0, done: false, input: None, reported: false }
    }
}

impl StateMachine for Average {
    fn receive_input(&mut self, value: &str) -> Option<&'static str> {
        match value.parse::<isize>() {
            Ok(value) => {
                if value < 0 {
                    self.done = true;
                } else {
                    self.input = Some(value);
                }
                None
            }
            Err(_) => {
                self.input = None;
                Some("Not an int")
            }
        }
    }

    fn update(&mut self) {
        self.total += self.input.unwrap();
        self.count += 1;
        self.input = None;
    }

    fn next_instruction(&mut self) -> Option<Instruction> {
        if self.reported {
            None
        } else {
            Some(if self.done {
                self.reported = true;
                Instruction::PrintInt(self.total / self.count)
            } else if self.input.is_some() {
                Instruction::Update
            } else {
                Instruction::Input("Enter a number (-1 to quit):")
            }
            )
        }
    }
}

#[derive(Copy,Clone,Debug)]
pub struct Pi {
    sum: f64,
    denominator: f64,
    prev_sum: f64,
    tolerance: Option<f64>,
    finished: bool
}

impl Pi {
    pub fn new() -> Self {
        Pi { sum: 1.0, denominator: -3.0, prev_sum: 0.0, tolerance: None, finished: false }
    }

    pub fn value(&self) -> f64 {
        self.sum * 4.0
    }
}

impl StateMachine for Pi {
    fn receive_input(&mut self, value: &str) -> Option<&'static str> {
        match value.parse::<f64>() {
            Ok(tolerance) => {self.tolerance = Some(tolerance); None},
            Err(_) => Some("Not a float")
        }
    }

    fn update(&mut self) {
        self.prev_sum = self.sum;
        self.sum += 1.0 / self.denominator;
        self.denominator += 2.0 * self.denominator.signum();
        self.denominator *= -1.0;
    }

    fn next_instruction(&mut self) -> Option<Instruction> {
        if self.finished {
            None
        } else {
            match self.tolerance {
                None => Some(Instruction::Input("Enter tolerance:")),
                Some(tolerance) => {
                    if (self.sum - self.prev_sum).abs() < tolerance {
                        self.finished = true;
                        Some(Instruction::PrintFloat(self.value()))
                    } else {
                        Some(Instruction::Update)
                    }
                }
            }
        }
    }
}