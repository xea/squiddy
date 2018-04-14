use termion::{ clear, color, cursor, event, input, scroll, style };
use std::io::{ Result, Write, stdin, stdout };

pub struct Terminal<'a> {
    out: &'a mut Write
}

impl<'a> Terminal<'a> {
    /// Create a new `Terminal` wrapper around the mutable Write reference (which is assumed to be
    /// an ANSI terminal).
    pub fn new<W: Write>(out: &'a mut W) -> Self {
        // As part of RAII initialisation, we want to start with a blank slate
        write!(out, "{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Hide).unwrap();

        Terminal {
            out: out
        }
    }

    pub fn clear(&mut self) {
        write!(self.out, "{}", clear::All).unwrap();
    }

    /// Release the terminal and assume that it isn't controlled any more by this instance.
    pub fn release(&mut self) {
        write!(self.out, "{}", cursor::Show).unwrap();
    }

    /// Return the size of the terminal in terms of number of columns and rows.
    pub fn size(&mut self) -> Result<(u16, u16)> {
        ::termion::terminal_size()
    }
}
