use std::{borrow::BorrowMut, io::{self, BufRead, Stdin}};


pub enum InputKind {
    Cursor(Vec<io::Cursor<String>>),
    Stdin(Stdin),
}

pub struct Input {
    reader: InputKind
}

impl From<Stdin> for Input {
    fn from(s: Stdin) -> Self {
        Input {
            reader: InputKind::Stdin(s)
        }
    }
}

impl From<Vec<io::Cursor<String>>> for Input {
    fn from(c: Vec<io::Cursor<String>>) -> Self {
        Input {
            reader: InputKind::Cursor(c)
        }
    }
}

impl Input {

    pub fn read_line(&mut self, buf: &mut String) -> Result<usize, io::Error>  {
        match self.reader.borrow_mut() {
            InputKind::Cursor(c) => c.pop().unwrap().read_line(buf),
            InputKind::Stdin(s) => s.read_line(buf),
        }
    }
}