use std::io;
fn main() {
    let mut custom_number = String::new();
    println!("Please input a number");
    io::stdin()
        .read_line(&mut custom_number)
        .expect("Failed to read line");
    let custom_number: i32 = custom_number.trim().parse().expect("Please type a number!");
    let mut v = Vec::new();

    //for i in 0..custom_number{
      //  v.push(febonacci_squ(i));
        //println!("{}",febonacci_squ(i));
    

    println!("This febonacci squence is : {:?}", febonacci(custom_number));


    //println!("Hello, world!");
}

fn febonacci_squ(n: i32) -> Vec<u64> {
    if n == 0{
        return Vec![0];
    }
    if n == 1 || n == 2 {
        return Vec![1];
    }
    let mut  fib1 = 1;
    let mut fib2 = 1;
    let mut result = 0;
    let mut v = Vec::new();

    for _ in 3..=n {
        result = fib1 + fib2;
        fib1 = fib2;
        fib2 = result;
        v.push(result);
    }
    return v;
}

