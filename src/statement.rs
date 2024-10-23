/*
 * separated_list0
 */

pub mod parsers {

    use nom::{bytes::complete::tag, multi::separated_list0, IResult};

    pub fn statements_parser() {
        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            separated_list0(tag("|"), tag("abc"))(s)
        }

        assert_eq!(parser("abc|abc|abc"), Ok(("", vec!["abc", "abc", "abc"])));
        assert_eq!(parser("abc123abc"), Ok(("123abc", vec!["abc"])));
        assert_eq!(parser("abc|def"), Ok(("|def", vec!["abc"])));
        assert_eq!(parser(""), Ok(("", vec![])));
        assert_eq!(parser("def|abc"), Ok(("def|abc", vec![])));
    }
}
