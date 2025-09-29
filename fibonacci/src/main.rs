fn main() {
    println!("Hello, world!");
    // A loop to print finbonacci numbers up to 20
    let n = 50;
    let mut memo = vec![0; (n + 1) as usize];
    for i in 0..=n {
        // println!("Fibonacci Recursive({}) = {}", i, fibonacci_recursive(i));
        println!("Fibonacci({}) = {}", i, fibonacci(i, &mut memo));
    }

}

// A function to calculate the nth Finbonacci number using recursion
fn fibonacci_recursive(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

// A function to calculate the nth fibonacci number using recursion and memoization
fn fibonacci(n: u64, memo: &mut Vec<u64>) -> u64 {
    if n <= 1 {
        return n;
    }
    if memo[n as usize] != 0 {
        return memo[n as usize];
    }
    memo[n as usize] = fibonacci(n - 1, memo) + fibonacci(n - 2, memo);
    memo[n as usize]
}