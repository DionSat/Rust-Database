use crate::SqlQuery::{CreateTable, Drop, Insert};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::multispace0,
    combinator::map,
    multi::separated_list0,
    sequence::delimited,
    IResult,
};
use std::fs::{remove_file, File, OpenOptions};
use std::fmt::Write as fmt_write;
use std::io::Write;
use std::io::{self, BufRead};
use std::path::Path;
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
pub struct SqlInsert<'a> {
    pub table_name: &'a str,
    pub columns: Vec<&'a str>,
    pub values: Vec<&'a str>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SqlQuery<'a> {
    CreateTable(SqlCreate<'a>),
    Drop(SqlDrop<'a>),
    Insert(SqlInsert<'a>),
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

pub fn parse_insert(input: &str) -> IResult<&str, SqlInsert> {
    // Consume the INSERT INTO statement
    let (input, _) = tag("INSERT INTO ")(input)?;
    // Consume the table name surrounded by whitespace
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
    // Consume the whitespace if exist
    let (input, _) = multispace0(input)?;
    // Consume the VALUES statement
    let (input, _) = tag("VALUES")(input)?;
    // Consume the whitespace if exist
    let (input, _) = multispace0(input)?;
    // Consume the values in between the parenthesis
    let (input, values) = delimited(
        tag("("),
        separated_list0(tag(", "), take_while(char::is_alphanumeric)),
        tag(")"),
    )(input)?;
    println!("{:?}", (input, values.clone()));
    // Consume the semi colon ending
    let (input, _) = tag(";")(input)?;

    Ok((
        input,
        SqlInsert {
            table_name,
            columns,
            values,
        },
    ))
}

pub fn parse_query(input: &str) -> IResult<&str, SqlQuery> {
    // Parse input string and return successful parse result
    // input: string reference
    // return: Result with remaining input and SqlQuery
    alt((
        map(parse_create, SqlQuery::CreateTable),
        map(parse_drop, SqlQuery::Drop),
        map(parse_insert, SqlQuery::Insert),
    ))(input)
}

pub fn create_table(query: SqlCreate<'_>) {
    let path = format!("data/{}.csv", query.table_name);
    // Create a file
    let mut data_file = File::create(path).expect("Table creation failed. Couldn't find Path");

    // Write contents to the file
    data_file
        .write_all(query.columns.join(",").as_bytes())
        .expect("Table creation faild. Failed to write");

    println!("Created table {}", query.table_name);
}

pub fn drop_table(query: SqlDrop<'_>) {
    let path = format!("data/{}.csv", query.table_name);
    // Delete file
    remove_file(path).expect("Table could not be dropped");
    println!("Table {} is dropped", query.table_name);
}

pub fn insert_into(query: SqlInsert<'_>) {
    let path = format!("data/{}.csv", query.table_name);
    if Path::new(&path).exists() {
        let read_file = File::open(path).expect("Table could not be brought up");
        let path = format!("data/{}.csv", query.table_name);
        let mut write_file = OpenOptions::new()
        .append(true)
        .open(path)
        .expect("Table could not be brought up");
        let mut file_iter = io::BufReader::new(read_file).lines();
        let file_columns = file_iter.nth(0).unwrap();
        let mut str_columns: String = "".to_string();
        match file_columns {
            Ok(s) => str_columns = s,
            Err(e) => eprintln!("{}", e),
        }
        let parts = str_columns.split(",");
        let vec_columns = parts.collect::<Vec<&str>>();
        println!("{:?}", vec_columns);
        println!("{:?}", query.columns);
        if vec_columns == query.columns {
            write_file.write_all("\n".as_bytes()).expect("Could not insert into table");
            write_file
            .write_all(query.values.join(",").as_bytes())
            .expect("Could not insert into table");
        }
        else {
            eprintln!("Error: Columns do not match table columns. Column names either not in order or not all columns are listed or are invalid")
        }
    }
}

fn main() {
    let (_, query) = parse_query("INSERT INTO name (column, column2) VALUES (0, 1);").unwrap();
    println!("{:?}", query);
    match query {
        CreateTable(c) => create_table(c),
        Drop(d) => drop_table(d),
        Insert(i) => insert_into(i),
    }
}

#[test]
fn parses_test() {
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

    // parsed: Ok(("", SqlCreate { table_name: "name", columns: ["column", "column2"] }))
    assert_eq!(
        parse_query("INSERT INTO name (column, column2) VALUES (0, 1);"),
        Ok((
            "",
            Insert(SqlInsert {
                table_name: "name",
                columns: ["column", "column2"].to_vec(),
                values: ["0", "1"].to_vec(),
            })
        ))
    );
}
