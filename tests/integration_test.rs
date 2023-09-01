use commonmark_rust::parser::parse;

#[test]
fn multiple_spaces() {
    assert_eq!(parse("Multiple     spaces"), "<p>Multiple     spaces</p>");
}

#[test]
fn foo_unicode() {
    assert_eq!(parse("Foo χρῆν"), "<p>Foo χρῆν</p>");
}

#[test]
fn hello_symbols_there() {
    assert_eq!(parse("hello $.;'there"), "<p>hello $.;'there</p>");
}

#[test]
fn foo_whitespace_newline_baz() {
    assert_eq!(parse("foo \n baz"), "<p>foo\nbaz</p>");
}

#[test]
fn foo_newline_baz() {
    assert_eq!(parse("foo\nbaz"), "<p>foo\nbaz</p>");
}
