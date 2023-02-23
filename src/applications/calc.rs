use core::fmt;
use alloc::{boxed::Box, string::String, vec::Vec};
use alloc::string::ToString;
use alloc::borrow::ToOwned;
use crate::{println, print,  mknode, std};


use async_trait::async_trait;
use crate::std::application::{
	Application,
	Error as ShellError
};

struct Parser {
    tokens: Vec<Token>,
    idx: i32,
    current: Token
}


struct Interpreter {}



impl Interpreter {

    fn new() -> Result<Self, Error>{
        return Ok(Self {})
    }

    fn visit(&mut self, node: Node) -> Result<Value, Error> {
        match node {
            Node::BinaryOperation(_) => return self.visit_binary_operation(node),
            Node::UnaryOperation(_) => return self.visit_unary_operation(node),
            Node::Number(_) => return self.visit_number(node),
            Node::Operator(_) => return self.visit_operator(node),
        }
    }


    fn visit_number(&mut self, node: Node) -> Result<Value, Error> {

        if let Node::Number(x) = node {
            Ok(Value::Number(x))
        } else {
            Err(Error::Other(String::from("value accessed was not an number")))
        }
    }


    fn visit_operator(&mut self, node: Node) -> Result<Value, Error> {

        if let Node::Operator(x) = node {
            Ok(Value::Operator(x))
        } else {
            Err(Error::Other(String::from("value is not an operator")))
        }

    }


    fn visit_binary_operation(&mut self, node: Node) -> Result<Value, Error>  {

        let left = match self.visit(self.get_node(node.clone(), "left")?.expect("returned none").to_owned())? {
            Value::Number(x) => x,
            _ => return Err(Error::Other(String::from("value is not a number"))),
        };

        let right = match self.visit(self.get_node(node.clone(), "right")?.expect("returned none").to_owned())? {
            Value::Number(x) => x,
            _ => return Err(Error::Other(String::from("value is not a number"))),
        };

        let operator = match self.visit(self.get_node(node.clone(), "operator")?.expect("returned none").to_owned())? {
            Value::Operator(x) => x,
            _ => return Err(Error::Other(String::from("value is not a binary operator"))),
        };

        match operator {
            Operator::Add => {
                return Ok(Value::Number(left + right))
            },
            Operator::Sub => {
                return Ok(Value::Number(left - right))
            },
            Operator::Div => {
                if right != 0.0 {
                    return Ok(Value::Number(left / right))
                } else {
                    return Err(Error::LogicalError(String::from("division by 0")))
                }
            },
            Operator::Mod => {
                return Ok(Value::Number(left % right))
            },
            Operator::Qot => {
                if right != 0.0 {
                    return Ok(Value::Number(
                        ((left / right) as i64) as f64
                    ))
                } else {
                    return Err(Error::LogicalError(String::from("division by 0")))
                }
            },
            Operator::Mul => {
                return Ok(Value::Number(left * right))
            },
            Operator::Exp => {
                return Ok(Value::Number({
                	let mut val = 1.0;
                	for _ in 0..(right as i64) {
                		val *= left
                	};
                	val
                }))
            }
        }

    }


    fn visit_unary_operation(&mut self, node: Node) -> Result<Value, Error>  {

        let other: f64 = match self.visit(self.get_node(node.clone(), "other")?.expect("returned none").to_owned())? {
            Value::Number(x) => x,
            _ => return Err(Error::LogicalError("value is not a number".to_string()))
        };

        if let Node::UnaryOperation(x) = node {
            match x.operator {
                Node::Operator(Operator::Sub) => {
                    return Ok(Value::Number(other * -1f64));
                },
                _ => return Err(Error::LogicalError("value is not an operator".to_string()))
            }
        } else {
            return Err(Error::LogicalError("node is not a binary operator".to_string()))
        }
    }

    fn get_node(&self, node: Node, position: &str) -> Result<Option<Node>, Error> {
        let result = match position {
            "left" => {
                if let Node::BinaryOperation(x) = node {
                    Some(x.left)
                } else {
                    None
                }
            }
            "right" => {
                if let Node::BinaryOperation(x) = node {
                    Some(x.right)
                } else {
                    None
                }
            }
            "other" => {
                if let Node::UnaryOperation(x) = node {
                    Some(x.other)
                } else {
                    None
                }
            }
            "operator" => {
                if let Node::BinaryOperation(x) = node {
                    Some(x.operator)
                } else {
                    None
                }
            },
            _ => return Err(Error::Other(String::from("invalid param for get_Node")))
        };
        Ok(result)

    }

}











impl Parser {
    fn new(tokens: Vec<Token>) -> Result<Self, Error> {
        let mut parser = Self { tokens, idx: -1, current: Token::Null };
        parser.advance()?;
        Ok(parser)

    }

    fn parse(&mut self) -> Result<Node, Error> {
        let result = self.expr();
        result
    }



    fn advance(&mut self) -> Result<Option<Token>, Error> {
        self.idx += 1;
        if self.idx < self.tokens.len() as i32 {
            self.current = self.tokens[self.idx as usize].clone();
            Ok(Some(self.current.clone()))
        } else if self.idx == self.tokens.len() as i32 {
            Ok(None)
        } else {
            Err(Error::Eof)
        }
    }

    fn atom(&mut self) -> Result<Node, Error> {
        let token = self.current.clone();

        match token {
            Token::Number(x) => {
                self.advance()?;
                return Ok(Node::Number(x))
            },

            Token::Bracket('(') => {
                self.advance()?;
                let expr = self.expr()?;

                if let Token::Bracket(')') = self.current {
                    self.advance()?;
                    return Ok(expr)
                } else {
                    return Err(Error::InvalidSyntax(0))
                }
            },
            _ => return Err(Error::InvalidSyntax(0))
        }
    }

    fn power(&mut self) -> Result<Node, Error> {
        let mut left = self.atom()?;

        while let Token::Operator(Operator::Exp) = self.current {
            let current = self.current.clone();
            self.advance()?;
            let operator = mknode!(current).expect("mknode function returned None");
            let right = self.factor()?;

            left = Node::BinaryOperation(Box::new(BinaryOperation { left: left.clone(), operator, right }));
        }
        Ok(left)
    }



    fn factor(&mut self) -> Result<Node, Error> {

        let token = self.current.clone();

        match token {
            Token::Operator(Operator::Add) | Token::Operator(Operator::Sub) => {
                self.advance()?;
                let operator = mknode!(token).expect("mknode returned none");
                let other = self.factor().expect("holup");
                return Ok(Node::UnaryOperation(Box::new(UnaryOperation { operator, other})))
            },

            _ => {
                return Ok(self.power()?)
            }
        }
    }



    fn term(&mut self) -> Result<Node, Error> {

        let mut left = self.factor()?;

        while let Token::Operator(Operator::Div)
                | Token::Operator(Operator::Mul)
                | Token::Operator(Operator::Mod)
                | Token::Operator(Operator::Qot) = self.current {

            let current = self.current.clone();
            self.advance()?;
            let operator = mknode!(current).expect("mknode function returned None");
            let right = self.factor()?;

            left = Node::BinaryOperation(Box::new(BinaryOperation { left: left.clone(), operator, right }));
        }
        Ok(left)
    }



    fn expr(&mut self) -> Result<Node, Error> {
        let mut left = self.term()?;

        while let Token::Operator(Operator::Sub) | Token::Operator(Operator::Add) = self.current {
            let current = self.current.clone();
            self.advance()?;
            let operator = mknode!(current).expect("mknode returned None");
            let right = self.term()?;

            left = Node::BinaryOperation(Box::new(BinaryOperation { left: left.clone(), operator, right }));
        }
        Ok(left)
    }
}



#[macro_export]
macro_rules! mknode {
    ($token:expr) => {
        match $token {
            Token::Operator(x) => Some(Node::Operator(x)),
            Token::Number(x) => Some(Node::Number(x)),
            _ => None
        }
    };
}



pub struct Calculator {}

#[async_trait]
impl Application for Calculator {
	fn new() -> Self {
		Self {}
	}

	async fn run(&mut self, args: Vec<String>) -> Result<(), ShellError> {
		if args.len() == 0 {
			loop {
				print!("enter equation > ");
				let inp = std::io::stdin().await;
				println!("{}", inp);
				if inp == String::from("exit\n") {
					return Ok(());
				}
				match calculate_inner(inp) {
			        Ok(_) => (),
			        Err(_) => { println!("your input must be a valid mathematical expression contaning only numbers (including floats) and the operators: [ +, -, *, **, /, //, % ]"); return Err(ShellError::CommandFailed(String::from("failed"))) },
		    	};
			}
		} else {
		    match calculate_inner(args.into_iter().collect()) {
		        Ok(x) => x,
		        Err(_) => { println!("your input must be a valid mathematical expression contaning only numbers (including floats) and the operators: [ +, -, *, **, /, //, % ]"); return Err(ShellError::CommandFailed(String::from("failed"))) },
		    };
			Ok(())
		}

	}
}


fn calculate_inner(mut equation: String) -> Result<f64, Error> {

	equation.push('\n');
	let mut neweq = equation.clone();
	neweq.pop();


    let tokens = tokenise(&equation)?;

    let mut parser = Parser::new(tokens)?;
    let ast = parser.parse()?;


    let mut interpreter = Interpreter::new()?;
    let result = interpreter.visit(ast)?;

	let return_res = {
		if let Value::Number(x) = result {
			x
		} else {
			panic!("did not return a float!");
		}
	};
	println!("\n\n
   _____                _        _
  / ____|              | |      | |
 | |     _ __ _   _ ___| |_ __ _| |
 | |    | '__| | | / __| __/ _` | |
 | |____| |  | |_| \\__ \\ || (_| | |
  \\_____|_|   \\__, |___/\\__\\__,_|_|
        _____  __/ |
       / ____||___/ |
      | |     __ _| | ___
      | |    / _` | |/ __|
      | |___| (_| | | (__
       \\_____\\__,_|_|\\___|

    ┌────────────────────────────────────────────┐
    │                                            │
    │    Expression -> [ {} ]
    │                                            │
    │    Calculated Solution -> [ {} ]
    │                                            │
    └────────────────────────────────────────────┘
    ", neweq, return_res);

    Ok(return_res)
}



fn tokenise(equation: &str) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::new();
    let mut current_num = "".to_string();
    let current_string: String = "".to_string();

    'mainloop: for (x, character) in equation.chars().enumerate() {


        match character {
            '0'..='9' => current_num.push(character),
            '.' => current_num.push(character),
            _ => {
                if current_num.len() != 0 {
                    tokens.push(Token::Number(current_num.parse::<f64>().unwrap()));
                    current_num = "".to_string();
                } else if current_string.len() != 0 {

                } match character {
                    '+' => tokens.push(Token::Operator(Operator::Add)),
                    '-' => tokens.push(Token::Operator(Operator::Sub)),
                    '%' => tokens.push(Token::Operator(Operator::Mod)),
                    '*' => {
                        if &equation.chars().nth(x-1).unwrap() == &'*' {
                            tokens.push(Token::Operator(Operator::Exp));
                        } else if &equation.chars().nth(x+1).unwrap() == &'*' {
                            ()
                        } else {
                            tokens.push(Token::Operator(Operator::Mul));
                        }
                    },
                    '/' => {
                        if &equation.chars().nth(x-1).unwrap() == &'/' {
                            tokens.push(Token::Operator(Operator::Qot));
                        } else if &equation.chars().nth(x+1).unwrap() == &'/' {
                            ()
                        } else {
                            tokens.push(Token::Operator(Operator::Div));
                        }
                    },
                    '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' => tokens.push(Token::Bracket(character)),
                    '\n' => break 'mainloop,
                    ' ' => (),
                    _ => {
                        return Err(Error::InvalidCharacter(x))
                    },
                }
            }
        }
    }
    Ok(tokens)
}




#[derive(Debug)]
enum Value {
    Number(f64),
    Operator(Operator)
}



#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Operator(Operator),
    Bracket(char),
    Null,
}


#[derive(Copy, Debug, Clone, PartialEq)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Qot,
    Mod,
    Exp,
}

#[derive(Debug)]
enum Error {
    InvalidSyntax(usize),
    InvalidCharacter(usize),
    LogicalError(String),
    Eof,
    Other(String),
}



#[derive(Debug, Clone, PartialEq)]
enum Node {
    Number(f64),
    Operator(Operator),
    BinaryOperation(Box<BinaryOperation>),
    UnaryOperation(Box<UnaryOperation>)
}


#[derive(Debug, Clone, PartialEq)]
struct BinaryOperation {
    left: Node,
    operator: Node,
    right: Node,
}

#[derive(Debug, Clone, PartialEq)]
struct UnaryOperation {
    operator: Node,
    other: Node,
}







impl fmt::Display for BinaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "(\n{} \n{} \n{}\n)

            ", self.left, self.operator, self.right
        )
    }
}
impl fmt::Display for UnaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "(\n{} \n{} \n)

            ", self.operator, self.other,
        )
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Number(x) => write!(f, "{}", x),
            Node::Operator(x) => write!(f, "{}", x),
            Node::BinaryOperation(x) => {
                let inner = *x.clone();
                write!(f, "{}", inner)
            }
            Node::UnaryOperation(x) => {
                let inner = *x.clone();
                write!(f, "{}", inner)
            }
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Sub => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
            Operator::Mod => write!(f, "%"),
            Operator::Qot => write!(f, "//"),
            Operator::Exp => write!(f, "**"),
        }
    }
}
