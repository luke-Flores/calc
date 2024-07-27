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
    Openparanthese,
    ClosedParanthese,
    FunctionName,
}

#[derive(Debug)]
struct Token{
    id: TokenId,
    value: String, //used to store value or preccedense in order of operations
    num: f64,
}

fn preprocess(input: String, preprocessed_str: &mut Vec<String>){
    //preprocess the input string
    let mut i: usize = 0;
    let mut chars = input.chars();
    let mut preproclen: usize = 0;
    preprocessed_str.push(String::new());
    while let Some(letter) = chars.next(){
        match letter{
            '(' | '+' | '*' | '/' | '%' | '^' =>{
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
}

fn tokenize(preprocessed_str: Vec<String>, token_stream: &mut Vec<Token>){
    let vars = HashMap::from([("e", std::f64::consts::E),("pi", std::f64::consts::PI)]);
    let funcs: [String; 8] = ["sin".to_string(), "cos".to_string(), "tan".to_string(), "ceil".to_string(), "floor".to_string(), "sqrt".to_string(), "round".to_string(), "abs".to_string()];
    let mut func_count: Vec<usize> = Vec::new();
    let mut paran_count = 0;
    let mut previously_func = false;
    'tokenloop : for (i, item) in preprocessed_str.iter().enumerate(){
        if item == "+" || item == "-"{
            token_stream.push(Token{
                id: TokenId::Operator,
                value: preprocessed_str[i].to_string(),
                num: 1.0,
            });
        }
        else if item == "*" || item == "/" || item == "%"{
            token_stream.push(Token{
                id: TokenId::Operator,
                value: preprocessed_str[i].to_string(),
                num: 2.0,
            });
        }
        else if item == "^" {
           token_stream.push(Token{
                id: TokenId::Operator,
                value: String::from("^"),
                num: 3.0,
            });
        }
        else if item == "("{
            token_stream.push(Token{
                id: TokenId::Openparanthese,
                value: String::from("("),
                num: 0.0,
            });
            if previously_func{
                func_count.push(paran_count);
                previously_func = false;
            }
            paran_count+=1;
        }
        else if item == ")"{
            token_stream.push(Token{
                id: TokenId::ClosedParanthese,
                value: String::from(")"),
                num: 0.0,
            });
            if paran_count > 0 {
                paran_count-=1;
                'funcloop : for count in func_count.iter(){
                    if *count == paran_count{
                        //get around the lack of unary operator handeling in the infix to postfix
                        //function
                        token_stream.push(Token{
                            id: TokenId::Num,
                            value: String::from("0"),
                            num: 0.0,
                        });
                        func_count.pop();
                        break 'funcloop;
                    }
                }
            }
            else{
                panic!("A closing paranthese appeared without a subsequent opening paranthese");
            }
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
                num: vars[&item.as_str()]
            });
        }
        else {
            for func in funcs.iter(){
                if func == item{
                    token_stream.push(Token{
                        id: TokenId::FunctionName,
                        value: preprocessed_str[i].to_string(),
                        num: 4.0,
                    });
                    if previously_func{
                        panic!("A function name appeared without a following paranthese");
                    }
                    previously_func = true;
                    continue 'tokenloop;
                }
            }
            panic!("A part of the expression typed could not be indentified as a number, operator, or paranthese!");
        }
        if previously_func{
            panic!("A function name appeared without a following paranthese");
        }
    }
    if previously_func{
        panic!("A function name appeared without a following paranthese");
    }
    else if paran_count != 0{
        panic!("There are unclosed parantheses in the expression");
    }
}

fn infix_to_postfix(token_stream: Vec<Token>, postfix_equation: &mut Vec<Token>){
    let mut stack: Vec<Token> = Vec::new();
    for token in token_stream{
        match token.id{
            TokenId::Num => postfix_equation.push(token),
            TokenId::Openparanthese => stack.push(token),
            TokenId::ClosedParanthese => {
                'parantheseLoop : loop{
                    match stack.last(){
                        None => panic!("A closed paranthese did not match an open paranthese"),
                        Some(n) => {
                            match n.id {
                                TokenId::Openparanthese => break 'parantheseLoop,
                                _ => postfix_equation.push(stack.pop().unwrap()),
                            }
                        },
                    }
                }
                stack.pop();
            }
            TokenId::Operator | TokenId::FunctionName=> {
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
}

fn evaluate(equation: Vec<Token>, result: &mut f64){
    let mut stack: Vec<f64> = Vec::new();
    for token in equation{
        match token.id{
            TokenId::Operator | TokenId::FunctionName => {
                let val1;
                let val2;
                //operands are switched so they come in the order expected
                match stack.pop(){
                    Some(n) => val2 = n,
                    None => panic!("Operator appeared without enough values"),
                }
                match stack.pop(){
                    Some(n) => val1 = n,
                    None => panic!("Operator appeared without enough values"),
                }
                match token.value.as_str(){
                    "+" => stack.push(val1+val2),
                    "-" => stack.push(val1-val2),
                    "*" => stack.push(val1*val2),
                    "/" => stack.push(val1/val2),
                    "%" => stack.push(val1%val2),
                    "^" => stack.push(val1.powf(val2)),
                    //all of these ignore val2
                    "sin" => stack.push(val1.sin()),
                    "cos" => stack.push(val1.cos()),
                    "tan" => stack.push(val1.tan()),
                    "ceil" => stack.push(val1.ceil()),
                    "floor" => stack.push(val1.floor()),
                    "round" => stack.push(val1.round()),
                    "sqrt" => stack.push(val1.sqrt()),
                    "abs" => stack.push(val1.abs()),
                    _ => panic!("Somehow something that shouldn't be an operator or function got coded as one this is most likely a problem with the program"),
                }
            },
            TokenId::Num => stack.push(token.num),
            _ => panic!("Something went wrong in making the equation"),
        }
    }
    match stack.pop(){
        None => panic!("No value computed somehow"),
        Some(n) => *result = n,
    }
}

pub fn solve(mut input: String) -> f64{
    let mut res: f64 = 0.0;
    // remove spaces so that they don't cause pain
    input.retain(|c| c != ' ');
    let mut preprocessed_str: Vec<String> = Vec::new();
    preprocess(input, &mut preprocessed_str);

    //make a token stream for easier proccessing
    let mut token_stream: Vec<Token> = Vec::new();
    tokenize(preprocessed_str, &mut token_stream);

    // convert expression to postfix
    let mut postfix_equation: Vec<Token> = Vec::new();
    infix_to_postfix(token_stream, &mut postfix_equation);

    evaluate(postfix_equation, &mut res);
    return res;
}
