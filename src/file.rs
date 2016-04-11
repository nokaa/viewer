/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU General Public License version 3. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/gpl.txt
 */

use std::fs::File;
use std::io::{self, Read};

pub fn read_file_lines(filename: &str)
    -> Result<Vec<Vec<u8>>, io::Error>
{
    let f = try!(File::open(filename));
    let mut lines: Vec<Vec<u8>> = vec![];
    let mut line: Vec<u8> = vec![];

    for byte in f.bytes() {
        match byte {
            Err(e) => return Err(e),
            Ok(b) => match b {
                b'\n' => {
                    line.push(b);
                    lines.push(line);
                    line = vec![];
                }
                b'\r' => { }
                _ => line.push(b),
            },
        }
    }

    Ok(lines)
}
