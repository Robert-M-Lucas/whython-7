use thiserror::__private::AsDynError;
use crate::ast::keywords::Keyword;
use crate::ast::{operators};
use crate::ast::operators::Operator;
use crate::basic_ast::punctuation::Punctuation;
use crate::basic_ast::symbol::{BasicAbstractSyntaxTree, BasicSymbol};
use crate::parser::file_reader::FileReader;
use crate::parser::parse::{BlockType, ParseError};
use crate::parser::string_parser::parse_string;

pub fn parse_normal(reader: &mut FileReader, block_type: BlockType) -> Result<BasicSymbol, ParseError> {
    let start_line = reader.line();

    let mut buffer = String::new();

    let mut operator_mode = false;

    let mut symbols: Vec<(BasicSymbol, usize)> = Vec::new();

    loop {
        let next = reader.move_read_any();

        // * EOF
        if next.is_none() {
            process_buffer(&mut buffer, &mut operator_mode, &mut symbols, &reader)?;
            return if matches!(block_type, BlockType::Base) {
                Ok(BasicSymbol::AbstractSyntaxTree(symbols))
            } else {
                let terminator = match block_type {
                    BlockType::Braces => Some('}'),
                    BlockType::Brackets => Some(')'),
                    BlockType::SquareBrackets => Some(']'),
                    BlockType::Base => None,
                };

                Err(reader.syntax_error(
                    format!("closing '{}' not found (started on line {})", terminator.unwrap(), start_line)
                ))
            }
        }

        let next = next.unwrap();

        // * Opening/Closing blocks
        match next {
            '"' => {
                process_buffer(&mut buffer, &mut operator_mode, &mut symbols, &reader)?;
                let start = reader.line();
                symbols.push((parse_string(reader)?, start));
                continue;
            }
            c => {
                let closed_block = match c {
                    '}' => Some(BlockType::Braces),
                    ')' => Some(BlockType::Brackets),
                    ']' => Some(BlockType::SquareBrackets),
                    _ => None
                };

                if let Some(closed_block) = closed_block {
                    process_buffer(&mut buffer, &mut operator_mode, &mut symbols, &reader)?;
                    return if closed_block != block_type {
                        Err(reader.syntax_error(
                            format!("closing '{c}' found with no corresponding opening bracket")
                        ))
                    }
                    else {
                        Ok(match block_type {
                            BlockType::Braces => BasicSymbol::BracedSection(symbols),
                            BlockType::Brackets => BasicSymbol::BracketedSection(symbols),
                            BlockType::SquareBrackets => BasicSymbol::SquareBracketedSection(symbols),
                            _ => panic!()
                        })
                    }
                }

                let new_block = match c {
                    '{' => Some(BlockType::Braces),
                    '(' => Some(BlockType::Brackets),
                    '[' => Some(BlockType::SquareBrackets),
                    _ => None
                };

                if let Some(new_block) = new_block {
                    process_buffer(&mut buffer, &mut operator_mode, &mut symbols, &reader)?;
                    let start = reader.line();
                    symbols.push((parse_normal(reader, new_block)?, start));
                    continue;
                }
            }
        }

        // * Process buffer
        if next == ' ' || next == '\t' || next == '\n' || next == '\r' {
            if buffer.is_empty() {
                continue;
            }
            process_buffer(&mut buffer, &mut operator_mode, &mut symbols, &reader)?;
            continue;
        }

        if operators::ALL_SYMBOLS.contains(&next) {
            if !operator_mode {
                process_buffer(&mut buffer, &mut operator_mode, &mut symbols, &reader)?;
                buffer.push(next);
                operator_mode = true;
                continue;
            }
        }
        else if operator_mode {
            process_buffer(&mut buffer, &mut operator_mode, &mut symbols, &reader)?;
            buffer.push(next);
            operator_mode = false;
            continue;
        }

        if next == ';' {
            process_buffer(&mut buffer, &mut operator_mode, &mut symbols, &reader)?;
            symbols.push((BasicSymbol::Punctuation(Punctuation::Semicolon), reader.line()));
            continue;
        }

        if next == ',' {
            process_buffer(&mut buffer, &mut operator_mode, &mut symbols, &reader)?;
            symbols.push((BasicSymbol::Punctuation(Punctuation::ListSeparator), reader.line()));
            continue;
        }

        buffer.push(next)
    }
}

fn process_buffer(buffer: &mut String, operator_mode: &mut bool, symbols: &mut Vec<(BasicSymbol, usize)>, reader: &FileReader) -> Result<(), ParseError> {
    if buffer.is_empty() {
        return Ok(());
    }

    if *operator_mode {
        symbols.push((process_operator_buffer(buffer, reader)?, reader.line()));
        *operator_mode = false;
        buffer.clear();
        return Ok(());
    }

    let mut split = buffer.split('.');

    let first = split.next().unwrap();
    if let Some(keyword) = Keyword::get_enum(first) {
        if split.next().is_some() {
            return Err(reader.syntax_error(
                format!("keywords (here '{first}') cannot be followed by '.'")
            ));
        }
        symbols.push((BasicSymbol::Keyword(keyword), reader.line()));
        buffer.clear();
        return Ok(());
    }

    let mut names = vec![first.to_string()];
    for name in split {
        names.push(name.to_string());
    }
    symbols.push((BasicSymbol::Name(names), reader.line()));
    buffer.clear();
    Ok(())
}

fn process_operator_buffer(buffer: &String, reader: &FileReader) -> Result<BasicSymbol, ParseError> {
    let operator = Operator::get_operator(buffer.as_str());
    if let Some(operator) = operator {
        return Ok(BasicSymbol::Operator(operator));
    }

    Err(reader.syntax_error(
        format!("operator '{buffer}' not recognised")
    ))
}