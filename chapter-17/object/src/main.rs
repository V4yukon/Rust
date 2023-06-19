use object::AveragedCollection;
fn main() {
    let mut m = AveragedCollection {
        list: vec![1, 10, 8, 19],
        average: 9.5,
    };

    m.add(5);

    println!("{:?}", m.average());

}
