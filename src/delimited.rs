/*
 * delimited
 */

pub mod parsers {

    use nom::{
        bytes::complete::tag,
        character::complete::{char, multispace0},
        error::ErrorKind,
        sequence::delimited,
        Err,
    };

    pub fn parentheses_parser() {
        let mut delimited_parser = delimited(tag("("), tag("abc"), tag(")"));
        assert_eq!(delimited_parser("(abc)"), Ok(("", "abc")));
        assert_eq!(delimited_parser("(abc)def"), Ok(("def", "abc")));
        assert_eq!(delimited_parser(""), Err(Err::Error(("", ErrorKind::Tag))));
        assert_eq!(
            delimited_parser("123"),
            Err(Err::Error(("123", ErrorKind::Tag)))
        );
    }

    pub fn space_parser() {
        let mut delimited_space_parser = delimited(multispace0, tag("abc"), multispace0);
        assert_eq!(delimited_space_parser(" abc "), Ok(("", "abc")));
        assert_eq!(delimited_space_parser(" abc def"), Ok(("def", "abc")));
        assert_eq!(
            delimited_space_parser("123"),
            Err(Err::Error(("123", ErrorKind::Tag)))
        );
        assert_eq!(
            delimited_space_parser(""),
            Err(Err::Error(("", ErrorKind::Tag)))
        );
    }

    pub fn plus_parser() {
        let mut delimited_plus_parser = delimited(multispace0, char('+'), multispace0);

        assert_eq!(
            delimited_plus_parser("1+2"),
            Err(Err::Error(("1+2", ErrorKind::Char)))
        );

        assert_eq!(delimited_plus_parser("+"), Ok(("", '+')));

        assert_eq!(delimited_plus_parser(" + "), Ok(("", '+')));

        assert_eq!(
            delimited_plus_parser(""),
            Err(Err::Error(("", ErrorKind::Char)))
        );
    }
}
