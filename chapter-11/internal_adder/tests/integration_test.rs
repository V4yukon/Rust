use internal_adder;

#[test]

fn it_add_two() {
    assert_eq!(12, internal_adder::add(4, 8));
}
