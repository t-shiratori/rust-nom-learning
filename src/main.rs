use nom::{
    bytes::complete::tag,
    error::{Error, ErrorKind},
    Err, IResult,
};

fn main() {
    fn parser(s: &str) -> IResult<&str, &str> {
        tag("Hello")(s)
    }

    assert_eq!(parser("Hello, World!"), Ok((", World!", "Hello")));
    assert_eq!(
        parser("Something"),
        Err(Err::Error(Error::new("Something", ErrorKind::Tag)))
    );
    assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Tag))));

    //     let mut parser = delimited(tag("("), tag("abc"), tag(")"));
    // assert_eq!(parser("(abc)"), Ok(("", "abc")));
    // assert_eq!(parser("(abc)def"), Ok(("def", "abc")));
}
