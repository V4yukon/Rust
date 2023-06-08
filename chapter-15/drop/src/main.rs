struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Drop CustomSmartPointer with data '{}'", self.data);
    }
}


fn main() {
    let c = CustomSmartPointer{
        data: String::from("oops"),
    };

    drop(c);

    let d = CustomSmartPointer{
        data: String::from("well"),
    };

    println!("CustomSmartPointer created!");
}

