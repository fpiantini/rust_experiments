use std::io;

fn main() {
    println!("Enter a positive integer n, I will generate the n-th Fibonacci number");

    let mut index = String::new();
    io::stdin().read_line(&mut index)
        .expect("Failed to read line");

    let index: u32 = index.trim().parse()
        .expect("Failed to convert number, please enter a floating point number");

    if index > 0 {
        let fibonacci_number = fibonacci(index-1);
        println!("the {}-th Fibonacci number is: {}", index, fibonacci_number);

    }
    else {
        println!("The index shall be greater than zero");
    }
}

fn fibonacci(index: u32) -> u64 {
    if index == 0 || index == 1 {
        return 1;
    }
    return fibonacci(index-1) + fibonacci(index-2);
}


