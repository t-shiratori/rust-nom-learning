mod alt;
mod delimited;
mod many0;
mod pair;
mod statement;
mod tag;

fn main() {
    /*
     *  alt
     */
    alt::parsers::alt_parser();

    /*
     *  tag
     */
    tag::parsers::tag_parser();

    /*
     * parir
     */
    pair::parsers::tag_parser();
    pair::parsers::plus_space_parser();

    /*
     * delimited
     */
    delimited::parsers::parentheses_parser();
    delimited::parsers::space_parser();
    delimited::parsers::plus_parser();

    /*
     * many0
     */
    many0::parsers::tag_parser();
    many0::parsers::expr_parser();

    /*
     * statement
     */
    statement::parsers::statements_parser();
}
