use heck::*;
use quote::format_ident;
use std::error::Error;
use std::fs;
use std::path::Path;
use syn::{Ident, Item};
use ungrammar::{Grammar, Node, NodeData, Rule, Token, TokenData};

type IResult<T = ()> = Result<T, Box<dyn Error>>;

pub fn parse_to_file(ungram: impl AsRef<Path>, dist: impl AsRef<Path>) -> IResult {
    let ungram: Grammar = fs::read_to_string(ungram)?.parse()?;

    Ok(())
}

fn parse_node(gram: &Grammar, node: Node) -> IResult<Vec<Item>> {
    let node = &gram[node];
    let name = format_ident!("{}", node.name.to_upper_camel_case());
    // ungram.rule
    todo!()
}

fn parse_rule(gram: &Grammar, rule: &Rule) -> IResult<Vec<Item>> {
    Ok(match rule {
        Rule::Labeled { label, rule } => parse_rule(gram, &*rule)?,
        Rule::Node(v) => parse_node(gram, *v)?,
        Rule::Token(v) => parse_token(gram, *v)?,
        Rule::Seq(v) | Rule::Alt(v) => {
            parse_rule(gram, v)
        },
        Rule::Opt(v) => todo!(),
        Rule::Rep(v) => todo!(),
    })
}

fn parse_token(gram: &Grammar, token: Token) -> IResult<Vec<Item>> {
    todo!()
}
