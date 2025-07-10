mod test;

use std::collections::HashMap;
use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]

//represents all possible JSON value types
pub enum JsonValue {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

//display trait to print JSON values as strings
impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JsonValue::Null => f.write_str("null"),
            JsonValue::Boolean(b) => b.fmt(f),
            JsonValue::Number(n) => n.fmt(f),
            JsonValue::String(s) => write!(f, "\"{}\"", s),
            JsonValue::Array(arr) => {
                if arr.is_empty() {
                    f.write_str("[]")
                } else {
                    f.write_str("[")?;
                    for (i, val) in arr.iter().enumerate() {
                        if i > 0 {
                            f.write_str(", ")?;
                        }
                        val.fmt(f)?;
                    }
                    f.write_str("]")
                }
            }
            JsonValue::Object(obj) => {
                if obj.is_empty() {
                    f.write_str("{}")
                } else {
                    f.write_str("{")?;
                    for (i, (key, val)) in obj.iter().enumerate() {
                        if i > 0 {
                            f.write_str(", ")?;
                        }
                        write!(f, "\"{}\": ", key)?;
                        val.fmt(f)?;
                    }
                    f.write_str("}")
                }
            }
        }
    }
}

//custom error type for JSON parsing errors
#[derive(Debug, Clone)]
pub struct JsonError(String);

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq for JsonError {
    fn eq(&self, _other: &JsonError) -> bool {
        true
    }
}

impl Eq for JsonError {}

#[derive(Debug)]
struct Parser<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser {
            chars: input.chars().peekable(),
        }
    }
    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    fn parse(&mut self) -> Result<JsonValue, JsonError> {
        let value = self.parse_value()?;
        Ok(value)
    }

    //parse any JSON value based on the first character
    fn parse_value(&mut self) -> Result<JsonValue, JsonError> {
        self.skip_whitespace();
        let c = match self.chars.peek() {
            Some(c) => *c,
            None => return Err(JsonError(String::from("Unexpected end of input"))),
        };
        let result = match c {
            '{' => self.parse_object(),
            '[' => self.parse_array(),
            '"' => self.parse_string().map(JsonValue::String),
            't' => self.parse_true(),
            'f' => self.parse_false(),
            'n' => self.parse_null(),
            '-' | '0'..='9' => self.parse_number(),
            _ => Err(JsonError(format!("Unexpected character: {}", c))),
        };
        result
    }

    //parse a JSON object
    fn parse_object(&mut self) -> Result<JsonValue, JsonError> {
        match self.chars.next() {
            Some('{') => {}
            _ => return Err(JsonError(String::from("Expected '{' at start of object"))),
        }
        self.skip_whitespace();

        let mut map = HashMap::new();
        if let Some('}') = self.chars.peek() {
            self.chars.next();
            return Ok(JsonValue::Object(map));
        }
        loop {
            self.skip_whitespace();
            let key = match self.parse_string() {
                Ok(k) => k,
                Err(e) => return Err(e),
            };
            match self.chars.next() {
                Some(':') => {}
                _ => return Err(JsonError(String::from("Expected ':' after object key"))),
            }
            let value = self.parse_value()?;
            map.insert(key, value);
            self.skip_whitespace();

            match self.chars.next() {
                Some(',') => {
                    continue;
                }
                Some('}') => {
                    break;
                }
                _ => {
                    return Err(JsonError(String::from(
                        "Expected ',' or '}' after object value",
                    )))
                }
            }
        }

        Ok(JsonValue::Object(map))
    }

    fn parse_array(&mut self) -> Result<JsonValue, JsonError> {
        match self.chars.next() {
            Some('[') => {}
            _ => return Err(JsonError(String::from("Expected '[' at start of array"))),
        }
        self.skip_whitespace();
        let mut array = Vec::new();
        if let Some(']') = self.chars.peek() {
            self.chars.next();
            return Ok(JsonValue::Array(array));
        }
        loop {
            let value = self.parse_value()?;
            array.push(value);
            self.skip_whitespace();
            match self.chars.next() {
                Some(',') => {
                    continue;
                }
                Some(']') => {
                    break;
                }
                _ => {
                    return Err(JsonError(String::from(
                        "Expected ',' or ']' after array element",
                    )))
                }
            }
        }

        Ok(JsonValue::Array(array))
    }

    fn parse_string(&mut self) -> Result<String, JsonError> {
        match self.chars.next() {
            Some('"') => {}
            _ => return Err(JsonError(String::from("Expected '\"' at start of string"))),
        }
        let mut result = String::new();
        loop {
            match self.chars.next() {
                Some('"') => {
                    break;
                }
                //unicode escape seqeunces
                Some('\\') => match self.chars.next() {
                    Some('"') => result.push('"'),
                    Some('\\') => result.push('\\'),
                    Some('/') => result.push('/'),
                    Some('b') => result.push('\u{0008}'),
                    Some('f') => result.push('\u{000C}'),
                    Some('n') => result.push('\n'),
                    Some('r') => result.push('\r'),
                    Some('t') => result.push('\t'),
                    _ => return Err(JsonError(String::from("Invalid escape sequence"))),
                },
                Some(c) => {
                    result.push(c);
                }
                None => return Err(JsonError(String::from("Unterminated string"))),
            }
        }

        Ok(result)
    }

    fn parse_number(&mut self) -> Result<JsonValue, JsonError> {
        let mut number_str = String::new();
        if let Some('-') = self.chars.peek() {
            number_str.push('-');
            self.chars.next();
        }
        let mut has_digits = false;
        if let Some('0') = self.chars.peek() {
            number_str.push('0');
            self.chars.next();
            has_digits = true;
        } else {
            match self.chars.peek() {
                Some(&c) if c.is_ascii_digit() => {
                    number_str.push(c);
                    self.chars.next();
                    has_digits = true;

                    while let Some(&c) = self.chars.peek() {
                        if c.is_ascii_digit() {
                            number_str.push(c);
                            self.chars.next();
                        } else {
                            break;
                        }
                    }
                }
                _ => {}
            }
        }

        if !has_digits {
            return Err(JsonError(String::from("Expected digit in number")));
        }

        if let Some('.') = self.chars.peek() {
            number_str.push('.');
            self.chars.next();

            let mut has_decimal_digits = false;
            while let Some(&c) = self.chars.peek() {
                if c.is_ascii_digit() {
                    number_str.push(c);
                    self.chars.next();
                    has_decimal_digits = true;
                } else {
                    break;
                }
            }

            if !has_decimal_digits {
                return Err(JsonError(String::from(
                    "Expected digit after decimal point",
                )));
            }
        }
        if let Some(&c) = self.chars.peek() {
            if c == 'e' || c == 'E' {
                number_str.push(c);
                self.chars.next();
                if let Some(&c) = self.chars.peek() {
                    if c == '+' || c == '-' {
                        number_str.push(c);
                        self.chars.next();
                    }
                }
                let mut has_exp_digits = false;
                while let Some(&c) = self.chars.peek() {
                    if c.is_ascii_digit() {
                        number_str.push(c);
                        self.chars.next();
                        has_exp_digits = true;
                    } else {
                        break;
                    }
                }

                if !has_exp_digits {
                    return Err(JsonError(String::from("Expected digit in exponent")));
                }
            }
        }
        match number_str.parse::<f64>() {
            Ok(n) => Ok(JsonValue::Number(n)),
            Err(_) => Err(JsonError(format!("Invalid number: {}", number_str))),
        }
    }

    fn parse_true(&mut self) -> Result<JsonValue, JsonError> {
        if self.consume_literal("true") {
            Ok(JsonValue::Boolean(true))
        } else {
            Err(JsonError(String::from("Expected true")))
        }
    }

    fn parse_false(&mut self) -> Result<JsonValue, JsonError> {
        if self.consume_literal("false") {
            Ok(JsonValue::Boolean(false))
        } else {
            Err(JsonError(String::from("Expected false")))
        }
    }

    fn parse_null(&mut self) -> Result<JsonValue, JsonError> {
        if self.consume_literal("null") {
            Ok(JsonValue::Null)
        } else {
            Err(JsonError(String::from("Expected null")))
        }
    }

    // Helper function to consume a literal string from the input
    // Returns true if the literal was found and consumed, false otherwise
    fn consume_literal(&mut self, literal: &str) -> bool {
        let chars = literal.chars();
        let mut input_matches = true;

        for expected in chars {
            match self.chars.next() {
                Some(c) if c == expected => {
                    continue;
                }
                _ => {
                    input_matches = false;
                    break;
                }
            }
        }
        input_matches
    }
}

pub fn parse(input: &str) -> Result<JsonValue, JsonError> {
    let mut parser = Parser::new(input);
    parser.parse()
}
