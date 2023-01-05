use mexpr::expr::*;
//use mexpr::parse::*;
use std::{io, rc::Rc};

fn main() {
    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("Failed to read line.");
    match Expression::try_from(string.as_str()) {
        Ok(mut expr) => {
            expr.table.insert("π".to_owned(), Value::Number(std::f64::consts::PI));
            expr.table.insert("τ".to_owned(), Value::Number(2.0 * std::f64::consts::PI));
            expr.table.insert("sin".to_owned(), Value::Function(Rc::new(|args| {
                match args.as_slice() {
                    [Value::Number(arg)] => Ok(Value::Number(arg.sin())),
                    _ => Err(Box::new(InvalidArguments(args))),
                }
            })));
            expr.table.insert("cos".to_owned(), Value::Function(Rc::new(|args| {
                match args.as_slice() {
                    [Value::Number(arg)] => Ok(Value::Number(arg.cos())),
                    _ => Err(Box::new(InvalidArguments(args))),
                }
            })));
            expr.table.insert("tan".to_owned(), Value::Function(Rc::new(|args| {
                match args.as_slice() {
                    [Value::Number(arg)] => Ok(Value::Number(arg.tan())),
                    _ => Err(Box::new(InvalidArguments(args))),
                }
            })));
            expr.table.insert("asin".to_owned(), Value::Function(Rc::new(|args| {
                match args.as_slice() {
                    [Value::Number(arg)] => Ok(Value::Number(arg.asin())),
                    _ => Err(Box::new(InvalidArguments(args))),
                }
            })));
            expr.table.insert("acos".to_owned(), Value::Function(Rc::new(|args| {
                match args.as_slice() {
                    [Value::Number(arg)] => Ok(Value::Number(arg.acos())),
                    _ => Err(Box::new(InvalidArguments(args))),
                }
            })));
            expr.table.insert("atan".to_owned(), Value::Function(Rc::new(|args| {
                match args.as_slice() {
                    [Value::Number(arg)] => Ok(Value::Number(arg.atan())),
                    _ => Err(Box::new(InvalidArguments(args))),
                }
            })));
            match expr.eval() {
                Ok(value) => println!("{:?}", value),
                Err(error) => println!("{}", error),
            }
        },
        Err(err) => println!("ParseError: {}", err),
    };
    /*match Parser::from(string.as_str()).parse() {
        Ok(tree) => {
            println!("{:?}", tree)
        },
        Err(err) => println!("{}", err),
    }*/
}
