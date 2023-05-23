use crate::SqlQuery::{CreateTable, Drop};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::multispace0,
    combinator::map,
    multi::separated_list0,
    sequence::delimited,
    IResult,
    Err::{Error, Failure, Incomplete},
};
use std::fs::{remove_file, File};
use std::io::Write;
use std::str;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SqlCreate<'a> {
    pub table_name: &'a str,
    pub columns: Vec<&'a str>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SqlDrop<'a> {
    pub table_name: &'a str,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SqlQuery<'a> {
    CreateTable(SqlCreate<'a>),
    Drop(SqlDrop<'a>),
}

pub fn parse_create(input: &str) -> IResult<&str, SqlCreate> {
    // Consume the CREATE TABLE statement
    let (input, _) = tag("CREATE TABLE ")(input)?;
    // Consume the whitespaces surrounding the table name
    let (input, table_name) =
        delimited(multispace0, take_while(char::is_alphanumeric), multispace0)(input)?;
    println!("{:?}", (input, table_name));
    // Consume the columns in between parenthesis
    let (input, columns) = delimited(
        tag("("),
        separated_list0(tag(", "), take_while(char::is_alphanumeric)),
        tag(")"),
    )(input)?;
    println!("{:?}", (input, columns.clone()));
    // Consume the semi colon ending
    let (input, _) = tag(";")(input)?;

    Ok((
        input,
        SqlCreate {
            table_name,
            columns,
        },
    ))
}

pub fn parse_drop(input: &str) -> IResult<&str, SqlDrop> {
    // Consume the DROP TABLE statement
    let (input, _) = tag("DROP TABLE ")(input)?;
    // Consume the whitespace if exist
    let (input, _) = multispace0(input)?;
    // Consume the table name
    let (input, table_name) = take_while(char::is_alphanumeric)(input)?;
    println!("{:?}", (input, table_name));
    // Consume the semi colon ending
    let (input, _) = tag(";")(input)?;

    Ok((input, SqlDrop { table_name }))
}

pub fn parse_query(input: &str) -> IResult<&str, SqlQuery> {
    // Parse input string and return successful parse result
    // input: string reference
    // return: Result with remaining input and SqlQuery
    alt((
        map(parse_create, |c| SqlQuery::CreateTable(c)),
        map(parse_drop, |d| SqlQuery::Drop(d)),
    ))(input)
}  

pub fn create_table(query: SqlCreate<'_>) {
    let path = format!("data/{}.csv", query.table_name);
    // Create a file
    let mut data_file = File::create(path).expect("Table creation failed. Couldn't find Path");

    // Write contents to the file
    data_file
        .write(query.columns.join(",").as_bytes())
        .expect("Table creation faild. Failed to write");

    println!("Created table {}", query.table_name);
}

pub fn drop_table(query: SqlDrop<'_>) {
    let path = format!("data/{}.csv", query.table_name);
    // Delete file
    remove_file(path).expect("Table could not be dropped");
    println!("Table {} is dropped", query.table_name);
}

fn main() {
    let (_, query) = parse_query("DROP TABLE name;").unwrap();
    println!("{:?}", query);
    match query {
        CreateTable(c) => create_table(c),
        Drop(d) => drop_table(d),
    }
}

#[test]
fn query_test() {
    // parsed: Ok(Name("hello"))
    // println!("parsed: {:?}", parse_query("CREATE TABLE hello;"));

    // parsed: parsed: Err(Error { input: "", code: Tag })
    // println!("parsed: {:?}", "CREATE TABLE 123".parse::<Name>());

    // parsed: Err(Error { input: "Hello World;", code: Tag })
    // println!("parsed: {:?}", "Hello World;".parse::<Name>());

    // parsed: Ok(("", SqlCreate { table_name: "name", columns: ["column"] }))
    assert_eq!(
        parse_query("CREATE TABLE name (column);"),
        Ok((
            "",
            CreateTable(SqlCreate {
                table_name: "name",
                columns: ["column"].to_vec()
            })
        ))
    );

    // parsed: Ok(("", SqlCreate { table_name: "name", columns: ["column", "column2"] }))
    assert_eq!(
        parse_query("CREATE TABLE name (column, column2);"),
        Ok((
            "",
            CreateTable(SqlCreate {
                table_name: "name",
                columns: ["column", "column2"].to_vec()
            })
        ))
    );

    // parsed: Ok(("", Drop(SqlDrop { table_name: "name" })))
    assert_eq!(
        parse_query("DROP TABLE name;"),
        Ok(("", Drop(SqlDrop { table_name: "name" })))
    );
}
