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

fn printhelp(){
    println!("Terminal based calculator");
    println!("Options(based on input arguments): ");
    println!("\t --help, prints this message");
    println!("\t a math expression, solves it and prints the result");
    println!("\t no arguments, enters the mainscreen ui");
    println!("The program uses standard pemdas to evaluate expressions, modulus is same priority as multiplication and division");
    println!("The program has support for: ");
    println!("\t operators: +, -, *(multiplication), /(division), %(modulus), and ^(exponentiation)");
    println!("\t functions: sin(sine, all trig functions are in radians), cos(cosine), tan(tangent), floor, ceil(ceiling), round, ln(natural logarithm), and abs(absolute value, NOT done via '|')");
    println!("\t Some functions use multiple parameters and are seperated via a comma(like f(x, y)) they include: log(logarithm first parameter is the base second is the argument), root(nth root of a number; first parameter is the n value, and the second is the number being taken the root of)");
    println!("\t variables: e(euler's number), and pi");
}

fn main() {
    let mut argv = env::args();
    let argc = argv.len();
    argv.next();

    if argc == 1{
        mainscreen::ui();
    }
    else{
        let args: Vec<String> = argv.collect();
        if args[0] == "--help"{
            printhelp();
        }
        else {
            let mut input: String = String::new();
            for arg in args{
                input.push_str(arg.as_str());
                input.push(' ');
            }
            //algorithm adds an extra space this removes it
            input.pop();
            match fundamentals::solve(input){
                Ok(n) => println!("{n}"),
                Err(e) => eprintln!("{e}"),
            }
        }
    }
}
