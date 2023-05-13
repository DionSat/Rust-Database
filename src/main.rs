use nom::branch::alt;
use nom::{
    bytes::complete::{tag, take_while},
    error::Error,
    Finish, IResult,
};
use std::str;
use std::str::FromStr;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct SqlQuery {
    pub table_name: String,
}

pub fn parse_query(input: &str) -> IResult<&str, &str> {
    Ok(parse_create(input)?)
}

pub fn parse_create(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("CREATE TABLE ")(input)?;
    let (input, name) = take_while(|c: char| c.is_alphabetic() || c.is_numeric())(input)?;
    let (input, _) = tag(";")(input)?;

    Ok((input, name))
}

#[derive(Debug)]
pub struct Name(pub String);

impl FromStr for Name {
    // the error must be owned as well
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_query(s).finish() {
            Ok((_remaining, name)) => Ok(Name(name.to_string())),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

fn main() {
    // parsed: Ok(Name("hello"))
    println!("parsed: {:?}", "CREATE TABLE hello;".parse::<Name>());

    // parsed: parsed: Err(Error { input: "", code: Tag })
    println!("parsed: {:?}", "CREATE TABLE 123".parse::<Name>());

    // parsed: Err(Error { input: "Hello World;", code: Tag })
    println!("parsed: {:?}", "Hello World;".parse::<Name>());
}
