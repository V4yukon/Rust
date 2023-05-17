fn largest<T>(list: &[T]) -> &T {
    let mut largest=&list[0];

    for item in list {
        if item > largest {
            largest=item;
        }
    }

    largest
}


fn main() {
    let num_list1 = vec![1,8,9,20,5];

    println!("the largest is :{}", largest(&num_list1));


}
