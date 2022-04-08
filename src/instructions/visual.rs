pub fn print(stack: &Vec<i32>) {
    println!("{}", stack.last().unwrap())
}

pub fn print_stack(stack: &Vec<i32>) {
    println!("{:?}", stack)
}