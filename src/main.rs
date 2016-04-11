/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU General Public License version 3. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/gpl.txt
 */

extern crate clap;
extern crate rustty;

mod file;
mod term;

use clap::App;

fn main() {
    let matches = App::new("forge")
        .version("v0.1")
        .author("nokaa <nokaa@cock.li>")
        .about("A fancy cli file viewer")
        .arg_from_usage("<FILE> 'The file to be viewed'")
        .get_matches();

    let filename = matches.value_of("FILE").unwrap();
    let mut term = term::Term::new(filename);
    term.run();
}
