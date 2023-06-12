use crate::SqlQuery::{CreateTable, Drop, Insert, Selection};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::multispace0,
    combinator::{map, opt, map_parser},
    multi::separated_list0,
    sequence::delimited,
    bytes::streaming::{is_not, take_until},
    character::streaming::space0,
    IResult,
};
use std::fs::{remove_file, File, OpenOptions};
use std::io::Write;
use nom::Parser;
use std::io::{self, BufRead};
use std::path::Path;
use std::str;

// SQL create table structure
// table_name: name of the table given for creation
// columns: the columns names given for creation
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SqlCreate<'a> {
    pub table_name: &'a str,
    pub columns: Vec<&'a str>,
}

// SQL drop table structure
// table_name: name of the table given for deletion
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SqlDrop<'a> {
    pub table_name: &'a str,
}

// SQL insert structure
// table_name: name of the table given for adding to an existing table
// columns: the columns names given for adding to an existing table
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SqlInsert<'a> {
    pub table_name: &'a str,
    pub columns: Vec<&'a str>,
    pub values: Vec<&'a str>,
}

// SQL select table structure
// columns: the columns names given for a table
// table_name: the table name given for a selected table
// all_columns: the asterisk to represent selecting all columns
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SqlSelect<'a> {
    pub columns: Option<Vec<&'a str>>,
    pub table_name: &'a str,
    pub all_columns: Option<&'a str>,
}

// SQL query structure
// CreateTable: create table type for sql create
// Drop: drop table type for sql drop
// Insert: insert type for sql insert
// Selection: select type for sql select
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SqlQuery<'a> {
    CreateTable(SqlCreate<'a>),
    Drop(SqlDrop<'a>),
    Insert(SqlInsert<'a>),
    Selection(SqlSelect<'a>),
}

// Parse the SQL CREATE [TABLE NAME] (column, column2, etc, ...)
// input: reference to sql create string
// returns: a IResult with the remaining input and the SqlCreate struct
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
        separated_list0(tag(",").and(space0), take_while(char::is_alphanumeric)),
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

// Parse the SQL DROP [TABLE NAME]
// input: reference to sql drop string
// returns: a IResult with the remaining input and the SqlDrop struct
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

// Parse the SQL INSERT INTO [TABLE NAME] (column, column2, etc, ...) VALUES (value, value2, etc, ...)
// input: reference to sql insert string
// returns: a IResult with the remaining input and the SqlInsert struct
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
        separated_list0(tag(",").and(space0), take_while(char::is_alphanumeric)),
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
        separated_list0(tag(",").and(space0), take_while(char::is_alphanumeric)),
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

// Parse the SQL SELECT (column, column2, etc, ...) FROM [TABLE NAME]
// input: reference to sql select string
// returns: a IResult with the remaining input and the SqlCSelect struct
pub fn parse_select(input: &str) -> IResult<&str, SqlSelect> {
    // Consume the SELECT statement and whitespace
    let (input, _) = tag("SELECT ")(input)?;
    // Consume the whitespace if exist
    let (input, _) = multispace0(input)?;
    // Consume column names if exist
    let (input, all_columns) = opt(tag("*"))(input)?;
    println!("{:?}", (input, all_columns));
    let (input, columns) = opt(alt((
        delimited(
            tag("("),
            separated_list0(tag(",").and(space0), take_while(char::is_alphanumeric)),
            tag(")"),
        ),
        separated_list0(tag(",").and(space0) , take_while(char::is_alphanumeric)),
    )))(input)?;
    println!("{:?}", (input, columns.clone()));
    // Consume the whitespace if exist
    let (input, _) = multispace0(input)?;
    // Consume the FROM statement
    let (input, _) = tag("FROM ")(input)?;
    // Consume the whitespace if exist
    let (input, _) = multispace0(input)?;
    // Consume the table name
    let (input, table_name) =
        delimited(multispace0, take_while(char::is_alphanumeric), multispace0)(input)?;
    println!("{:?}", (input, columns.clone()));
    // Consume the semi colon ending
    let (input, _) = tag(";")(input)?;

    Ok((
        input,
        SqlSelect {
            columns,
            table_name,
            all_columns,
        },
    ))
}

// Parse the SQL string and return the successful parser
// input: reference to sql string
// returns: a IResult with the remaining input and the SqlQuery struct
pub fn parse_query(input: &str) -> IResult<&str, SqlQuery> {
    alt((
        map(parse_create, SqlQuery::CreateTable),
        map(parse_drop, SqlQuery::Drop),
        map(parse_insert, SqlQuery::Insert),
        map(parse_select, SqlQuery::Selection),
    ))(input)
}

// Function to create a table as a csv
// query: the SqlCreate struct holding the necessary information
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

// Function to drop a table and delete the csv
// query: the SqlDrop struct holding the necessary information
pub fn drop_table(query: SqlDrop<'_>) {
    let path = format!("data/{}.csv", query.table_name);
    // Delete file
    remove_file(path).expect("Table could not be dropped");
    println!("Table {} is dropped", query.table_name);
}

// Function to insert into a table and the csv
// query: the SqlInsert struct holding the necessary information
pub fn insert_into(query: SqlInsert<'_>) {
    // Create the datapath string
    let path = format!("data/{}.csv", query.table_name);
    // Check if the path or table exists first
    if Path::new(&path).exists() {
        // Open the table csv for reading
        let read_file = File::open(path).expect("Table could not be brought up");
        // Create the datapath string
        let path = format!("data/{}.csv", query.table_name);
        // Open the table csv for appending
        let mut write_file = OpenOptions::new()
            .append(true)
            .open(path)
            .expect("Table could not be brought up");
        let mut file_iter = io::BufReader::new(read_file).lines();
        let file_columns = file_iter.next().unwrap();
        let mut str_columns: String = "".to_string();
        match file_columns {
            Ok(s) => str_columns = s,
            Err(e) => eprintln!("{}", e),
        }
        let parts = str_columns.split(',');
        let vec_columns = parts.collect::<Vec<&str>>();
        println!("{:?}", vec_columns);
        println!("{:?}", query.columns);
        if vec_columns == query.columns {
            write_file
                .write_all("\n".as_bytes())
                .expect("Could not insert into table");
            write_file
                .write_all(query.values.join(",").as_bytes())
                .expect("Could not insert into table");
        } else {
            eprintln!("Error: Columns do not match table columns. Column names either not in order or not all columns are listed or are invalid")
        }
    }
}

pub fn select_table(_query: SqlSelect<'_>) {
    todo!();
}

fn main() {
    let (_, query) = parse_query("INSERT INTO name (column, column2) VALUES (0, 1);").unwrap();
    println!("{:?}", query);
    match query {
        CreateTable(c) => create_table(c),
        Drop(d) => drop_table(d),
        Insert(i) => insert_into(i),
        Selection(s) => select_table(s),
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

    // Create file for testing
    // create_table(SqlCreate {
    //             table_name: "name",
    //             columns: ["column", "column2"].to_vec()
    //         });

    // parsed: Ok(("", Drop(SqlDrop { table_name: "name" })))
    assert_eq!(
        parse_query("DROP TABLE name;"),
        Ok(("", Drop(SqlDrop { table_name: "name" })))
    );

    // parsed: Ok(("", SqlInsert { table_name: "name", columns: ["column", "column2"], values: ["0", "1"] }))
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

    // parsed: Ok(("", SqlInsert { table_name: "name", columns: ["column", "column2"], values: ["0", "1"] }))
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

    // parsed: Ok(("", Selection(SqlSelect { columns: Some([""]), table_name: "name", all_columns: Some("*") })))
    assert_eq!(
        parse_query("SELECT * FROM name;"),
        Ok((
            "",
            Selection(SqlSelect {
                table_name: "name",
                columns: Some([""].to_vec()),
                all_columns: Some("*"),
            })
        ))
    );

    // parsed: Ok(("", Selection(SqlSelect { columns: Some([""]), table_name: "name", all_columns: Some("*") })))
    assert_eq!(
        parse_query("SELECT column,        column2 FROM name;"),
        Ok((
            "",
            Selection(SqlSelect {
                table_name: "name",
                columns: Some(["column","column2"].to_vec()),
                all_columns: None,
            })
        ))
    );

}
