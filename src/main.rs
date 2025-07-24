use untwine::{parser};
use untwine::context::ParserContext;
use untwine::pretty::PrettyOptions;
use cel_rs::{DynSegment};

parser! {
    [recover = true, context = ctx, data = DynSegment]
    __ = #{char::is_ascii_whitespace}*;
    pub expression = or_expression -> ();
    or_expression = and_expression (__ "||" __ and_expression)* -> ();
    and_expression = comparison_expression (__ "&&" __ comparison_expression)* -> ();
    comparison_expression = bitwise_or_expression (__ ("==" | "!=" | "<" | ">" | "<=" | ">=") __ bitwise_or_expression)? -> ();
    bitwise_or_expression = bitwise_xor_expression (__ "|" __ bitwise_xor_expression)* -> ();
    bitwise_xor_expression = bitwise_and_expression (__ "^" __ bitwise_and_expression)* -> ();
    bitwise_and_expression = bitwise_shift_expression (__ "&" __ bitwise_shift_expression)* -> ();
    bitwise_shift_expression = additive_expression (__ ("<<" | ">>") __ additive_expression)* -> ();
    additive_expression = multiplicative_expression (__ ("+" | "-") __ multiplicative_expression)* -> ();
    multiplicative_expression = unary_expression (__ ("*" | "/" | "%") __ unary_expression)* -> ();
    unary_expression = ((("-" | "!") __ unary_expression) | primary_expression) -> ();
    primary_expression: ("(" __ expression __ ")" | literal | identifier) -> () {}
    literal: digits=<'0'-'9'+> -> () { let r: u32 = digits.parse().unwrap(); ctx.data_mut().op0(move || r) }
    identifier = #["ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_"]+ -> ();
}

static TEST: &str = "42";

fn main() {
    let mut ctx = ParserContext::new(TEST, DynSegment::new::<()>());
    let result = expression(&ctx);
    let _ = ctx.pretty_result(result, PrettyOptions::default());
    println!("{:?}", ctx.data_mut().call0::<u32>().unwrap());
}
