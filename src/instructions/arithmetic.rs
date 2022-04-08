pub fn add(stack: &mut Vec<i32>) {
    let a = stack.pop();
    let b = stack.pop();
    let res = a.unwrap() + b.unwrap();
    stack.push(res)
}

pub fn sub(stack: &mut Vec<i32>) {
    let a = stack.pop();
    let b = stack.pop();
    let res = a.unwrap() - b.unwrap();
    stack.push(res)
}

pub fn mul(stack: &mut Vec<i32>) {
    let a = stack.pop();
    let b = stack.pop();
    let res = a.unwrap() * b.unwrap();
    stack.push(res)
}

pub fn div(stack: &mut Vec<i32>) {
    let a = stack.pop();
    let b = stack.pop();
    let res = a.unwrap() / b.unwrap();
    stack.push(res)
}