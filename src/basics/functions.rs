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

/*
    This module deals with functions and operations regarding the function itself(typically calculus)
*/

use super::fundamentals;
use std::collections::HashMap;

pub struct Functions{
    function_mem: HashMap<String, Vec<fundamentals::Token>>,
}

impl Functions{
    // adds a function in postfix tokenized form
    pub fn add_function(&mut self, math: fundamentals::Interpreter, name: String, strval: String) -> Option<&'static str>{
        match math.get_tokens(strval){
            Err(e) => return Some(e),
            Ok(n) => self.function_mem.insert(name, n),
        };

        return None;
    }
    pub fn evaluate(&self, math: fundamentals::Interpreter, name: String, xval: f64)-> Result<f64, &'static str>{
        let expression;
        let mut expression_without_x: Vec<fundamentals::Token> = Vec::new();
        match self.function_mem.get(&name){
            None => return Err("Function does not exist"),
            Some(n) => expression = n,
        };
        for token in expression{
            match token.id{
                fundamentals::TokenId::XVar =>{
                    expression_without_x.push(fundamentals::Token{
                        id: fundamentals::TokenId::Num,
                        value: String::from("X"),
                        num: xval,
                    });
                },
                _ => expression_without_x.push(token.clone()),
            };
        }
        let mut res: f64 = 0.0;
        match math.evaluate(expression_without_x, &mut res){
            None => return Ok(res),
            Some(n) => return Err(n),
        }
    }
}
