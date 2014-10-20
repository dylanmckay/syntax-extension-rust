use syntax::parse::parser::Parser;
use syntax::parse::token;

use syntax::ext::base;

use tags::template::SubTag;
use tags::template::RawRust;

use parse_utils::block_to_string;

/// Parse the inside of a orphan rust tag
/// TODO: implement it, for the moment we simply "consume" the inside
pub fn parse_rust_tag (
    parser: &mut Parser,
    context: &base::ExtCtxt
) -> SubTag {

    let start_rust_block = parser.span.clone();
    while parser.token != token::EOF {

        if parser.token == token::BINOP(token::PERCENT) {
            if parser.look_ahead(1, |token| *token == token::GT) {

                let inner_string = block_to_string(
                    context,
                    &start_rust_block,
                    &parser.span
                );
                
                //TODO: certainly a better way to do "consume % and >"
                parser.bump();
                parser.bump();

                return RawRust(inner_string);
            }
        }
        parser.bump();
    }

    RawRust(String::new())
}

