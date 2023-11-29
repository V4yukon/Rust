async fn foo() -> u32 { 
    10
}

fn foo1() {
    let rt = tokio::runtime::builder::new_current_thread()
        .enable_all()
        .build().unwrap();


    let num = rt.block_on(foo());


    println!("{}", num);

}

fn main() {
    foo1();

}


