/*
 * pair
 */

pub mod parsers {

    use nom::{
        bytes::complete::tag,
        character::complete::{alphanumeric1, char, multispace0},
        error::ErrorKind,
        sequence::{delimited, pair},
        Err,
    };

    pub fn tag_parser() {
        let mut parser_pair = pair(tag("abc"), tag("efg"));

        assert_eq!(parser_pair("abcefg"), Ok(("", ("abc", "efg"))));
        assert_eq!(parser_pair("abcefghij"), Ok(("hij", ("abc", "efg"))));
        assert_eq!(parser_pair(""), Err(Err::Error(("", ErrorKind::Tag))));
        assert_eq!(parser_pair("123"), Err(Err::Error(("123", ErrorKind::Tag))));
    }

    pub fn plus_space_parser() {
        let mut plus_space_parser = pair(
            delimited(multispace0, char('+'), multispace0),
            delimited(multispace0, alphanumeric1, multispace0),
        );

        assert_eq!(plus_space_parser(" + 2"), Ok(("", ('+', "2"))));

        assert_eq!(
            plus_space_parser("1 + 2"),
            Err(Err::Error(("1 + 2", ErrorKind::Char)))
        );

        assert_eq!(
            plus_space_parser("1+2"),
            Err(Err::Error(("1+2", ErrorKind::Char)))
        );

        assert_eq!(
            plus_space_parser("+"),
            Err(Err::Error(("", ErrorKind::AlphaNumeric)))
        );

        assert_eq!(
            plus_space_parser(" + "),
            Err(Err::Error(("", ErrorKind::AlphaNumeric)))
        );

        assert_eq!(
            plus_space_parser(""),
            Err(Err::Error(("", ErrorKind::Char)))
        );

        assert_eq!(
            plus_space_parser("1 + 2 + 6"),
            Err(Err::Error(("1 + 2 + 6", ErrorKind::Char)))
        );
    }
}
