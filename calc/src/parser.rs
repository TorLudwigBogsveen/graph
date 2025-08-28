use std::{collections::{hash_map::Entry, HashMap}, fmt::Debug, iter::Peekable, marker::PhantomData, str::Chars};

use crate::calc::{Evaluator, Function, Functions, Instruction};

struct VariableMap<'a> {
    map: HashMap<&'a str, usize>,
    n: usize
}

impl<'a> VariableMap<'a> {
    fn new() -> Self {
        Self { map: HashMap::new(), n: 0 }
    }

    fn get(&mut self, var: &'a str) -> usize {
        match self.map.entry(var) {
            Entry::Occupied(e) => {
                *e.get()
            },
            Entry::Vacant(e) => {
                e.insert(self.n);
                self.n += 1;
                return self.n-1;
            }
        }
    }
}

struct FunctionMap<'a, T> {
    var: VariableMap<'a>,
    funcs: Vec<Box<dyn Function<T>>>
}

impl<'a, T: Functions> FunctionMap<'a, T> {
    fn new() -> Self {
        Self { var: VariableMap::new(), funcs: Vec::new() }
    }

    fn get(&mut self, var: &'a str) -> Option<usize> {
        let has = self.var.map.contains_key(var);
        let i = self.var.get(var);

        if !has {
            let f = T::get(var)?;
            self.funcs.push(f);
        }

        Some(i)
    }
}

pub struct Parser<'a, T> {
    expr: &'a str,
    function_mapping: FunctionMap<'a, T>,
    variable_mapping: VariableMap<'a>,
    phantomdata: PhantomData<T>
}

impl<'a, T: NumberParser + Copy + Debug + Functions> Parser<'a, T> {
    pub fn new(expr: &'a str) -> Self {
        Self { expr, function_mapping: FunctionMap::<'a, T>::new(), variable_mapping: VariableMap::new(), phantomdata: PhantomData }
    }

    pub fn parse(mut self) -> Option<Evaluator<T>> {
        let mut t = Tokenizer::new(self.expr);

        let i = self.parse_expr(&mut t)?;
        let map = self.variable_mapping.map.iter().map(|(k, v)| (k.to_string(), *v)).collect::<HashMap<_,_>>();
        
        Some(Evaluator::new(i, self.function_mapping.funcs, map))
    }

    fn parse_call(&mut self, t: &mut Tokenizer<'a>, var: &'a str) -> Option<Instruction<T>> {
        let mut args = Vec::new();

        match t.next::<T>()? {
            Token::LeftParens => {}
            _ => panic!()
        }

        loop {
            let n = t.peek_char()?;
            let arg = match n {
                ')' => {
                    t.next_char();
                    break;
                },
                ',' => {
                    t.next_char();
                    continue;
                },
                _ => self.parse_expr(t)
            };
            args.push(arg?);
        }

        Some(Instruction::Call(self.function_mapping.get(var)?, args))
    }

    fn parse_variable(&mut self, t: &mut Tokenizer<'a>, var: &'a str) -> Option<Instruction<T>> {
        match t.peek_char() {
            Some('(') => {
                self.parse_call(t, var)
            },
            _ => {
                Some(Instruction::Variable(self.variable_mapping.get(var)))
            }
        }        
    }

    fn parse_expr(&mut self, t: &mut Tokenizer<'a>) -> Option<Instruction<T>> {
        match t.next()? {
            Token::Variable(var) => self.parse_variable(t, var),
            Token::Number(num) => Some(Instruction::Number(num)),
            _ => None
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Token<'a, T> {
    Variable(&'a str),
    Number(T),
    LeftParens,
    RightParens,
    Comma,
}

pub struct Tokenizer<'a> {
    expr: &'a str,
    iter: Peekable<Chars<'a>>,
    index: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(expr: &'a str) -> Tokenizer<'a> {
        Self { expr: expr, iter: expr.chars().peekable(), index: 0 }
    }

    pub fn peek_char(&mut self) -> Option<char> {
        self.iter.peek().map(|c| *c)
    }

    pub fn next_char(&mut self) -> Option<char> {
        self.index += 1;
        self.iter.next()
    }

    fn next_str(&mut self) -> Option<&'a str> {
        let start = self.index;
        let mut c = self.peek_char()?;
        
        while c.is_alphabetic() {
            self.next_char();
            c = if let Some(c) = self.peek_char() {
                c
            } else {
                break;
            };
        }

        Some(&self.expr[start..self.index])
    }

    fn next<T: NumberParser>(&mut self) -> Option<Token<'a, T>> {
        let n = self.peek_char()?;
        Some(if n == '(' {
            self.next_char();
            Token::LeftParens
        } else if n == ')' {
            self.next_char();
            Token::RightParens
        } else if n == ',' {
            self.next_char();
            Token::Comma
        } else if n.is_ascii_alphabetic() {
            Token::Variable(self.next_str()?)
        } else {
            Token::Number(T::parse(self)?)
        })
    }
}

pub trait NumberParser {
    fn parse<'a>(t: &mut Tokenizer<'a>) -> Option<Self> where Self: Sized;
}

impl NumberParser for f32 {
    fn parse<'a>(t: &mut Tokenizer<'a>) -> Option<Self> where Self: Sized {
        let start = t.index;
        let mut c = t.peek_char()?;
        
        while c.is_digit(10) || c == '.' {
            t.next_char();
            c = if let Some(c) = t.peek_char() {
                c
            } else {
                break;
            }
        }

        t.expr[start..t.index].parse().ok()
    }
}

impl NumberParser for f64 {
    fn parse<'a>(t: &mut Tokenizer<'a>) -> Option<Self> where Self: Sized {
        let start = t.index;
        let mut c = t.peek_char()?;
        
        while c.is_digit(10) || c == '.' {
            t.next_char();
            c = if let Some(c) = t.peek_char() {
                c
            } else {
                break;
            }
        }

        t.expr[start..t.index].parse().ok()
    }
}

impl NumberParser for i64 {
    fn parse<'a>(t: &mut Tokenizer<'a>) -> Option<Self> where Self: Sized {
        let start = t.index;
        let mut c = t.peek_char()?;
        
        while c.is_digit(10) {
            t.next_char();
            c = if let Some(c) = t.peek_char() {
                c
            } else {
                break;
            }
        }

        t.expr[start..t.index].parse().ok()
    }
}


impl NumberParser for i32 {
    fn parse<'a>(t: &mut Tokenizer<'a>) -> Option<Self> where Self: Sized {
        let start = t.index;
        let mut c = t.peek_char()?;
        
        while c.is_digit(10) {
            t.next_char();
            c = if let Some(c) = t.peek_char() {
                c
            } else {
                break;
            }
        }

        t.expr[start..t.index].parse().ok()
    }
}
