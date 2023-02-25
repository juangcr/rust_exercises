fn main() {
    let i = [
        CalculatorInput::Value(2),
        // CalculatorInput::Value(3),
        CalculatorInput::Add,
        CalculatorInput::Value(1),
        CalculatorInput::Subtract,
    ];

    println!("{:?}", evaluate(&i));
}

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = Vec::new();
    for input in inputs.iter() { 
        let check = match input {
            CalculatorInput::Add => stack_op(&mut stack, add),
            CalculatorInput::Subtract => stack_op(&mut stack, sub),
            CalculatorInput::Multiply => stack_op(&mut stack, prod),
            CalculatorInput::Divide => stack_op(&mut stack, div),
            CalculatorInput::Value(n) => Ok(stack.push(*n)), 
        };
        match check {
            Ok(()) => (),
            Err(()) => return None,
        }
        println!("{:?}", stack);
    }
    if stack.len() == 1 { return stack.first().copied(); } else { return None; }
}

fn stack_op(stack: &mut Vec<i32>, func: fn(i32, i32) -> i32) -> Result<(), ()> {
    if stack.len() < 2 {
        Err(())
    } else {
        let y: i32 = stack.pop().unwrap();
        let x: i32 = stack.pop().unwrap();
        stack.push(func(x, y));
        Ok(())
    }
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn sub(x: i32, y: i32) -> i32 {
    x - y
}

fn prod(x: i32, y: i32) -> i32 {
    x * y
}

fn div(x: i32, y: i32) -> i32 {
    if y == 0 { panic!("Division by zero.") }
    x / y
}
