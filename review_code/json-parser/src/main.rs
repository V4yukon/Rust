use indexmap::IndexMap;
use nom::IResult;
use nom::number::complete::double;
use nom::Parser;

pub fn map<I, O1, O2, E, F, G>(mut parser: F, mut f: G) -> impl FnMut(I) -> IResult<I, O2, E>
where
  F: Parser<I, O1, E>,
  G: FnMut(O1) -> O2,
{
  move |input: I| {
    let (input, o1) = parser.parse(input)?;
    Ok((input, f(o1)))
  }
}


fn parse_number(json: &str) -> IResult<&str, JsonNode> {
    map(double, JsonNode::Number)(json)
}
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

#[derive(Debug, PartialEq, Clone)]
enum JsonNode<'a> {
    Object(Box<IndexMap<&'a str, JsonNode<'a>>>),
    Array(Vec<JsonNode<'a>>),
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Null,
}

fn main() {
    // let num_1 = parse_number("12.4");
    // println!("{:?}", &num_1);
    // print_type_of(&num_1);


    let list = vec![1, 4, 7, 2, 5, 8];

    let only_borrows =  || println!("From closure: {:?}", list);

    println!("{:?}", list);

    only_borrows();

    print_type_of(&only_borrows);
}
