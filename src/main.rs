use nom::branch::alt;
use nom::bytes::complete::take;
use nom::bytes::complete::take_until;
use nom::character::streaming::char;
use nom::combinator::opt;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::{
    bytes::complete::{tag, take_while},
    bytes::streaming::is_not,
    character::complete::multispace0,
    combinator::value,
    error::Error,
    error::ParseError,
    multi::separated_list0,
    sequence::{delimited, pair},
    Finish, IResult,
};
use std::str;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct SqlCreate<'a> {
    pub table_name: &'a str,
    pub columns: Vec<&'a str>,
}

pub fn parse_query(input: &str) -> IResult<&str, SqlCreate> {
    let create = parse_create(input);
    match create {
        Ok((i, o)) => Ok((i, o)),
        Err(e) => Err(e),
    }
}

pub fn parse_create(input: &str) -> IResult<&str, SqlCreate> {
    let (input, _) = tag("CREATE TABLE ")(input)?;
    let (input, table_name) =
        delimited(multispace0, take_while(char::is_alphanumeric), multispace0)(input)?;
    println!("{:?}", (input, table_name));
    let (input, columns) = delimited(
        tag("("),
        separated_list0(tag(", "), take_while(char::is_alphanumeric)),
        tag(")"),
    )(input)?;
    println!("{:?}", (input, columns.clone()));
    let (input, _) = tag(";")(input)?;

    Ok((
        input,
        SqlCreate {
            table_name,
            columns,
        },
    ))
}

// #[derive(Debug)]
// pub struct Name(pub String);

// impl FromStr for Name {
//     // the error must be owned as well
//     type Err = Error<String>;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match parse_query(s).finish() {
//             Ok((_remaining, name)) => Ok(Name(name.to_string())),
//             Err(Error { input, code }) => Err(Error {
//                 input: input.to_string(),
//                 code,
//             }),
//         }
//     }
// }

fn main() {
    // parsed: Ok(Name("hello"))
    // println!("parsed: {:?}", parse_query("CREATE TABLE hello;"));

    // parsed: parsed: Err(Error { input: "", code: Tag })
    // println!("parsed: {:?}", "CREATE TABLE 123".parse::<Name>());

    // parsed: Err(Error { input: "Hello World;", code: Tag })
    // println!("parsed: {:?}", "Hello World;".parse::<Name>());

    // parsed: Ok(("", SqlCreate { table_name: "name", columns: ["column"] }))
    println!("parsed: {:?}", parse_query("CREATE TABLE name (column);"));

    // parsed: Ok(("", SqlCreate { table_name: "name", columns: ["column", "column2"] }))
    println!(
        "parsed: {:?}",
        parse_query("CREATE TABLE name (column, column2);")
    );
}
