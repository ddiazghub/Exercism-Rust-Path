use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub struct RPNCalculator(Vec<i32>);

impl RPNCalculator {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn operate(&mut self, operation: fn(i32, i32) -> i32) -> Option<()> {
        if self.0.len() < 2 {
            return None;
        }

        let (b, a) = (self.0.pop().unwrap(), self.0.pop().unwrap());
        self.0.push(operation(a, b));
        Some(())
    }

    pub fn evaluate(&mut self, inputs: &[CalculatorInput]) -> Option<i32> {
        for &input in inputs.iter() {
            match input {
                CalculatorInput::Value(value) => self.0.push(value),
                CalculatorInput::Add => self.operate(i32::add)?,
                CalculatorInput::Subtract => self.operate(i32::sub)?,
                CalculatorInput::Multiply => self.operate(i32::mul)?,
                CalculatorInput::Divide => self.operate(i32::div)?,
            }
        }

        if self.0.len() == 1 {
            Some(self.0.pop().unwrap())
        } else {
            None
        }
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut calculator = RPNCalculator::new();
    calculator.evaluate(inputs)
}
