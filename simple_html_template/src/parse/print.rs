use syntax::parse::parser::Parser;
use syntax::parse::token;

use syntax::ext::base;

use tags::template::SubTag;
use tags::template::Print;

use parse_utils::block_to_string;

/// Parse the inside of a orphan print tag
pub fn parse_print_tag (
    parser: &mut Parser,
) -> SubTag {

    let ident = parser.parse_ident();
    if parser.token == token::BINOP(token::PERCENT) {
        if parser.look_ahead(1, |token| *token == token::GT) {

            //TODO: certainly a better way to do "consume % and >"
            parser.bump();
            parser.bump();

            return Print(ident);
        }
    }

    parser.fatal("`<%= %>` tag open but not closed");
}


