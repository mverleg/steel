#![allow(unused)] //TODO @mark: TEMPORARY! REMOVE THIS!

use ::std::path::PathBuf;

use ::lalrpop_util::lalrpop_mod;

use ::steel_api::log::debug;

use crate::ast::Ast;
use crate::parser::errors::build_error;
use crate::SteelErr;

mod custom;
mod errors;
mod lexer;

// lalrpop_mod!(gen_parser, "/grammar.rs");

pub fn parse_str(src_pth: PathBuf, code: &str) -> Result<Ast, SteelErr> {
    self::custom::parse_str(src_pth, code)
    // let parser = gen_parser::ProgParser::new();
    // let res = parser.parse(code);
    // match res {
    //     Ok(ast) => {
    //         debug!("ast: {:?}", &ast);
    //         Ok(ast)
    //     },
    //     Err(err) => {
    //         let (msg, line) = build_error(err, src_pth.to_str().unwrap(), code);
    //         Err(SteelErr::ParseErr {
    //             file: src_pth,
    //             line,
    //             msg,
    //         })
    //     }
    // }
    //TODO @mark: no unwrap
}
