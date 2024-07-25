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

use std::collections::HashMap;
#[derive(Debug)]
enum TokenId{
    Num,
    Operator,
    Paranthese,
    FunctionName,
}

#[derive(Debug)]
struct Token{
    id: TokenId,
    value: String,
}

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
                if preprocessed_str[preproclen] != ""{
                    preprocessed_str.push(letter.to_string());
                    preproclen+=1;
                }
                else{
                    preprocessed_str[preproclen].push(letter);
                }
                preprocessed_str.push(String::new());
                preproclen+=1;
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
    //make a token stream for easier proccessing
    let vars = HashMap::from([("e", "2.71828182845904523536028747135266250"),("pi", "3.14159265358979323846264338327950288")]);
    let funcs: [String; 8] = ["sin".to_string(), "cos".to_string(), "tan".to_string(), "ceil".to_string(), "floor".to_string(), "sqrt".to_string(), "round".to_string(), "abs".to_string()];
    let mut token_stream: Vec<Token> = Vec::new();
    'tokenloop : for (i, item) in preprocessed_str.iter().enumerate(){
        if item == "+" || item == "*" || item == "/" || item == "%" || item == "^" || item == "!"{
            token_stream.push(Token{
                id: TokenId::Operator,
                value: preprocessed_str[i].to_string(),
            });
        }
        else if item == "(" || item == ")"{
            token_stream.push(Token{
                id: TokenId::Paranthese,
                value: preprocessed_str[i].to_string(),
            });
        }
        else if item.parse::<f64>().is_ok(){
            token_stream.push(Token{
                id: TokenId::Num,
                value: preprocessed_str[i].to_string(),
            });
        }
        else if vars.contains_key(&item.as_str()){
            token_stream.push(Token{
                id: TokenId::Num,
                value: vars[&item.as_str()].to_string(),
            });
        }
        else {
            for func in funcs.iter(){
                if func == item{
                    token_stream.push(Token{
                        id: TokenId::FunctionName,
                        value: preprocessed_str[i].to_string(),
                    });
                    continue 'tokenloop;
                }
            }
            panic!("A part of the expression typed could not be indentified as a number, operator, or paranthese!");
        }
    }
    println!("{:#?}", token_stream);
    return res;
}
