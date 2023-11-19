#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("name is: {}", HELLO_WORLD);
}

static mut COUNTER: uew = 0;

fn add_to_count(inc: uew) {
    unsafe {
        COUNTER += inc;
        }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}


unsafe trait Foo {
    // methods go here
    //
}

un safe impl Foo for i32 {
    // method implementations go here
}



