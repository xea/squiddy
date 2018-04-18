use termion::{ clear, /*color,*/ cursor, /*event, input, scroll, style, */
    /*input::TermRead, raw::IntoRawMode,*/
    //AsyncReader,
    async_stdin };
use std::io::{ Read, Result, Write };
use std::thread;
use std::time::Duration;
use std::sync::{ Arc, RwLock };
use super::state::State;
use agent::Agent;

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
            out,
            state,
            current_view: View::AgentList,
            exit_requested: false
        }
    }

    /*
    fn clear(&mut self) {
        write!(self.out, "{}", clear::All).unwrap();
    }

    fn reposition(&mut self) {
        write!(self.out, "{}", cursor::Goto(1, 1)).unwrap();
    }

    fn draw_frame(&mut self, title: String, width: u16, height: u16, lines: Vec<String>) {
        let top_frame = format!("+{:-^32}+", title);
        write!(self.out, "{}", top_frame);
    }*/

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
        // input isn't a top priority so we want to read keyboard events asynchronously to minimise
        // the amount of time spent on blocking calls.
        let mut stdin = async_stdin().bytes();

        // print out the initial screen, this may be changed later at the user's request
        self.render_view();

        while !self.exit_requested {
            // we don't want to repaint the whole screen every time (it would be expensive) so we'll
            // just redraw the parts that may actually change
            self.update_view();

            if let Some(Ok(read_byte)) = stdin.next() {
                self.handle_input(read_byte);
            }

            self.out.flush().unwrap();

            // a 50ms sleep should be low enough in order to provide a comfortable user experience
            // and keep a relatively low cpu usage
            thread::sleep(Duration::from_millis(50));
        }
    }

    fn handle_input(&mut self, read_byte: u8) {
        match read_byte {
            b'q' => self.exit_requested = true,
            _ => ()
        }
    }

    /// `render_view` issues a full repaint of the screen drawing out the selected view based on the
    /// current state.
    fn render_view(&mut self) {
        match self.current_view {
            View::AgentList => self.render_agent_list()
        }
    }

    fn update_view(&mut self) {
        if let Ok(state) = self.state.try_read() {
            let output = match self.current_view {
                View::AgentList => self.update_agent_list(&state.registered_agents),
            };

            write!(self.out, "{}", output);
        }
    }

    fn render_agent_list(&mut self) {
        self.initialise_screen();
        write!(self.out, "{:32} {:20} {:10}", "NAME", "ADDRESS", "APP").unwrap();
        self.update_view();
    }

    fn update_agent_list(&self, agents: &Vec<Agent>) -> String {
        let mut output = format!("{}", cursor::Goto(1, 2));

        for agent in agents {
            output += &format!("{:32} {:20} {:10}", agent.agent_id, agent.address, agent.app);
        }

        output
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
        let mut state = State::new();
        let mut state = Arc::new(RwLock::new(state));

        {
            let mut t = Terminal::new(&mut bw, state.clone());

            for a in actions {
                a(&mut t);
            }
        }

        assert_eq!(expected, bw.into_inner().unwrap());

    }
}
