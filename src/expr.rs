use crate::parse::*;
use std::{collections::HashMap, error::Error, rc::Rc, fmt::{Debug, Display}};

#[derive(Clone)]
pub enum Value {        
    Number(f64),
    Function(Rc<dyn Fn(Vec<Value>) -> Result<Value, Box<dyn Error>>>),
}
impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => write!(f, "Number({})", num),
            Self::Function(_) => write!(f, "Function"),
        }
    }
}

#[derive(Debug)]
pub enum ExpressionError {
    UndefinedIdentifier(String),
    FunctionCallFailure(Box<dyn Error>),
}
impl Display for ExpressionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UndefinedIdentifier(ident) => write!(f, "undefined identifier \"{}\"", ident),
            Self::FunctionCallFailure(err) => write!(f, "function call failure {:?}", err),
        }
    }
}
impl Error for ExpressionError {}

#[derive(Debug, Clone)]
pub struct InvalidFunction(pub Value);
impl Display for InvalidFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid function {:?}", self.0)
    }
}
impl Error for InvalidFunction {}

#[derive(Debug, Clone)]
pub struct InvalidArguments(pub Vec<Value>);
impl Display for InvalidArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid arguments {:?}", self.0)
    }
}
impl Error for InvalidArguments {}

fn plus(args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
    match args.as_slice() {
        [Value::Number(num1), Value::Number(num2)] => Ok(Value::Number(num1 + num2)),
        [Value::Function(func1), Value::Function(func2)] => {
            let func1 = func1.clone();
            let func2 = func2.clone();
            Ok(Value::Function(Rc::new(move |args| {
                plus(vec![func1(args.clone())?, func2(args)?])
            }))) 
        },
        [value, Value::Function(func)] => {
            let value = value.clone();
            let func = func.clone();
            Ok(Value::Function(Rc::new(move |args| {
                plus(vec![value.clone(), func(args)?])
            }))) 
        },
        [Value::Function(func), value] => {
            let func = func.clone();
            let value = value.clone();
            Ok(Value::Function(Rc::new(move |args| {
                plus(vec![func.clone()(args)?, value.clone()])
            }))) 
        },
        [Value::Number(num)] => Ok(Value::Number(*num)),
        [Value::Function(func)] => {
            let func = func.clone();
            Ok(Value::Function(Rc::new(move |args| {
                plus(vec![func(args)?])
            }))) 
        },
        _ => Err(Box::new(InvalidArguments(args.clone()))),
    }
}
fn minus(args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
    match args.as_slice() {
        [Value::Number(num1), Value::Number(num2)] => Ok(Value::Number(num1 - num2)),
        [Value::Function(func1), Value::Function(func2)] => {
            let func1 = func1.clone();
            let func2 = func2.clone();
            Ok(Value::Function(Rc::new(move |args| {
                minus(vec![func1(args.clone())?, func2(args)?])
            }))) 
        },
        [value, Value::Function(func)] => {
            let value = value.clone();
            let func = func.clone();
            Ok(Value::Function(Rc::new(move |args| {
                minus(vec![value.clone(), func(args)?])
            }))) 
        },
        [Value::Function(func), value] => {
            let func = func.clone();
            let value = value.clone();
            Ok(Value::Function(Rc::new(move |args| {
                minus(vec![func.clone()(args)?, value.clone()])
            }))) 
        },
        [Value::Number(num)] => Ok(Value::Number(-num)),
        [Value::Function(func)] => {
            let func = func.clone();
            Ok(Value::Function(Rc::new(move |args| {
                minus(vec![func(args)?])
            }))) 
        },
        _ => Err(Box::new(InvalidArguments(args.clone()))),
    }
}
fn star(args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
    match args.as_slice() {
        [Value::Number(num1), Value::Number(num2)] => Ok(Value::Number(num1 * num2)),
        [Value::Function(func1), Value::Function(func2)] => {
            let func1 = func1.clone();
            let func2 = func2.clone();
            Ok(Value::Function(Rc::new(move |args| {
                star(vec![func1(args.clone())?, func2(args)?])
            }))) 
        },
        [value, Value::Function(func)] => {
            let value = value.clone();
            let func = func.clone();
            Ok(Value::Function(Rc::new(move |args| {
                star(vec![value.clone(), func(args)?])
            }))) 
        },
        [Value::Function(func), value] => {
            let func = func.clone();
            let value = value.clone();
            Ok(Value::Function(Rc::new(move |args| {
                star(vec![func.clone()(args)?, value.clone()])
            }))) 
        },
        _ => Err(Box::new(InvalidArguments(args.clone()))),
    }
}
fn slash(args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
    match args.as_slice() {
        [Value::Number(num1), Value::Number(num2)] => Ok(Value::Number(num1 / num2)),
        [Value::Function(func1), Value::Function(func2)] => {
            let func1 = func1.clone();
            let func2 = func2.clone();
            Ok(Value::Function(Rc::new(move |args| {
                slash(vec![func1(args.clone())?, func2(args)?])
            }))) 
        },
        [value, Value::Function(func)] => {
            let value = value.clone();
            let func = func.clone();
            Ok(Value::Function(Rc::new(move |args| {
                slash(vec![value.clone(), func(args)?])
            }))) 
        },
        [Value::Function(func), value] => {
            let func = func.clone();
            let value = value.clone();
            Ok(Value::Function(Rc::new(move |args| {
                slash(vec![func.clone()(args)?, value.clone()])
            }))) 
        },
        _ => Err(Box::new(InvalidArguments(args.clone()))),
    }
}
fn percent(args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
    match args.as_slice() {
        [Value::Number(num1), Value::Number(num2)] => Ok(Value::Number(num1 % num2)),
        [Value::Function(func1), Value::Function(func2)] => {
            let func1 = func1.clone();
            let func2 = func2.clone();
            Ok(Value::Function(Rc::new(move |args| {
                percent(vec![func1(args.clone())?, func2(args)?])
            }))) 
        },
        [value, Value::Function(func)] => {
            let value = value.clone();
            let func = func.clone();
            Ok(Value::Function(Rc::new(move |args| {
                percent(vec![value.clone(), func(args)?])
            }))) 
        },
        [Value::Function(func), value] => {
            let func = func.clone();
            let value = value.clone();
            Ok(Value::Function(Rc::new(move |args| {
                percent(vec![func.clone()(args)?, value.clone()])
            }))) 
        },
        _ => Err(Box::new(InvalidArguments(args.clone()))),
    }
}
fn caret(args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
    match args.as_slice() {
        [Value::Number(num1), Value::Number(num2)] => Ok(Value::Number(num1.powf(*num2))),
        [Value::Function(func1), Value::Function(func2)] => {
            let func1 = func1.clone();
            let func2 = func2.clone();
            Ok(Value::Function(Rc::new(move |args| {
                caret(vec![func1(args.clone())?, func2(args)?])
            }))) 
        },
        [value, Value::Function(func)] => {
            let value = value.clone();
            let func = func.clone();
            Ok(Value::Function(Rc::new(move |args| {
                caret(vec![value.clone(), func(args)?])
            }))) 
        },
        [Value::Function(func), value] => {
            let func = func.clone();
            let value = value.clone();
            Ok(Value::Function(Rc::new(move |args| {
                caret(vec![func.clone()(args)?, value.clone()])
            }))) 
        },
        _ => Err(Box::new(InvalidArguments(args.clone()))),
    }
}

#[derive(Clone)]
pub struct Expression {
    tree: ParseTree,
    pub table: HashMap<String, Value>,
}
impl Expression {
    fn eval_tree(&self, tree: &ParseTree) -> Result<Value, ExpressionError> {
        match tree {
            ParseTree::Number(num) => Ok(Value::Number(*num)),
            ParseTree::Identifier(ident) => self.table.get(ident).map_or(Err(ExpressionError::UndefinedIdentifier(ident.clone())), |args| Ok(args.clone())),
            ParseTree::FunctionCall(func, args) => {
                let func = self.eval_tree(func)?;
                let args = args.iter().map(|args| self.eval_tree(args)).collect::<Result<Vec<_>,_>>()?;
                if let Value::Function(func) = func {
                    func(args).map_err(|err| ExpressionError::FunctionCallFailure(err))
                } else {
                    Err(ExpressionError::FunctionCallFailure(Box::new(InvalidFunction(func))))
                }
            },
        }
    }
    pub fn eval(&self) -> Result<Value, ExpressionError> {
        self.eval_tree(&self.tree)
    }
}
impl<'a> TryFrom<&'a str> for Expression {
    type Error = ParseError<'a>;

    fn try_from(string: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            tree: Parser::from(string).parse()?,
            table: HashMap::from([
                ("+".to_owned(), Value::Function(Rc::new(plus))),
                ("-".to_owned(), Value::Function(Rc::new(minus))),
                ("*".to_owned(), Value::Function(Rc::new(star))),
                ("/".to_owned(), Value::Function(Rc::new(slash))),
                ("%".to_owned(), Value::Function(Rc::new(percent))),
                ("^".to_owned(), Value::Function(Rc::new(caret))),
            ]),
        })
    }
}
