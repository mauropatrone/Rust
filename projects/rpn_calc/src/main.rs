#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    for input in inputs {
        match input {
            CalculatorInput::Add =>  {
                if let Err(()) = rpn_add(&mut stack) {
                    return None
                }
            },
            CalculatorInput::Subtract =>  {
                if let Err(()) = rpn_sub(&mut stack) {
                    return None
                }
            },
            CalculatorInput::Multiply => {
                if let Err(()) = rpn_mul(&mut stack) {
                    return None
                }
            },
            CalculatorInput::Divide => {
                if let Err(()) = rpn_div(&mut stack) {
                    return None
                }
            },
            CalculatorInput::Value(x) => stack.push(*x),
        }
    }
    if stack.len() > 1 {
        return None
    }
    stack.pop()
}

pub fn rpn_add(stack: &mut Vec<i32>) -> Result<(),()> {
    let n2 = stack.pop().ok_or(())?;
    let n1 = stack.pop().ok_or(())?;
    stack.push(n1 + n2);
    Ok(())
}

pub fn rpn_sub(stack: &mut Vec<i32>) -> Result<(),()> {
    let n2 = stack.pop().ok_or(())?;
    let n1 = stack.pop().ok_or(())?;
    stack.push(n1 - n2);
    Ok(())
}

pub fn rpn_mul(stack: &mut Vec<i32>) -> Result<(),()> {
    let n2 = stack.pop().ok_or(())?;
    let n1 = stack.pop().ok_or(())?;
    stack.push(n1 * n2);
    Ok(())
}

pub fn rpn_div(stack: &mut Vec<i32>) -> Result<(),()> {
    let n2 = stack.pop().ok_or(())?;
    if n2 == 0 {
        return Err(())
    }
    let n1 = stack.pop().ok_or(())?;
    stack.push(n1 / n2);
    Ok(())
}

#[test]
fn test_evaluate() {
    let mut rpn: Vec<CalculatorInput> = Vec::new();
    rpn.push(CalculatorInput::Value(4));
    rpn.push(CalculatorInput::Value(8));
    rpn.push(CalculatorInput::Add);
    rpn.push(CalculatorInput::Value(7));
    rpn.push(CalculatorInput::Value(5));
    rpn.push(CalculatorInput::Subtract);
    rpn.push(CalculatorInput::Divide);
    assert_eq!(evaluate(&rpn),Some(6));
}

#[test]
fn test_add() {
    let mut stack: Vec<i32> = Vec::new();
    stack.push(3);
    stack.push(2);
    assert_eq!(rpn_add(&mut stack),Ok(()));
    assert_eq!(stack[0],5);
    assert_eq!(rpn_add(&mut stack),Err(()));
}

#[test]
fn test_sub() {
    let mut stack: Vec<i32> = Vec::new();
    stack.push(1);
    stack.push(2);
    assert_eq!(rpn_sub(&mut stack),Ok(()));
    assert_eq!(stack[0],-1);
    assert_eq!(rpn_sub(&mut stack),Err(()));
}

#[test]
fn test_mul() {
    let mut stack: Vec<i32> = Vec::new();
    stack.push(-1);
    stack.push(-2);
    assert_eq!(rpn_mul(&mut stack),Ok(()));
    assert_eq!(stack[0],2);
    assert_eq!(rpn_mul(&mut stack),Err(()));
}

#[test]
fn test_div() {
    let mut stack: Vec<i32> = Vec::new();
    stack.push(-10);
    stack.push(3);
    assert_eq!(rpn_div(&mut stack),Ok(()));
    assert_eq!(stack[0],-3);
    assert_eq!(rpn_div(&mut stack),Err(()));
    stack.push(0);
    assert_eq!(rpn_div(&mut stack),Err(()));
}

fn main() {
    println!("Hello, world!");
}
