use std::io;
fn factorial(n: u32, acc: u32) -> u32 {
    if n == 0 {
        return acc
    } else {
        return factorial(n - 1, acc * n) 
    }// 使用尾递归解决栈溢出问题
}

fn main() {

    //input a number ,caculate 
    println!("Please input a number: ");
    let mut  n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read");

    let num = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid Input");
            return
        }
    };
    let  result = factorial(num, 1);
   // result.push(factorial(num));

    println!("The {} is {}", num , result);

}
    

