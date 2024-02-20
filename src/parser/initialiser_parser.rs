use crate::ast::literals::Literal;
use crate::basic_ast::punctuation::Punctuation;
use crate::basic_ast::symbol::{BasicSymbol, NAME_VALID_CHARS};
use crate::parser::file_reader::FileReader;
use crate::parser::line_info::LineInfo;
use crate::parser::normal_parser::parse_normal;
use crate::parser::parse::{BlockType, ParseError};

pub fn parse_initialiser(symbols: &mut Vec<(BasicSymbol, LineInfo)>,
                     reader: &mut FileReader,
    ) -> Result<(), ParseError> {
    let line_info = reader.checkpoint();
    let (name, eof) = reader.move_read_to_next_char('{');
    for c in name.chars() {
        if !NAME_VALID_CHARS.contains(&c) {
            let mut utf8 = Vec::with_capacity(c.len_utf8());
            for _ in 0..c.len_utf8() { utf8.push(0); }
            c.encode_utf8(&mut utf8);
            return Err(ParseError::BadName(reader.get_line_info(), c, utf8));
        }
    }
    if name.is_empty() {
        return Err(ParseError::NoInitialiserType(reader.get_line_info_current()))
    }

    if eof {
        return Err(ParseError::NoInitialiserContents(reader.get_line_info_current()))
    }

    let BasicSymbol::BracedSection(attributes) = parse_normal(reader, BlockType::Braces)? else { panic!() };
    let mut attributes: Vec<Vec<(BasicSymbol, LineInfo)>> = attributes.into_iter().fold(
        Vec::new(),
        |mut v, next| {
            let last =
                if let Some(last) = v.last_mut() { last }
                else { v.push(Vec::new()); v.last_mut().unwrap() };

            if matches!(next.0, BasicSymbol::Punctuation(Punctuation::ListSeparator)) {
                v.push(Vec::new())
            }
            else {
                last.push(next)
            }
            v
        }
    );

    for (i, attribute) in attributes.iter().enumerate() {
        if attribute.is_empty() && i + 1 != attributes.len() {
            return Err(ParseError::NoInitialiserAttribute(reader.get_line_info_current()))
        }
    }

    if attributes.last().is_some_and(|x| x.is_empty()) {
        attributes.pop();
    }

    symbols.push((BasicSymbol::Literal(Literal::Initialiser(name, attributes)), reader.get_line_info_changed(line_info.0, line_info.1)));

    Ok(())
}