#[allow(dead_code)]

use indexmap::map::IndexMap;
use nom::multi::separated_list0;
use nom::{
    self,
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::multispace0,
    combinator::{map, value},
    number::complete::double,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, PartialEq, Clone)]
enum JsonNode<'a> {
    Object(Box<IndexMap<&'a str, JsonNode<'a>>>),
    Array(Vec<JsonNode<'a>>),
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Null,
}

///Parsing a full json will result in a JsonNode and should consume all the input
fn parse_json(json: &str) -> IResult<&str, JsonNode> {
    alt((
        parse_object,
        parse_array,
        parse_number,
        parse_string,
        parse_boolean,
        parse_null,
    ))(json)
}


fn parse_object(json: &str) -> IResult<&str, JsonNode> {
    map(
        delimited(
            tag("{"),
            separated_list0(
                tag(","),
                separated_pair(
                    delimited(multispace0, parse_string_inner, multispace0),
                    tag(":"),
                    delimited(multispace0, parse_json, multispace0),
                ),
            ),
            tag("}"),
         ),
         |v| JsonNode::Object(Box::new(v.into_iter().collect())),
    )(json)
}


fn parse_array(json: &str) -> IResult<&str, JsonNode> {
    map(
        delimited(
            tag("["),
            separated_list0(delimited(multispace0, tag(","), multispace0), parse_json),
            tag("]"),
        ),
        JsonNode::Array,
    )(json)
}

fn parse_number(json: &str) -> IResult<&str, JsonNode> {
    map(double, JsonNode::Number)(json)
}

fn parse_string_inner(json: &str) -> IResult<&str, &str> {
    delimited(tag("\""), take_until("\""), tag("\""))(json)
}

fn parse_string(json: &str) -> IResult<&str, JsonNode> {
    map(parse_string_inner, JsonNode::String)(json)
}

fn parse_boolean(json: &str) -> IResult<&str, JsonNode> {
    alt((
        value(JsonNode::Boolean(true), tag("true")),
        value(JsonNode::Boolean(false), tag("false")),
    ))(json)
}

fn parse_null(json: &str) -> IResult<&str, JsonNode> {
    value(JsonNode::Null, tag("null"))(json)
}

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;
    use nom::error::make_error;

    use crate::{
        parse_array, parse_boolean, parse_json, parse_number, parse_object, parse_string, parse_null, JsonNode,
    };

    #[test]
    fn can_parse_null() {
        assert_eq!(Ok(("", JsonNode::Null)), parse_null("null"));

        assert_eq!(
            parse_null("Something"),
            Err(nom::Err::Error(make_error(
                "Something",
                nom::error::ErrorKind::Tag
            )))
        );
    }

    #[test]
    fn can_parse_boolean() {
        assert_eq!(Ok(("", JsonNode::Boolean(true))), parse_boolean("true"));
        assert_eq!(Ok(("", JsonNode::Boolean(false))), parse_boolean("false"));

        assert_eq!(
            parse_boolean("Something"),
            Err(nom::Err::Error(make_error(
                "Something",
                nom::error::ErrorKind::Tag
            )))
        );
    }

    #[test]
    fn can_parse_number() {
        assert_eq!(Ok(("", JsonNode::Number(42f64))), parse_number("42"));
        assert_eq!(Ok(("", JsonNode::Number(4.2f64))), parse_number("4.2"));
        assert_eq!(Ok(("", JsonNode::Number(1.3e4f64))), parse_number("1.3e4"));
        assert_eq!(Ok(("", JsonNode::Number(0.42f64))), parse_number("0.42"));


        assert_eq!(
            parse_string("Something"),
            Err(nom::Err::Error(make_error(
                "Something",
                nom::error::ErrorKind::Tag
            )))
        );
    }

    #[test]
    fn can_parse_array() {
        assert_eq!(Ok(("", JsonNode::Array(Vec::new()))), parse_array("[]"));
        assert_eq!(
            Ok(("", JsonNode::Array(vec![JsonNode::Boolean(true)]))),
            parse_array("[true]")
        );

        assert_eq!(
            Ok((
                "",
            JsonNode::Array(vec![
                JsonNode::Boolean(false),
                JsonNode::Null,
                JsonNode::Boolean(false)
            ])
        )),
        parse_array("[false, null, false]")
        );


        assert_eq!(
            parse_array("Something"),
            Err(nom::Err::Error(make_error(
                "Something",
                nom::error::ErrorKind::Tag
            )))
        );
    }

    #[test]
    fn can_parse_object() {
        assert_eq!(
            Ok(("", JsonNode::Object(Box::new(IndexMap::new())))),
            parse_object("{}")
        );

        assert_eq!(
            Ok((
                "",
                JsonNode::Object(Box::new(
                    vec![("b", JsonNode::Boolean(false))].into_iter().collect()
                )
            ))),
            parse_object("{\"b\": false}")
        );


        assert_eq!(
            Ok((
                "",
                JsonNode::Object(Box::new(
                    vec![("a", JsonNode::String("x")), ("b", JsonNode::Boolean(true)),]
                    .into_iter()
                    .collect()
                ))
            )),
            parse_object("{\"a\": \"x\", \"b\": true}")
        );

        assert_eq!(
            parse_object("Something"),
            Err(nom::Err::Error(make_error(
                "Something",
                nom::error::ErrorKind::Tag
            )))
        );
    }


}
