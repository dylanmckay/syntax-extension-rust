use syntax::ast;
use syntax::codemap;
use syntax::ext::base;
use syntax::parse::token;
use syntax::parse::parser::Parser;

use syntax::codemap::Pos;

use html::HtmlState;

/// Trait that means something can be parsed with a configuration.
pub trait Parse<Cfg> {
    /// Parse Self from a Parser and a configuration object.
    fn parse(&mut Parser, Cfg) -> Self;
}

const TEMPLATE:      &'static str = "template";
const END:           &'static str = "end";

///
fn parse_start_template(state: &mut HtmlState, parser: &mut Parser) {

    match (
        parser.parse_ident(),
        parser.parse_fn_decl(true),
        parser.bump_and_get(),
        parser.bump_and_get()
    ) {
        (
            functioname,
            ref function,
            token::BINOP(token::PERCENT),
            token::GT
        ) => {
            println!("found template beginning")
        },

        (one, two, three, four) => {
            parser.fatal(format!(
                "Expected `<% template xxx() %>`, found <% template {} {} {}{}",
                one,
                two,
                three,
                four
            ).as_slice());
        }
    };
}

fn parse_end_template(state: &mut HtmlState, parser: &mut Parser) {

    match (
        parser.parse_ident().as_str(),
        parser.bump_and_get(),
        parser.bump_and_get()
    ) {
        (
            template,
            token::BINOP(token::PERCENT),
            token::GT
        ) if template == TEMPLATE => { println!("found end template")},

        (one, two, three) => {
            parser.fatal(format!(
                "Expected `<% end template %>`, found <% end {} {}{}",
                one,
                Parser::token_to_string(&two),
                Parser::token_to_string(&three),
            ).as_slice());
        }
    };

}

fn is_template_tag_start (parser: &Parser, last_token: Option<token::Token>) -> bool {

    if (parser.token != token::BINOP(token::PERCENT)) {
        return false;
    }


    match last_token {
        None => return false,
        Some(value) => {
            return value == token::LT;
        }
    }

}


///
///
///
impl<'a, 'b> Parse<(
    codemap::Span,
    &'a mut base::ExtCtxt<'b>,
    Option<ast::Ident>
)> for HtmlState {

    ///
    ///
    ///
    fn parse(
        parser: &mut Parser,
        (_, _, name): (codemap::Span, &'a mut base::ExtCtxt, Option<ast::Ident>)
    ) -> HtmlState {

        let mut state = HtmlState::new(name);

        println!("parser");

        let mut last_token = None;

        while parser.token != token::EOF {
            
            if !is_template_tag_start(parser, last_token) {
                last_token = Some(parser.token.clone());
                parser.bump();
                continue;
            }

            last_token = Some(parser.token.clone());
            parser.bump();
            //TODO handle token::LE (see how they've done for brain_fuck macro
            match parser.parse_ident().as_str() {

                TEMPLATE => {
                    if state.template_opened {
                        parser.fatal("<% template %> can't be nested");
                    }
                    parse_start_template(&mut state, parser);
                    state.template_opened = true;
                },
                END => {
                    if state.template_opened == false {
                        parser.fatal(
                            "<% end xxxx %> found without opening tag"
                        );
                    }
                    parse_end_template(&mut state, parser);
                    state.template_opened = false;
                },
                otherwise => {
                    let span = parser.last_span;
                    parser.span_fatal(
                        span,
                        format!(
                            "Expected `template` or `end`, but found `{}`",
                            otherwise
                        ).as_slice()
                    );
                }
            }

            last_token = Some(parser.token.clone());
            parser.bump();
        }

        state
    }
}
