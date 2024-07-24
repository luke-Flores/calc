/*
    calc - terminal based calculator
    Copyright (C) 2024  Luke Flores

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use std::env;
use calc::basics::*;

fn main() {
    let mut argv = env::args();
    let argc = argv.len();
    argv.next();

    if argc == 1{
        mainscreen::ui();
    }
    else{
        let mut input: String = String::new();
        for arg in argv{
            input.push_str(arg.as_str());
            input.push(' ');
        }
        //algorithm adds an extra space this removes it
        input.pop();
        fundamentals::solve(input);
    }
}
