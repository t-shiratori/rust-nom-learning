/*
 * many0
 */

pub mod parsers {

    use nom::{bytes::complete::tag, multi::many0, sequence::delimited, IResult};

    pub fn tag_parser() {
        fn many0_parser(s: &str) -> IResult<&str, Vec<&str>> {
            many0(tag("abc"))(s)
        }

        assert_eq!(many0_parser("abcabc"), Ok(("", vec!["abc", "abc"])));
        assert_eq!(many0_parser("abc123"), Ok(("123", vec!["abc"])));
        assert_eq!(many0_parser("123123"), Ok(("123123", vec![])));
        assert_eq!(many0_parser(""), Ok(("", vec![])));
        assert_eq!(many0_parser(""), Ok(("", vec![])));
    }

    pub fn expr_parser() {
        fn expr(i: &str) -> IResult<&str, &str> {
            print!("{}", i);
            tag("abc")(i)
        }

        fn many0_parser(s: &str) -> IResult<&str, Vec<&str>> {
            many0(delimited(tag("("), expr, tag(")")))(s)
        }

        assert_eq!(many0_parser("(abc)"), Ok(("", vec!["abc"])));
        assert_eq!(many0_parser(""), Ok(("", vec![])));
    }
}
