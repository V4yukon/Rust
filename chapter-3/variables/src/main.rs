fn main() {
    let x = 5;
    let x = x + 5;
    {
        let x = x * 2;
        println!("the x is bound of {x} in this scope");
    }
    println!("the value of x is {x}");
    const TEST_VALUE : i32 = 60*60*3;
    println!(" The constant number is {TEST_VALUE}");
}
