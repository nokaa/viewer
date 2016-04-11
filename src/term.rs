/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU General Public License version 3. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/gpl.txt
 */

use file;

use rustty::{self, Color, Event};

use std::io::{stderr, Write};
use std::process;

pub struct Term<'a> {
    term: rustty::Terminal,
    filename: &'a str,
    contents: Vec<Vec<u8>>,
    quit: bool,
    top_line: usize,
    line: usize,
    total_lines: usize,
}

impl<'a> Term<'a> {
    pub fn new(filename: &str) -> Term {
        let file = if let Ok(f) = file::read_file_lines(filename) {
            f
        } else {
            writeln!(stderr(), "Unable to read file {}", filename).unwrap();
            process::exit(-1);
        };

        let total_lines = file.len();

        Term {
            term: rustty::Terminal::new().unwrap(),
            filename: filename,
            contents: file,
            quit: false,
            top_line: 0,
            line: 0,
            total_lines: total_lines,
        }
    }

    pub fn run(&mut self) {
        self.print_file();
        self.prompt();
        self.term.swap_buffers().unwrap();

        while !self.quit {
            let evt = self.term.get_event(100).unwrap();
            if let Some(Event::Key(ch)) = evt {
                match ch {
                    'q' => self.quit = true,
                    _ => { }
                }
            }
        }
    }

    fn print_file(&mut self) {
        let w = self.term.cols();
        let h = self.term.rows() - 1;
        let len = self.contents.len() - 1;
        let mut top_line = self.top_line;

        let mut i = 0;
        while i < h {
            let line = &self.contents[top_line];
            let mut j = 0;
            for &c in line {
                match c {
                    b'\n' => break,
                    b'\t' => {
                        for k in 0..4 {
                            self.term[(j+k, i)].set_ch(' ');
                        }
                        j += 4;
                    }
                    _ => {
                        self.term[(j, i)].set_ch(c as char);
                        j += 1;
                    }
                }

                if j == w {
                    j = 0;
                    i += 1;
                }
            }

            i += 1;
            if top_line == len {
                break;
            }
            top_line += 1;
        }
    }

    fn prompt(&mut self) {
        let w = self.term.cols();
        let h = self.term.rows() - 1;

        for i in 0..w {
            self.term[(i, h)].set_bg(Color::Red);
        }

        for (i, c) in self.filename.chars().enumerate() {
            self.term[(i, h)].set_ch(c);
        }
    }
}
