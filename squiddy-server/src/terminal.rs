use termion::{ clear, /*color,*/ cursor, /*event, input, scroll, style, */
    /*input::TermRead, raw::IntoRawMode,*/
    //AsyncReader,
    async_stdin };
use std::io::{ Read, Result, Write };
use std::thread;
use std::time::Duration;
use std::sync::{ Arc, RwLock };
use super::state::State;

pub struct Terminal<'o> {
    out: &'o mut Write,
    current_view: View,
    exit_requested: bool,
    state: Arc<RwLock<State>>
}

impl<'o> Terminal<'o> {
    /// Create a new `Terminal` wrapper around the mutable Write reference (which is assumed to be
    /// an ANSI terminal).
    pub fn new<W: Write>(out: &'o mut W, state: Arc<RwLock<State>>) -> Self {
        Terminal {
            out: out,
            current_view: View::AgentList,
            exit_requested: false,
            state: state
        }
    }

    fn clear(&mut self) {
        write!(self.out, "{}", clear::All).unwrap();
    }

    fn reposition(&mut self) {
        write!(self.out, "{}", cursor::Goto(1, 1)).unwrap();
    }

    fn draw_frame(&mut self, title: String, width: u16, height: u16, lines: Vec<String>) {
        let top_frame = format!("+{:-^32}+", title);
        write!(self.out, "{}", top_frame);
    }

    /// Release the terminal and assume that it isn't controlled any more by this instance.
    pub fn release(&mut self) {
        write!(self.out, "{}", cursor::Show).unwrap();
    }

    /// Return the size of the terminal in terms of number of columns and rows.
    fn size(&mut self) -> Result<(u16, u16)> {
        ::termion::terminal_size()
    }

    pub fn initialise_screen(&mut self) {
        write!(self.out, "{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Hide).unwrap();
    }

    pub fn start(&mut self) {
        let mut stdin = async_stdin().bytes();

        self.render_view();

        while !self.exit_requested {
            self.update_view();

            if let Some(Ok(read_byte)) = stdin.next() {
                self.handle_input(read_byte);
            }

            self.out.flush().unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    }

    fn handle_input(&mut self, read_byte: u8) {
        match read_byte {
            b'q' => self.exit_requested = true,
            _ => ()
        }
    }

    fn render_view(&mut self) {
        match self.current_view {
            View::AgentList => self.render_agent_list()
        }
    }

    fn update_view(&mut self) {
        if let Ok(state) = self.state.try_read() {
            write!(self.out, "{}", cursor::Goto(1, 2)).unwrap();

            for agent in &state.registered_agents {
                write!(self.out, "{} {} {}", agent.agent_id, agent.address, agent.app).unwrap();
            }
        }
    }

    fn render_agent_list(&mut self) {
        self.initialise_screen();
        write!(self.out, "{} {} {}", "NAME", "ADDRESS", "APP").unwrap();
    }

}

enum View {
    AgentList
}

impl<'o> Drop for Terminal<'o> {
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
        //           <------- show cursor --->
                vec![ 27, 91, 63, 50, 53, 104 ]);
    }

    #[test]
    fn test_clear() {
        compare(vec![ (|t| t.clear()) ],
        //            < clear screen> <------- show cursor --->
                vec![ 27, 91, 50, 74, 27, 91, 63, 50, 53, 104 ]);
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
