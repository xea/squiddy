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

impl<'a> Drop for Terminal<'a> {
    /// When this instance goes out of scope, we'll release the terminal
    fn drop(&mut self) {
        self.release();
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::io::BufWriter;

    #[test]
    fn test_initialise() {
        compare(vec![ ],
        //          < clear screen >  <----- goto 1,1 ------> <---- hide cursor ---->  <------- show cursor --->
                vec![ 27, 91, 50, 74, 27, 91, 49, 59, 49, 72, 27, 91, 63, 50, 53, 108, 27, 91, 63, 50, 53, 104 ]);
        compare(vec![ (|t| t.clear()) ],
                vec![
        //          < clear screen >  
                    27, 91, 50, 74,
        //          <----- goto 1,1 ------>
                    27, 91, 49, 59, 49, 72,
        //          <---- hide cursor ---->
                    27, 91, 63, 50, 53, 108,
        //          < clear screen >
                    27, 91, 50, 74,
        //          show cursor
                    27, 91, 63, 50, 53, 104 ]);
    }

    fn compare(actions: Vec<fn(&mut Terminal) -> ()>, expected: Vec<u8>) {
        let buf = vec![];
        let mut bw = BufWriter::new(buf);

        {
            let mut t = Terminal::new(&mut bw);

            for a in actions {
                a(&mut t);
            }
        }

        assert_eq!(expected, bw.into_inner().unwrap());

    }
}
