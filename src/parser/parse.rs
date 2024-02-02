use crate::ast::keywords::MOD_KEYWORD;
use crate::basic_ast::symbol::{BasicAbstractSyntaxTree, BasicSymbol};

use crate::parser::file_reader::FileReader;
use crate::parser::normal_parser::parse_normal;

use std::path::PathBuf;
use std::{fs, io};
use thiserror::Error;
use ParseError::{Nested};

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("file read error on path '{0}'")]
    FileRead(PathBuf, io::Error),
    #[error("syntax error in file {0}:{1} - {2}")]
    Syntax(PathBuf, usize, String),
    #[error("In file {0}:{1}:\n{2}")]
    Nested(PathBuf, usize, Box<ParseError>),
}

pub fn parse(path: PathBuf, asts: &mut Vec<BasicAbstractSyntaxTree>) -> Result<(), ParseError> {
    let data = fs::read_to_string(&path);

    if let Err(e) = data {
        return Err(ParseError::FileRead(path, e));
    }
    let mut reader = FileReader::new(path, data.unwrap());

    // * IMPORT PHASE
    {
        while reader.read_until_char(' ').0 == MOD_KEYWORD {
            reader.move_to_next_char(' ');

            let (file, eof) = reader.move_read_to_next_char(';');
            let trimmed = file.trim();
            if trimmed.is_empty() {
                return Err(
                    reader.syntax_error(format!("'{MOD_KEYWORD}' must be followed by a path"))
                );
            }
            if eof {
                return Err(
                    reader.syntax_error("import path must be followed by a ';'".to_string())
                );
            }

            if let Err(e) = parse(PathBuf::from(file), asts) {
                return Err(Nested(reader.get_path(), reader.line(), Box::new(e)));
            }
        }
    }

    let ast = parse_normal(&mut reader, BlockType::Base)?;

    let inner = match ast {
        BasicSymbol::AbstractSyntaxTree(inner) => inner,
        _ => panic!(),
    };

    asts.push((reader.get_path(), inner));
    Ok(())
}

#[derive(PartialEq)]
pub enum BlockType {
    Base,
    Braces,         // start line
    Brackets,       // start line
    SquareBrackets, // start line
}

// fn recursively_parse_symbols(reader: &mut FileReader, block_type: BlockType) -> Result<Symbol, ParseError> {
//     match block_type {
//         BlockType::String(start_line) => {
//             parse_string(reader, start_line)
//         }
//         _ => {
//             parse_normal(reader, block_type)
//         }
//     }
// }
