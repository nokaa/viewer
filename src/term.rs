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
    /// The terminal object that we use to modify our UI
    term: rustty::Terminal,
    /// The name of the file we are viewing
    filename: &'a str,
    /// The contents of `filename` as a `Vec` of lines.
    contents: Vec<Vec<u8>>,
    /// The running status of our UI
    quit: bool,
    /// The line at the top of the UI
    top_line: usize,
    /// The bottom most line of the UI
    bottom_line: usize,
    /// The total number of lines in our file
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
            bottom_line: 0,
            total_lines: total_lines,
        }
    }

    pub fn run(&mut self) {
        self.print_file();
        self.prompt();
        self.term.swap_buffers().unwrap();

        while !self.quit {
            if self.term.check_resize() {
                self.term.try_resize().unwrap();
                self.print_file();
                self.prompt();
                self.term.swap_buffers().unwrap();
            }

            let evt = self.term.get_event(100).unwrap();
            if let Some(Event::Key(ch)) = evt {
                match ch {
                    // quit
                    'q' => self.quit = true,
                    // Move down
                    'j' => {
                        if self.total_lines > self.term.rows() - 2 &&
                            self.bottom_line < self.total_lines - 1
                        {
                            self.top_line += 1;
                            self.print_file();
                            self.prompt();
                            self.term.swap_buffers().unwrap();
                        }
                    }
                    // Move up
                    'k' => {
                        if self.top_line > 0 {
                            self.top_line -= 1;
                            self.print_file();
                            self.prompt();
                            self.term.swap_buffers().unwrap();
                        }
                    }
                    // Go to top
                    'g' => {
                        if self.top_line > 0 {
                            self.top_line = 0;
                            self.print_file();
                            self.prompt();
                            self.term.swap_buffers().unwrap();
                        }
                    }
                    // Go to bottom
                    'G' => {
                        let len = self.total_lines - 1;
                        if len > self.term.rows() - 1 {
                            self.bottom_line = len;

                            self.print_file_reverse();
                            self.prompt();
                            self.term.swap_buffers().unwrap();
                        }
                    }
                    _ => { }
                }
            }
        }
    }

    fn print_file(&mut self) {
        let w = self.term.cols();
        let h = self.term.rows() - 1;
        let len = self.total_lines - 1;
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

                if j == w && i < h - 1 {
                    j = 0;
                    i += 1;
                } else if j == w && i < h {
                    break;
                } else if i >= h {
                    break;
                }
            }
            // Write blank spaces for rest of line. This makes sure that
            // if the line previously had more characters than it does now,
            // the old characters are deleted.
            while j < w {
                self.term[(j, i)].set_ch(' ');
                j += 1;
            }

            if top_line == len {
                self.bottom_line = top_line;
                return;
            }

            i += 1;
            top_line += 1;
        }

        self.bottom_line = top_line - 1;
    }

    fn print_file_reverse(&mut self) {
        let w = self.term.cols();
        let mut h = self.term.rows();
        let mut bottom_line = self.bottom_line;

        while h > 1 {
            let line = &self.contents[bottom_line];
            let mut j = 0;
            for &c in line {
                match c {
                    b'\n' => break,
                    b'\t' => {
                        for k in 0..4 {
                            self.term[(j+k, h-2)].set_ch(' ');
                        }
                        j += 4;
                    }
                    _ => {
                        self.term[(j, h-2)].set_ch(c as char);
                        j += 1;
                    }
                }

                if j == w && h > 1 {
                    j = 0;
                    h -= 1;
                } else if j == w && h > 0 {
                    break;
                } else if h == 0 {
                    break;
                }
            }

            // Write blank spaces for rest of line. This makes sure that
            // if the line previously had more characters than it does now,
            // the old characters are deleted.
            while j < w {
                self.term[(j, h-2)].set_ch(' ');
                j += 1;
            }

            bottom_line -= 1;
            h -= 1;
        }

        self.top_line = bottom_line + 1;
    }

    fn prompt(&mut self) {
        let w = self.term.cols();
        let h = self.term.rows() - 1;

        for i in 0..w {
            self.term[(i, h)].set_bg(Color::Red);
            self.term[(i, h)].set_ch(' ');
        }

        for (i, c) in self.filename.chars().enumerate() {
            self.term[(i, h)].set_ch(c);
        }
        self.prompt_line_number();
    }

    fn prompt_line_number(&mut self) {
        let w = self.term.cols();
        let h = self.term.rows() - 1;
        let len = format!("{}/{} lines", self.bottom_line+1, self.total_lines);

        for (i, c) in len.chars().rev().enumerate() {
            self.term[(w-i-1, h)].set_ch(c);
        }
    }
}
