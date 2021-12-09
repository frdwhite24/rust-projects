pub fn create_empty() -> Vec<u8> {
    Vec::new()
}

pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

pub fn fibonacci() -> Vec<u8> {
    let mut fibonacci_nums = vec![1, 1];
    for index in 2..5 {
        fibonacci_nums.push(fibonacci_nums[index - 2] + fibonacci_nums[index - 1]);
    }
    fibonacci_nums
}
