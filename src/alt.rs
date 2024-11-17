/*
 * alt
 */

pub mod parsers {

    use nom::branch::alt;
    use nom::character::complete::{alpha1, digit1};
    use nom::error::ErrorKind;
    use nom::IResult;
    use nom::{error_position, Err};

    pub fn alt_parser() {
        fn parser(input: &str) -> IResult<&str, &str> {
            alt((alpha1, digit1))(input)
        }
        // the first parser, alpha1, recognizes the input
        assert_eq!(parser("abc"), Ok(("", "abc")));

        // the first parser returns an error, so alt tries the second one
        assert_eq!(parser("123456"), Ok(("", "123456")));

        assert_eq!(parser("abc123456"), Ok(("123456", "abc")));

        assert_eq!(parser("123456abc"), Ok(("abc", "123456")));

        assert_eq!(
            parser("?"),
            Err(Err::Error(error_position!("?", ErrorKind::Digit)))
        );

        assert_eq!(
            parser("?1"),
            Err(Err::Error(error_position!("?1", ErrorKind::Digit)))
        );

        assert_eq!(
            parser("?a"),
            Err(Err::Error(error_position!("?a", ErrorKind::Digit)))
        );

        // both parsers failed, and with the default error type, alt will return the last error
        assert_eq!(
            parser(" "),
            Err(Err::Error(error_position!(" ", ErrorKind::Digit)))
        );
    }
}
