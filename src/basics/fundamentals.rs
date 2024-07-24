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

pub fn solve(mut input: String) -> f64{
    // remove spaces so that they don't cause pain
    input.retain(|c| c != ' ');
    let res: f64 = 0.0;
    //preprocess the input string
    let mut i: usize = 0;
    let mut chars = input.chars();
    let mut preproclen: usize = 0;
    let mut preprocessed_str: Vec<String> = Vec::new();
    preprocessed_str.push(String::new());
    while let Some(letter) = chars.next(){
        match letter{
            '(' | '+' | '*' | '/' | '%' | '^' | '!' =>{
                preprocessed_str.push(letter.to_string());
                preprocessed_str.push(String::new());
                preproclen+=2;
            },
            ')' => {
                preprocessed_str.push(letter.to_string());
                preproclen+=1;
            }
            '-' => {
                if i > 0{
                    let n = input.chars().nth(i-1).unwrap();
                    //if the previous letter is a number then it is a minus sign or is a closing
                    //paranthese
                    if n.to_digit(10) != None || n == '.'  || n == ')'{
                        preprocessed_str.push(letter.to_string());
                        preprocessed_str.push(String::new());
                        preproclen+=2;
                    }
                    else {
                        preprocessed_str[preproclen].push(letter);
                    }
                }
                else {
                    preprocessed_str[preproclen].push(letter);
                }

            },
            _ => preprocessed_str[preproclen].push(letter),
        }
        i+=1;
    }
    println!("{:?}", preprocessed_str);
    return res;
}
