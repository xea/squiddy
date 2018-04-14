extern crate termion;

mod terminal;

use std::io::stdout;
use termion::raw::{ IntoRawMode };
use terminal::Terminal;

fn main() {
    if let Ok(mut terminal) = stdout().into_raw_mode() {
        let out = &mut terminal;
        let mut terminal = Terminal::new(out);

        terminal.release();
    }
}
