mod html_entity;
use html_entity::html_entity;
use std::str;

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take},
    character::complete::{alphanumeric1, char, one_of},
    combinator::{eof, peek},
    multi::fold_many_m_n,
    sequence::{delimited, pair, preceded},
    IResult,
};

pub fn is_space(c: char) -> bool {
    match c {
        ' ' | '\t' => true,
        _ => false,
    }
}

pub fn line_term(input: &str) -> IResult<&str, &str> {
    alt((eol, eof))(input)
}

pub fn eol(input: &str) -> IResult<&str, &str> {
    alt((tag("\r\n"), tag("\r"), tag("\n")))(input)
}

const ASCII_PUNCTUATION_CHARACTERS: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
const REPLACEMENT_CHARACTER: &str = "\u{FFFD}";

pub fn insecure(input: &str) -> IResult<&str, &str> {
    tag("\u{0000}")(input)
}

pub fn escaped(input: &str) -> IResult<&str, &str> {
    peek(preceded(char('\\'), one_of(ASCII_PUNCTUATION_CHARACTERS)))(input)?;
    preceded(take(1usize), take(1usize))(input)
}

pub fn entity_ref(input: &str) -> IResult<&str, &str> {
    let (input, output) = delimited(char('&'), alphanumeric1, char(';'))(input)?;
    match html_entity(output) {
        Some(entity) => Ok((input, entity)),
        None => Err(nom::Err::Error(nom::error::Error {
            input: input,
            code: nom::error::ErrorKind::Tag,
        })),
    }
}

// pub fn num_char_ref(input: &str) -> IResult<&str, &str> {
//     let (input, output) = peek(alt((hex_char_num, dec_char_num)))(input)?;
// }

fn hex_char_num(input: &str) -> IResult<&str, char> {
    let (input, output) = delimited(
        pair(tag("&#"), one_of("xX")),
        fold_many_m_n(
            1,
            6,
            is_a("0123456789ABCDEF"),
            String::new,
            |s: String, digit| s + digit,
        ),
        char(';'),
    )(input)?;

    let output = u32::from_str_radix(output.as_str(), 16).unwrap();
    let output = match char::from_u32(output) {
        Some(c) => c,
        None => char::REPLACEMENT_CHARACTER,
    };
    Ok((input, output))
}

fn dec_char_num(input: &str) -> IResult<&str, char> {
    let (input, output) = delimited(
        tag("&#"),
        fold_many_m_n(1, 7, is_a("0123456789"), String::new, |s: String, digit| {
            s + digit
        }),
        char(';'),
    )(input)?;

    let output = u32::from_str_radix(output.as_str(), 10).unwrap();
    let output = match char::from_u32(output) {
        Some(c) => c,
        None => char::REPLACEMENT_CHARACTER,
    };
    Ok((input, output))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_ref_capital_aelig() {
        assert_eq!(entity_ref("&AElig;"), Ok(("", "\u{00c6}")));
    }

    #[test]
    fn non_entity_ref() {
        assert_eq!(
            entity_ref("&madeUpEntity;"),
            Err(nom::Err::Error(nom::error::Error {
                input: "",
                code: nom::error::ErrorKind::Tag,
            })),
        );
    }

    #[test]
    fn illformed_entity_ref() {
        assert_eq!(
            entity_ref("&copy"),
            Err(nom::Err::Error(nom::error::Error {
                input: "",
                code: nom::error::ErrorKind::Char,
            })),
        );
        assert_eq!(
            entity_ref("&nbsp &x;"),
            Err(nom::Err::Error(nom::error::Error {
                input: " &x;",
                code: nom::error::ErrorKind::Char,
            })),
        );
    }

    #[test]
    fn decimal_char_number() {
        assert_eq!(dec_char_num("&#1234;"), Ok(("", 'Ӓ')),);
    }
}
