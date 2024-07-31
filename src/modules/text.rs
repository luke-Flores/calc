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
use std::io::Write;

pub fn textcalc(){
    println!("Type math expressions. Type \"exit\" to return to the main menu");
    loop {
        let mut input = String::new();
        print!(">>> ");
        let _ = std::io::stdout().flush();
        let _ = std::io::stdin().read_line(&mut input);
        input.pop();

        if input.get(0..4) == Some(&"exit"){
            break;
        }
        let res = crate::basics::fundamentals::solve(input);
        match res{
            Err(e) => println!("There was a problem: {e}"),
            Ok(n) => println!("{n}"),
        }
    }
}
