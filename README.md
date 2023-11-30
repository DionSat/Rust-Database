

# Rust-Database
By: Dion Satcher

## Description
This is small DB for postgres SQL in Rust. In other words this program has the ability to parse postgres SQL and apply SQL to data. Its intended to be small toy example with only some of SQL statements implemented. 
To Run my program simply run
```cargo run main``` in the ```src``` directory of the repo
To run the tests run
```cargo test```
The tests were essentially on the parser itself. I tested how the parser handled different formatted input and if the results were valid. I spent most of time actually on the parser than the actual implementation of the SQL functionality so I focused my test more on that. Because the parser should be able to pick up any malformatted input before its sent to the SQL functions. At least thats my thought process.

# Use
To run the program run 
```cargo run main``` in the ```src``` directory

You will be prompted by the command line for input displayed like this
```psql> ```

The Current SQL statements that work with this program as of now are

```CREATE TABLE name (column, etc, ...);```

Primary keys and foreign keys are not implemented as of yet so this is only way you can use this statement as of now. Functionality such as CASCADE, ON, etc are not implemented. The above format is only format that works for now

```DROP TABLE name;```

Functionality such as CASCADE, ON, etc are not implemented. The above format is only format that works for now

```INSERT INTO name (column, etc, ...)  VALUES (val, etc, ...);```

The above format is only format that works for now.

```SELECT [column, etc, ... | (column, etc, ..)] FROM name;```

The above format is only format that works for now. The column names can be in parentheses or not.

## Example
```psql> CREATE TABLE test (column, column2)```

Result: ```Created table test```

```psql> CREATE TABLE test (,);```

Result: ```Error empty or no column name provided```

# Reflection
I was able to get the parser working but I did run into some issues along the way. In beginning I couldn't read lists of columns unless they were indented with a space. It was difficult figuring out how to optionally take into account spaces when parsing a list; so I went to the TA and he found a great functinality that solved my problem by combining parsers. Another issue I have is with how I print the results. I couldn't figure out how to pretty print or format the results correctly, so for now they are just separated with | when printed. Which doesn't look as good, but I also believe I missed some edge case which I would like to address later. In the future I would like to address as much edge cases as I can and catch all input errors in the parser so that the input sent to the SQL functionality is valid. 
