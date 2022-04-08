pub fn get_arg(stack: &mut Vec<i32>, n: usize) {
    stack.push(stack[n])
}

pub fn set_arg(stack: &mut Vec<i32>, n: usize, val: i32) {
    stack[n] = val
}