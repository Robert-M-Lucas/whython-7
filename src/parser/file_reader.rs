use crate::parser::parse::ParseError;
use std::path::PathBuf;


pub struct FileReader {
    path: PathBuf,
    data: String,
    cursor: usize,
    line: usize,
}

impl FileReader {
    pub fn new(path: PathBuf, data: String) -> FileReader {
        FileReader {
            path,
            data,
            cursor: 0,
            line: 1,
        }
    }

    pub fn syntax_error(&self, message: String) -> ParseError {
        ParseError::Syntax(self.path.clone(), self.line, message)
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn move_read_any(&mut self) -> Option<char> {
        let c = self.data.chars().nth(self.cursor);
        if c.is_some() {
            self.cursor += 1;
            if c.unwrap() == '\n' {
                self.line += 1;
            }
        }
        c
    }

    pub fn read_until_char(&self, c: char) -> (String, bool) {
        let mut out = String::new();

        let chars = self.data.chars().skip(self.cursor);

        let mut eof = true;

        for char in chars {
            if char == c {
                eof = false;
                break;
            }
            out.push(char);
        }

        (out, eof)
    }

    pub fn move_to_next_char(&mut self, c: char) {
        let chars = self.data.chars().skip(self.cursor);

        for char in chars {
            self.cursor += 1;
            if char == '\n' {
                self.line += 1;
            }
            if char == c {
                break;
            }
        }
    }

    pub fn move_read_to_next_char(&mut self, c: char) -> (String, bool) {
        let mut out = String::new();

        let chars = self.data.chars().skip(self.cursor);

        let mut eof = true;

        for char in chars {
            self.cursor += 1;

            if char == '\n' {
                self.line += 1;
            }

            if char == c {
                eof = false;
                break;
            }
            out.push(char);
        }

        (out, eof)
    }

    pub fn skip_whitespace(&mut self) -> bool {
        let chars = self.data.chars().skip(self.cursor);

        let mut eof = true;

        for char in chars {
            if !char.is_whitespace() {
                eof = false;
                break;
            }

            self.cursor += 1;
        }

        eof
    }

    pub fn all_read(&self) -> bool {
        self.cursor >= self.data.len()
    }
}
