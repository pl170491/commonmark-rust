use crate::html::{Element, HtmlTag, Node, NodeType};
use crate::parser::common::{is_space, line_term};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::space0,
    multi::many_m_n,
    sequence::{delimited, terminated, tuple},
    IResult,
};

pub fn line_hr(input: &str) -> IResult<&str, Node> {
    let (input, _) = delimited(hr_pre, hr, line_term)(input)?;

    let hr_el = Element::new(HtmlTag::HR);
    let hr_node = Node::new(NodeType::ElementNode(hr_el));

    Ok((input, hr_node))
}

fn hr(input: &str) -> IResult<&str, &str> {
    alt((hr_star, hr_dash, hr_underscore))(input)
}

fn hr_star(input: &str) -> IResult<&str, &str> {
    hr_arb_char((input, '*'))
}

fn hr_dash(input: &str) -> IResult<&str, &str> {
    hr_arb_char((input, '-'))
}

fn hr_underscore(input: &str) -> IResult<&str, &str> {
    hr_arb_char((input, '_'))
}

fn hr_arb_char(input: (&str, char)) -> IResult<&str, &str> {
    let test_char = input.1;
    let is_char = |c: char| c == test_char;
    let test_str = input.1.to_string();
    let test_str = test_str.as_str();

    let (input, _) = terminated(
        tuple((tag(test_str), space0, tag(test_str), space0, tag(test_str))),
        take_while(|c| is_space(c) || is_char(c)),
    )(input.0)?;
    Ok((input, ""))
}

fn hr_pre(input: &str) -> IResult<&str, &str> {
    let (input, _) = many_m_n(0, 3, tag(" "))(input)?;
    Ok((input, ""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hr_wrong_characters() {
        assert_eq!(
            line_hr("+++"),
            Err(nom::Err::Error(nom::error::Error {
                input: "+++",
                code: nom::error::ErrorKind::Tag
            }))
        );

        assert_eq!(
            line_hr("==="),
            Err(nom::Err::Error(nom::error::Error {
                input: "===",
                code: nom::error::ErrorKind::Tag
            }))
        );
    }

    #[test]
    fn hr_different_types_simple() {
        let hr_el = Element::new(HtmlTag::HR);
        let hr_node = Node::new(NodeType::ElementNode(hr_el));
        assert_eq!(line_hr("___"), Ok(("", hr_node)));

        let hr_el = Element::new(HtmlTag::HR);
        let hr_node = Node::new(NodeType::ElementNode(hr_el));
        assert_eq!(line_hr("---"), Ok(("", hr_node)));

        let hr_el = Element::new(HtmlTag::HR);
        let hr_node = Node::new(NodeType::ElementNode(hr_el));
        assert_eq!(line_hr("***"), Ok(("", hr_node)));
    }

    #[test]
    fn hr_mixed() {
        assert_eq!(
            line_hr("-*_"),
            Err(nom::Err::Error(nom::error::Error {
                input: "-*_",
                code: nom::error::ErrorKind::Tag
            }))
        );

        assert_eq!(
            line_hr(" *-*"),
            Err(nom::Err::Error(nom::error::Error {
                input: "*-*",
                code: nom::error::ErrorKind::Tag
            }))
        );
    }

    #[test]
    fn hr_long() {
        let hr_el = Element::new(HtmlTag::HR);
        let hr_node = Node::new(NodeType::ElementNode(hr_el));

        assert_eq!(
            line_hr("_____________________________________"),
            Ok(("", hr_node))
        );
    }

    #[test]
    fn hr_mixed_termination() {
        assert_eq!(
            line_hr("***-_*"),
            Err(nom::Err::Error(nom::error::Error {
                input: "-_*",
                code: nom::error::ErrorKind::Eof
            }))
        );
    }

    #[test]
    fn hr_insufficient_chars() {
        assert_eq!(
            line_hr("--"),
            Err(nom::Err::Error(nom::error::Error {
                input: "--",
                code: nom::error::ErrorKind::Tag
            }))
        );
    }

    #[test]
    fn hr_allowed_indentation_spaces() {
        let hr_el = Element::new(HtmlTag::HR);
        let hr_node = Node::new(NodeType::ElementNode(hr_el));
        assert_eq!(line_hr(" ***"), Ok(("", hr_node)));

        let hr_el = Element::new(HtmlTag::HR);
        let hr_node = Node::new(NodeType::ElementNode(hr_el));
        assert_eq!(line_hr("  ***"), Ok(("", hr_node)));

        let hr_el = Element::new(HtmlTag::HR);
        let hr_node = Node::new(NodeType::ElementNode(hr_el));
        assert_eq!(line_hr("   ***"), Ok(("", hr_node)));
    }

    #[test]
    fn hr_too_many_indentation_spaces() {
        assert_eq!(
            line_hr("    ***"),
            Err(nom::Err::Error(nom::error::Error {
                input: " ***",
                code: nom::error::ErrorKind::Tag
            }))
        );
    }

    #[test]
    fn hr_intermediate_spaces() {
        let hr_el = Element::new(HtmlTag::HR);
        let hr_node = Node::new(NodeType::ElementNode(hr_el));
        assert_eq!(line_hr(" - - -"), Ok(("", hr_node)));

        let hr_el = Element::new(HtmlTag::HR);
        let hr_node = Node::new(NodeType::ElementNode(hr_el));
        assert_eq!(line_hr(" **  * ** * ** * **"), Ok(("", hr_node)));

        let hr_el = Element::new(HtmlTag::HR);
        let hr_node = Node::new(NodeType::ElementNode(hr_el));
        assert_eq!(line_hr("-     -      -      -"), Ok(("", hr_node)));
    }

    #[test]
    fn hr_allowed_post_spaces() {
        let hr_el = Element::new(HtmlTag::HR);
        let hr_node = Node::new(NodeType::ElementNode(hr_el));
        assert_eq!(line_hr(" - - -    "), Ok(("", hr_node)));

        let hr_el = Element::new(HtmlTag::HR);
        let hr_node = Node::new(NodeType::ElementNode(hr_el));
        assert_eq!(line_hr(" - - -\t"), Ok(("", hr_node)));
    }

    #[test]
    fn hr_disallowed_chars() {
        assert_eq!(
            line_hr("_ _ _ _ a"),
            Err(nom::Err::Error(nom::error::Error {
                input: "a",
                code: nom::error::ErrorKind::Eof
            }))
        );

        assert_eq!(
            line_hr("a------"),
            Err(nom::Err::Error(nom::error::Error {
                input: "a------",
                code: nom::error::ErrorKind::Tag
            }))
        );

        assert_eq!(
            line_hr("---a---"),
            Err(nom::Err::Error(nom::error::Error {
                input: "a---",
                code: nom::error::ErrorKind::Eof
            }))
        );
    }
}
