pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    if inputs.len() == 0 {
        return None;
    }

    let mut stack: Vec<i32> = Vec::new();
    for input in inputs {
        match input {
            CalculatorInput::Add => match (stack.pop(), stack.pop()) {
                (Some(right), Some(left)) => stack.push(left + right),
                _ => return None,
            },
            CalculatorInput::Subtract => match (stack.pop(), stack.pop()) {
                (Some(right), Some(left)) => stack.push(left - right),
                _ => return None,
            },
            CalculatorInput::Multiply => match (stack.pop(), stack.pop()) {
                (Some(right), Some(left)) => stack.push(left * right),
                _ => return None,
            },
            CalculatorInput::Divide => match (stack.pop(), stack.pop()) {
                (Some(right), Some(left)) => stack.push(left / right),
                _ => return None,
            },
            CalculatorInput::Value(val) => {
                stack.push(*val);
            }
        }
    }
    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}
