/* calc - terminal based calculator
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

#[derive(Debug, PartialEq)]
enum TokenId{
    Num,
    Operator,
    OpenParanthese,
    ClosedParanthese,
}

#[derive(Debug)]
struct Token{
    id: TokenId,
    value: String, //used to store value or preccedense in order of operations
    num: f64,
}

fn preprocess(input: String, preprocessed_str: &mut Vec<String>){
    preprocessed_str.push(String::new());
    let mut tmp: String = String::new();
    for letter in input.chars(){
        match letter{
            '(' | '+' | '*' | '/' | '%' | '^' | ',' | ')' | '-' =>{
                preprocessed_str.push(tmp);
                preprocessed_str.push(letter.to_string());
                tmp = String::new()
            },
            _ => tmp.push(letter),
        }
    }
    preprocessed_str.push(tmp);
    //remove empty strings
    preprocessed_str.retain(|s| s != "");
}

fn tokenize(preprocessed_str: Vec<String>, token_stream: &mut Vec<Token>) -> Option<&'static str>{
    // Some predefined funtions or vars
    let vars = HashMap::from([("e", std::f64::consts::E),("pi", std::f64::consts::PI)]);
    let funcs: [String; 9] = ["sin".to_string(), "cos".to_string(), "tan".to_string(), "ceil".to_string(), "floor".to_string(), "sqrt".to_string(), "round".to_string(), "abs".to_string(), "ln".to_string()];
    let bin_funcs: [String; 2] = ["log".to_string(), "root".to_string()];

    //variables to help keep track of stuff
    let mut func_count: Vec<usize> = Vec::new();
    let mut binfunc_count: Vec<usize> = Vec::new();
    let mut paran_count = 0;
    let mut tmp: Vec<Token> = Vec::new();
    let mut skip = false;

    for (i, item) in preprocessed_str.iter().enumerate(){
        if skip{
            skip = false;
            continue;
        }
        else if item.parse::<f64>().is_ok(){
            token_stream.push(Token{
                id: TokenId::Num,
                value: preprocessed_str[i].to_string(),
                num: item.parse::<f64>().unwrap(),
            });
        }
        else if vars.contains_key(&item.as_str()){
            token_stream.push(Token{
                id: TokenId::Num,
                value: vars[&item.as_str()].to_string(),
                num: vars[&item.as_str()],
            });
        }
        else if funcs.contains(item){
            token_stream.push(Token{
                id: TokenId::Operator,
                value: preprocessed_str[i].to_string(),
                num: 4.0,
            });
            if preprocessed_str.get(i+1) == Some(&"(".to_string()){
                token_stream.push(Token{
                    id: TokenId::OpenParanthese,
                    value: String::from("("),
                    num: 0.0,
                });
                func_count.push(paran_count);
                paran_count+=1;
                skip = true;
            }
        }
        else if bin_funcs.contains(item){
            tmp.push(Token{
                id: TokenId::Operator,
                value: preprocessed_str[i].to_string(),
                num: 4.0,
            });
            if preprocessed_str.get(i+1) == Some(&"(".to_string()){
                token_stream.push(Token{
                    id: TokenId::OpenParanthese,
                    value: String::from("("),
                    num: 0.0,
                });
                paran_count+=1;
                binfunc_count.push(paran_count);
                skip = true;
            }
            else{
                return Some("A function name appeared without a following paranthese");
            }
        }
        else {
            match item.as_str(){
                "+"=> {
                    token_stream.push(Token{
                        id: TokenId::Operator,
                        value: String::from("+"),
                        num: 1.0,
                    });
                }
                "-" => {
                    match token_stream.last(){
                        None => {
                            token_stream.push(Token{
                                id: TokenId::Num,
                                value: String::from("-1"),
                                num: -1.0,
                            });
                            token_stream.push(Token{
                                id: TokenId::Operator,
                                value: String::from("*"),
                                num: 2.0,
                            });
                        }
                        Some(n) => {
                            match n.id{
                                TokenId::Operator | TokenId::OpenParanthese => {
                                    token_stream.push(Token{
                                        id: TokenId::Num,
                                        value: String::from("-1"),
                                        num: -1.0,
                                    });
                                    token_stream.push(Token{
                                        id: TokenId::Operator,
                                        value: String::from("*"),
                                        num: 2.0,
                                    });
                                    continue;
                                },
                                TokenId::Num | TokenId::ClosedParanthese => {
                                    token_stream.push(Token{
                                        id: TokenId::Operator,
                                        value: String::from("-"),
                                        num: 1.0,
                                    });
                                }
                            }
                        }
                    }
                }
                "*" | "/" | "%" => {
                    token_stream.push(Token{
                        id: TokenId::Operator,
                        value: preprocessed_str[i].to_string(),
                        num: 2.0,
                    });
                }
                "^" => {
                    token_stream.push(Token{
                        id: TokenId::Operator,
                        value: String::from("^"),
                        num: 3.0,
                    });
                }
                "(" => {
                    token_stream.push(Token{
                        id: TokenId::OpenParanthese,
                        value: String::from("("),
                        num: 0.0,
                    });
                    paran_count+=1;
                }
                ")" => {
                    token_stream.push(Token{
                        id: TokenId::ClosedParanthese,
                        value: String::from(")"),
                        num: 0.0,
                    });
                    //get around the lack of unary operator handeling in the infix to postfix function
                    if paran_count > 0 {
                        paran_count-=1;
                        if func_count.contains(&paran_count){
                            token_stream.push(Token{
                                id: TokenId::Num,
                                value: String::from("0"),
                                num: 0.0,
                            });
                            func_count.pop();
                        }
                    }
                    else{
                        return Some("A closing paranthese appeared without a subsequent opening paranthese");
                    }
                    if binfunc_count.last() != None && binfunc_count.last() != Some(&paran_count){
                        return Some("A closing paranthese appeared before a comma could appear in a multi parameter function");
                    }
                }
                "," => {
                    if binfunc_count.last() == Some(&paran_count){
                        //the paranthese are so that the value of the parameters get evualuated before the function is evaluated
                        token_stream.push(Token{
                            id: TokenId::ClosedParanthese,
                            value: String::from(")"),
                            num: 0.0,
                        });
                        token_stream.push(tmp.pop().unwrap());
                        token_stream.push(Token{
                            id: TokenId::OpenParanthese,
                            value: String::from("("),
                            num: 0.0,
                        });
                        binfunc_count.pop();
                    }
                    else {
                        return Some("A comma appeared outside of(or nested within a lower paranthese level) a function that accepts two parameters");
                    }
                }
                _ => return Some("A part of the expression typed could not be indentified as a number, operator, or paranthese!"),
            }
        }
    }
    if paran_count != 0{
        return Some("There are unclosed parantheses");
    }
    return None
}

fn infix_to_postfix(token_stream: Vec<Token>, postfix_equation: &mut Vec<Token>) -> Option<&'static str>{
    let mut stack: Vec<Token> = Vec::new();
    for token in token_stream{
        match token.id{
            TokenId::Num => postfix_equation.push(token),
            TokenId::OpenParanthese => stack.push(token),
            TokenId::ClosedParanthese => {
                'parantheseLoop : loop{
                    match stack.last(){
                        None => return Some("A closed paranthese did not match an open paranthese"),
                        Some(n) => {
                            match n.id {
                                TokenId::OpenParanthese => break 'parantheseLoop,
                                _ => postfix_equation.push(stack.pop().unwrap()),
                            }
                        },
                    }
                }
                stack.pop();
            }
            TokenId::Operator=> {
                'stackLoop : while stack.len() != 0{
                    //unwrap operation can be assumed since length is not 0
                    if token.num <= stack.last().unwrap().num{
                        postfix_equation.push(stack.pop().unwrap());
                    }
                    else{
                        break 'stackLoop;
                    }
                }
                stack.push(token);
            },
        }
    }
    while let Some(n) = stack.pop(){
        postfix_equation.push(n);
    }
    return None;
}

fn evaluate(equation: Vec<Token>, result: &mut f64) -> Option<&'static str>{
    let mut stack: Vec<f64> = Vec::new();
    for token in equation{
        match token.id{
            TokenId::Operator => {
                let val1;
                let val2;
                //operands are switched so they come in the order expected
                match stack.pop(){
                    Some(n) => val2 = n,
                    None => return Some("Operator appeared without enough values"),
                }
                match stack.pop(){
                    Some(n) => val1 = n,
                    None => return Some("Operator appeared without enough values"),
                }
                match token.value.as_str(){
                    "+" => stack.push(val1+val2),
                    "-" => stack.push(val1-val2),
                    "*" => stack.push(val1*val2),
                    "/" => stack.push(val1/val2),
                    "%" => stack.push(val1%val2),
                    "^" => stack.push(val1.powf(val2)),
                    "log" => stack.push(val2.log(val1)),
                    // a^(1/b) is equal to the bth root of a
                    "root" => stack.push(val2.powf(1.0/val1)),
                    //all of these ignore val2
                    "sin" => stack.push(val1.sin()),
                    "cos" => stack.push(val1.cos()),
                    "tan" => stack.push(val1.tan()),
                    "ceil" => stack.push(val1.ceil()),
                    "floor" => stack.push(val1.floor()),
                    "round" => stack.push(val1.round()),
                    "sqrt" => stack.push(val1.sqrt()),
                    "abs" => stack.push(val1.abs()),
                    "ln" => stack.push(val1.ln()),
                    _ => return Some("Somehow something that shouldn't be an operator or function got coded as one this is most likely a problem with the program"),
                }
            },
            TokenId::Num => stack.push(token.num),
            _ => return Some("Something went wrong in making the equation"),
        }
    }
    match stack.pop(){
        None => return Some("No value computed somehow"),
        Some(n) => *result = n,
    }
    return None;
}

pub fn solve(mut input: String) -> Result<f64, &'static str>{
    // remove spaces so that they don't cause pain
    input.retain(|c| c != ' ');
    let mut preprocessed_str: Vec<String> = Vec::new();
    preprocess(input, &mut preprocessed_str);

    //make a token stream for easier proccessing
    let mut token_stream: Vec<Token> = Vec::new();
    match tokenize(preprocessed_str, &mut token_stream){
        None => (),
        Some(n) => return Err(n),
    }

    // convert expression to postfix
    let mut postfix_equation: Vec<Token> = Vec::new();
    match infix_to_postfix(token_stream, &mut postfix_equation){
        None => (),
        Some(n) => return Err(n),
    }

    //actually evaluate the expression
    let mut res: f64 = 0.0;
    match evaluate(postfix_equation, &mut res){
        None => (),
        Some(n) => return Err(n),
    }
    return Ok(res);
}
