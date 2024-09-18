use std::io;

fn main() {
    println!("Welcome to the fibonacci sequence!");
    println!("Input desired list of list: ");

    //Declares list length variable
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    //Checks input is a valid number
    let length_of_fib: usize = input.trim().parse().expect("Please type a number!");

    //Creates vector to length of users input
    let mut fibonacci_sequence: Vec<usize> = vec![1; length_of_fib];
    for i in 2..length_of_fib {
    
            fibonacci_sequence[i] = fibonacci_sequence[i - 2] + fibonacci_sequence[i - 1];
    }
    println!("Fibonacci sequence of length {} is: {:?}", length_of_fib, fibonacci_sequence);
    println!("Fibonacci sequence: {:?}", fibonacci_sequence); 
}
