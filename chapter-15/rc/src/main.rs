enum List{
    Cons( i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

use std::rc::Rc;


fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("The count of a is a = {}",Rc::strong_count(&a));
    let b = Cons(5, Rc::clone(&a));
    println!("The count of b is b = {}",Rc::strong_count(&a));
    {
        let c = Cons(10, Rc::clone(&a));
        println!("The count of c is c = {}",Rc::strong_count(&a));
    }


    println!("The count of after c is c = {}",Rc::strong_count(&a));


}




