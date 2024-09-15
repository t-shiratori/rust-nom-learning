use nom::{
    bytes::complete::tag,
    error::{Error, ErrorKind},
    multi::many0,
    sequence::delimited,
    Err, IResult,
};

fn main() {
    /*
     *  tag
     */
    fn parser(s: &str) -> IResult<&str, &str> {
        tag("Hello")(s)
    }
    assert_eq!(parser("Hello, World!"), Ok((", World!", "Hello")));
    assert_eq!(
        parser("Something"),
        Err(Err::Error(Error::new("Something", ErrorKind::Tag)))
    );
    assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Tag))));

    /*
     * delimited
     */
    let mut delimited_parser = delimited(tag("("), tag("abc"), tag(")"));
    assert_eq!(delimited_parser("(abc)"), Ok(("", "abc")));
    assert_eq!(delimited_parser("(abc)def"), Ok(("def", "abc")));
    assert_eq!(delimited_parser(""), Err(Err::Error(("", ErrorKind::Tag))));
    assert_eq!(
        delimited_parser("123"),
        Err(Err::Error(("123", ErrorKind::Tag)))
    );

    /*
     * many0
     */

    fn many0_parser(s: &str) -> IResult<&str, Vec<&str>> {
        many0(tag("abc"))(s)
    }

    assert_eq!(many0_parser("abcabc"), Ok(("", vec!["abc", "abc"])));
    assert_eq!(many0_parser("abc123"), Ok(("123", vec!["abc"])));
    assert_eq!(many0_parser("123123"), Ok(("123123", vec![])));
    assert_eq!(many0_parser(""), Ok(("", vec![])));
}
