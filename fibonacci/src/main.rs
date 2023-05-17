fn main() {
    let n = 10;
    println!("斐波那契数列前 {} 项：", n);
    for i in 0..n {
        print!("{} ", fibonacci(i));
    }
}
fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 || n == 2 {
        return 1;
    }
    let mut fib1 = 1;
    let mut fib2 = 1;
    let mut result = 0;
    for _ in 3..=n {
        result = fib1 + fib2;
        fib1 = fib2;
        fib2 = result;
    }
    return result;
}
