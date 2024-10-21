/*
 * tag
 */

pub mod parsers {

    use nom::{bytes::complete::tag, error::{Error, ErrorKind}, Err, IResult};

    pub fn tag_parser() {
        fn parser(s: &str) -> IResult<&str, &str> {
            tag("Hello")(s)
        }
        assert_eq!(parser("Hello, World!"), Ok((", World!", "Hello")));
        assert_eq!(
            parser("Something"),
            Err(Err::Error(Error::new("Something", ErrorKind::Tag)))
        );
        assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Tag))));
    }
}
