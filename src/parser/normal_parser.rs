use crate::ast::keywords::Keyword;
use crate::ast::literals::Literal;
use crate::ast::operators;
use crate::ast::operators::Operator;
use crate::basic_ast::punctuation::Punctuation;
use crate::basic_ast::symbol::{BasicSymbol, NameAccessType, NameType};
use crate::parser::file_reader::FileReader;
use crate::parser::line_info::LineInfo;
use crate::parser::parse::{BlockType, ParseError};
use crate::parser::string_parser::parse_string;

pub fn parse_normal(
    reader: &mut FileReader,
    block_type: BlockType,
) -> Result<BasicSymbol, ParseError> {
    let start_line = reader.line();

    let mut buffer = String::new();

    let mut operator_mode = false;

    let mut symbols: Vec<(BasicSymbol, LineInfo)> = Vec::new();

    reader.checkpoint();
    loop {
        let next = reader.move_read_any();
        reader.checkpoint();

        // * EOF
        if next.is_none() {
            process_buffer(&mut buffer, &mut operator_mode, &mut symbols, reader)?;
            return if matches!(block_type, BlockType::Base) {
                Ok(BasicSymbol::AbstractSyntaxTree(symbols))
            } else {
                let terminator = match block_type {
                    BlockType::Braces => Some('}'),
                    BlockType::Brackets => Some(')'),
                    BlockType::SquareBrackets => Some(']'),
                    BlockType::Base => None,
                };

                Err(ParseError::NotClosed(
                    reader.get_line_info(),
                    terminator.unwrap(),
                    start_line,
                ))
            };
        }

        let next = next.unwrap();

        // * Opening/Closing blocks
        match next {
            '"' => {
                process_buffer(&mut buffer, &mut operator_mode, &mut symbols, reader)?;
                let start = reader.line();
                reader.checkpoint();
                symbols.push((parse_string(reader)?, reader.get_line_info()));
                continue;
            }
            c => {
                let closed_block = match c {
                    '}' => Some(BlockType::Braces),
                    ')' => Some(BlockType::Brackets),
                    ']' => Some(BlockType::SquareBrackets),
                    _ => None,
                };

                if let Some(closed_block) = closed_block {
                    process_buffer(&mut buffer, &mut operator_mode, &mut symbols, reader)?;
                    return if closed_block != block_type {
                        Err(ParseError::NoOpening(reader.get_line_info_current(), c))
                    } else {
                        Ok(match block_type {
                            BlockType::Braces => BasicSymbol::BracedSection(symbols),
                            BlockType::Brackets => BasicSymbol::BracketedSection(symbols),
                            BlockType::SquareBrackets => {
                                BasicSymbol::SquareBracketedSection(symbols)
                            }
                            _ => panic!(),
                        })
                    };
                }

                let new_block = match c {
                    '{' => Some(BlockType::Braces),
                    '(' => Some(BlockType::Brackets),
                    '[' => Some(BlockType::SquareBrackets),
                    _ => None,
                };

                if let Some(new_block) = new_block {
                    process_buffer(&mut buffer, &mut operator_mode, &mut symbols, reader)?;

                    reader.checkpoint();
                    let parsed_pos = reader.get_line_info();
                    let parsed = parse_normal(reader, new_block)?;

                    let mut intercepted = false;
                    if !symbols.is_empty()
                        && matches!(symbols.last().unwrap().0, BasicSymbol::Name(_))
                        && matches!(&parsed, BasicSymbol::BracketedSection(_))
                        && matches!(
                            symbols
                                .last()
                                .unwrap()
                                .0
                                .get_name_contents()
                                .last()
                                .unwrap()
                                .2,
                            NameType::Normal
                        )
                    {
                        let (s, l) = &mut symbols.last_mut().unwrap();
                        let BasicSymbol::Name(v) = s else { panic!() };

                        intercepted = true;
                        let mut arguments = vec![Vec::new()];

                        let BasicSymbol::BracketedSection(symbols) = parsed else {
                            panic!();
                        };
                        for (symbol, line) in symbols {
                            match symbol {
                                BasicSymbol::Punctuation(Punctuation::ListSeparator) => {
                                    arguments.push(Vec::new())
                                }
                                symbol => arguments.last_mut().unwrap().push((symbol, line)),
                            }
                        }
                        if arguments.len() == 1 && arguments.last().unwrap().len() == 0 {
                            arguments.pop();
                        }

                        *(&mut v.last_mut().unwrap().2) = NameType::Function(arguments);
                    } else {
                        symbols.push((parsed, parsed_pos));
                    }
                    continue;
                }
            }
        }

        // * Process buffer
        if next == ' ' || next == '\t' || next == '\n' || next == '\r' {
            if buffer.is_empty() {
                continue;
            }
            process_buffer(&mut buffer, &mut operator_mode, &mut symbols, reader)?;
            continue;
        }

        if operators::ALL_SYMBOLS.contains(&next) {
            if !operator_mode {
                process_buffer(&mut buffer, &mut operator_mode, &mut symbols, reader)?;
                buffer.push(next);
                operator_mode = true;
                continue;
            }
        } else if operator_mode {
            process_buffer(&mut buffer, &mut operator_mode, &mut symbols, reader)?;
            buffer.push(next);
            operator_mode = false;
            continue;
        }

        if next == ';' {
            process_buffer(&mut buffer, &mut operator_mode, &mut symbols, reader)?;
            symbols.push((
                BasicSymbol::Punctuation(Punctuation::Semicolon),
                reader.get_line_info_current(),
            ));
            continue;
        }

        if next == ',' {
            process_buffer(&mut buffer, &mut operator_mode, &mut symbols, reader)?;
            symbols.push((
                BasicSymbol::Punctuation(Punctuation::ListSeparator),
                reader.get_line_info_current(),
            ));
            continue;
        }

        if next == ':' {
            process_buffer(&mut buffer, &mut operator_mode, &mut symbols, reader)?;
            symbols.push((
                BasicSymbol::Punctuation(Punctuation::Colon),
                reader.get_line_info_current(),
            ));
            continue;
        }

        if next == '~' {
            process_buffer(&mut buffer, &mut operator_mode, &mut symbols, reader)?;
            symbols.push((
                BasicSymbol::Punctuation(Punctuation::Tilda),
                reader.get_line_info_current(),
            ));
            continue;
        }

        if buffer.is_empty() {
            reader.checkpoint();
        }
        buffer.push(next)
    }
}

fn process_buffer(
    buffer: &mut String,
    operator_mode: &mut bool,
    symbols: &mut Vec<(BasicSymbol, LineInfo)>,
    reader: &FileReader,
) -> Result<(), ParseError> {
    if buffer.is_empty() {
        return Ok(());
    }

    if *operator_mode {
        symbols.push((
            process_operator_buffer(buffer, reader)?,
            reader.get_line_info(),
        ));
        *operator_mode = false;
        buffer.clear();
        return Ok(());
    }

    if buffer == "true" {
        symbols.push((
            BasicSymbol::Literal(Literal::Bool(true)),
            reader.get_line_info(),
        ));
        buffer.clear();
        return Ok(());
    }
    if buffer == "false" {
        symbols.push((
            BasicSymbol::Literal(Literal::Bool(false)),
            reader.get_line_info(),
        ));
        buffer.clear();
        return Ok(());
    }

    if let Ok(val) = buffer.parse() {
        symbols.push((
            BasicSymbol::Literal(Literal::Int(val)),
            reader.get_line_info(),
        ));
        buffer.clear();
        return Ok(());
    }

    let mut sections = Vec::new();
    let mut section_buffer = String::new();
    let mut section_type = NameType::Normal;
    let mut last_separator = NameAccessType::Base;
    for c in buffer.chars() {
        if c == '.' {
            sections.push((section_buffer, last_separator, section_type));
            section_buffer = String::new();
            section_type = NameType::Normal;
            last_separator = NameAccessType::Normal;
            continue;
        }
        if c == '#' {
            sections.push((section_buffer, last_separator, section_type));
            section_buffer = String::new();
            section_type = NameType::Normal;
            last_separator = NameAccessType::Static;
            continue;
        }

        section_buffer.push(c);
    }

    sections.push((section_buffer, last_separator, section_type));

    if let Some(kwd) = Keyword::get_enum(&sections.first().unwrap().0) {
        if sections.len() > 1 {
            return Err(ParseError::KeywordFollowed(
                reader.get_line_info(),
                sections.first().unwrap().0.clone(),
            ));
        }
        symbols.push((BasicSymbol::Keyword(kwd), reader.get_line_info()));
        buffer.clear();
    } else {
        symbols.push((BasicSymbol::Name(sections), reader.get_line_info()));
        buffer.clear();
    }

    Ok(())
}

fn process_operator_buffer(
    buffer: &String,
    reader: &FileReader,
) -> Result<BasicSymbol, ParseError> {
    let operator = Operator::get_operator(buffer.as_str());
    if let Some(operator) = operator {
        return Ok(BasicSymbol::Operator(operator));
    } else if buffer == "=" {
        return Ok(BasicSymbol::Assigner(None));
    } else if buffer.chars().last().unwrap() == '=' {
        if let Some(operator) =
            Operator::get_operator(&buffer[..buffer.char_indices().last().unwrap().0])
        {
            return Ok(BasicSymbol::Assigner(Some(operator)));
        }
    }

    Err(ParseError::OperatorNotRecognised(
        reader.get_line_info(),
        buffer.clone(),
    ))
}
