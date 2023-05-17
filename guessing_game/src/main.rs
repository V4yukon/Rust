use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    //println!("Guessing a secrete number");


    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    println!("Secret Number is : {secret_number}");
    loop {
    println!("Guessing a number:");
    println!("Please input your guessing number:");
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    let guess : i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            };
    println!("Your guess: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => {
            println!("You win");
            break;
        
        }
    }
 }
}
