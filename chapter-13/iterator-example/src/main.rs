#[derive(PartialEq, Debug)]

struct Shoe{
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]


mod tests{
    use super::*;

    #[test]

    fn filter_by_size() {
        let shoes = vec![
            Shoe{
                size: 42,
                style: String::from("sneaker")
            },
            Shoe{
                size: 44,
                style: String::from("nike")
            },
            Shoe{
                size: 42,
                style: String::from("adidas")
            },
        ];


        let in_my_size = shoes_in_size(shoes, 42);

        assert_eq!(
            in_my_size,
            vec![
            Shoe{
                size: 42,
                style: String::from("sneaker")
            },
            Shoe{
                size: 42,
                style: String::from("adidas")
            },
            ]);
    }
}
