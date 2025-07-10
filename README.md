# JSON Parser

## Overview
A simple JSON parser written in Rust that converts JSON text into Rust data structures. The parser handles all standard JSON types including null, booleans, numbers, strings, arrays, and objects.

This library implements a JSON parser with the following features:

- Parses JSON text into structured Rust data types
- Provides error messages for invalid JSON

### JSON Data Types
The parser supports the following JSON data types:
- Null: `null`
- Booleans: `true`, `false`
- Numbers: integers and floating-point (e.g., `123`, `-45.67`, `1.23e-4`)
- Strings: text enclosed in double quotes, with escape sequences (e.g., `"hello"`, `"line\nbreak"`)
- Arrays: ordered lists of values (e.g., `[1, 2, 3]`)
- Objects: collections of key-value pairs (e.g., `{"name": "John", "age": 30}`)

### Parsing Process
The parser works by:
1. Looking at the first character of the input to determine the type
2. Calling the appropriate parsing method based on the type
3. Building a `JsonValue` enum variant that represents the parsed data
4. Handling any errors that might occur during parsing

### Using Peekable for Parsing
The parser uses Rust's `Peekable` iterator, which is a key for effective parsing:

- `Peekable` allows looking at the next character without consuming it
- This "look-ahead" capability is essential for deciding how to parse different elements
- For example, when encountering a `{` character, the parser knows to call `parse_object()`
- The parser can check for delimiters (like commas and closing brackets) without removing them prematurely

 For more details on using `Peekable`, refer to the [Rust standard library documentation](https://doc.rust-lang.org/std/iter/struct.Peekable.html).

## References
- [Writing a Simple Parser in Rust](https://adriann.github.io/rust_parser.html)
- [JSON Parsing in Rust - Part 1](https://blog.davimiku.com/tutorials/json-parsing-rust-1)
