use nom::{
    bytes::complete::{tag, take_while},
    error::Error,
    Finish, IResult,
};
use std::str;
use std::str::FromStr;
// use create::{CreateTableStatement};

// #[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
// pub struct CreateTableStatement {
//     pub table: Table,
//     pub fields: Vec<&str>,
//     pub keys: Option<Vec<&str>>,
// }s

// #[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
// pub enum SqlQuery {
//     CreateTable(CreateTableStatement),
// }

pub fn parse_query(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("CREATE TABLE ")(input)?;
    let (input, name) = take_while(|c: char| c.is_alphabetic() || c.is_numeric())(input)?;
    // let (input, _) = tag("!")(i)?;

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
    println!("parsed: {:?}", "CREATE TABLE hello".parse::<Name>());

    // parsed: Err(Error { input: "123!", code: Tag })
    println!("parsed: {:?}", "CREATE TABLE 123".parse::<Name>());
}
