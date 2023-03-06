#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {

    if inputs.is_empty() {return None};

    let mut stack: Vec<i32> = Vec::new();

    for input in inputs {
        match input {
            CalculatorInput::Value(x) => stack.push(x.to_owned()),
            CalculatorInput::Subtract if len_geq_2(&stack) => sub(&mut stack),
            CalculatorInput::Multiply if len_geq_2(&stack) => mul(&mut stack),
            CalculatorInput::Divide if len_geq_2(&stack) => div(&mut stack),
            CalculatorInput::Add if len_geq_2(&stack) => add(&mut stack),
            _ => return None,
        }
    }

    if stack.len() > 1 { return None;} 
    return Some(stack.pop().unwrap());

   
}

fn sub(stack: &mut Vec<i32>){
    let x1 = stack.pop().unwrap();
    let x2 = stack.pop().unwrap();
    let sub = x2 - x1;
    stack.push(sub);
}

fn mul(stack: &mut Vec<i32>){
    let mul = stack.pop().unwrap() * stack.pop().unwrap();
    stack.push(mul);
}

fn div(stack: &mut Vec<i32>){
    let x1 = stack.pop().unwrap();
    let x2 = stack.pop().unwrap();
    let div = x2 / x1;
    stack.push(div);
}

fn add(stack: &mut Vec<i32>){
    let add = stack.pop().unwrap() + stack.pop().unwrap();
    stack.push(add);
}

fn len_geq_2(stack: &Vec<i32>) -> bool {
    return stack.len() >= 2;
}