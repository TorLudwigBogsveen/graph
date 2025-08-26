use std::{ops::Neg, vec::*};

pub type FunctionID = usize;
pub type VariableID = usize;

pub type Args<T> = Vec<T>;
pub type Func<T> = fn(&[T]) -> Option<T>;

#[derive(Debug)]
pub enum Instruction<T> {
    Number(T),
    Variable(VariableID),
    Call(FunctionID, Vec<Instruction<T>>),
}

impl<T: Default> Default for Instruction<T> {
    fn default() -> Self {
        Instruction::Number(T::default())
    }
}

#[derive(Default, Debug)]
pub struct Evaluator<T> {
    instructions: Instruction<T>,
    functions: Vec<Func<T>>
}

impl<T: Clone + Copy> Evaluator<T> {
    pub fn new(instructions: Instruction<T>, functions: Vec<Func<T>>) -> Evaluator<T> {
        Self {
            instructions,
            functions
        }
    }

    pub fn eval(&self, mut args: Args<T>) -> Option<T> {
        self.eval_inner(&mut args, &self.instructions)
    }

    fn eval_inner(&self, args: &mut Args<T>, instruction: &Instruction<T>) -> Option<T> {
        match instruction {
            Instruction::Number(v) => Some(*v),
            Instruction::Variable(variable_id) => args.get(*variable_id).map(|arg| *arg),
            Instruction::Call(function_id, function_args) => {
                let function_args: Option<Vec<T>> = function_args.into_iter().map(
                    |instruction| 
                        self.eval_inner(args, instruction)
                ).collect();
                let function_args = function_args?;
                self.functions.get(*function_id).map(|f| f(&function_args)).flatten()
            },
        }
    }
}

pub trait Functions {
    fn get(name: &str) -> Option<Func<Self>> where Self: Sized;
}

impl Functions for f32 {
    fn get(name: &str) -> Option<Func<Self>> {
        Some(match name {
            "add" => |args| Some(args.get(0)? + args.get(1)?),
            "sub" => |args| Some(args.get(0)? - args.get(1)?),
            "mul" => |args| Some(args.get(0)? * args.get(1)?),
            "div" => |args| Some(args.get(0)? / args.get(1)?),
            "pow" => |args| Some(args.get(0)?.powf(*args.get(1)?)),
            "log" => |args| Some(args.get(0)?.log(*args.get(1)?)),

            "ln" => |args| Some(args.get(0)?.ln()),
            "abs" => |args| Some(args.get(0)?.abs()),
            "neg" => |args| Some(args.get(0)?.neg()),
            "exp" => |args| Some(args.get(0)?.exp()),

            "ceil" => |args| Some(args.get(0)?.ceil()),
            "floor" => |args| Some(args.get(0)?.floor()),
            "round" => |args| Some(args.get(0)?.round()),

            "cos" => |args| Some(args.get(0)?.cos()),
            "sin" => |args| Some(args.get(0)?.sin()),
            "tan" => |args| Some(args.get(0)?.tan()),
            "acos" => |args| Some(args.get(0)?.acos()),
            "asin" => |args| Some(args.get(0)?.asin()),
            "atan" => |args| Some(args.get(0)?.atan()),
            "cosh" => |args| Some(args.get(0)?.cosh()),
            "sinh" => |args| Some(args.get(0)?.sinh()),
            "tanh" => |args| Some(args.get(0)?.tanh()),

            "deriv" => Some(args.get(0)?.tanh()),

            _ => None?
        })
    }
}

impl Functions for f64 {
    fn get(name: &str) -> Option<Func<Self>> {
        Some(match name {
            "add" => |args| Some(args.get(0)? + args.get(1)?),
            "sub" => |args| Some(args.get(0)? - args.get(1)?),
            "mul" => |args| Some(args.get(0)? * args.get(1)?),
            "div" => |args| Some(args.get(0)? / args.get(1)?),
            "pow" => |args| Some(args.get(0)?.powf(*args.get(1)?)),
            "log" => |args| Some(args.get(0)?.log(*args.get(1)?)),

            "ln" => |args| Some(args.get(0)?.ln()),
            "abs" => |args| Some(args.get(0)?.abs()),
            "neg" => |args| Some(args.get(0)?.neg()),
            "exp" => |args| Some(args.get(0)?.exp()),

            "ceil" => |args| Some(args.get(0)?.ceil()),
            "floor" => |args| Some(args.get(0)?.floor()),
            "round" => |args| Some(args.get(0)?.round()),

            "cos" => |args| Some(args.get(0)?.cos()),
            "sin" => |args| Some(args.get(0)?.sin()),
            "tan" => |args| Some(args.get(0)?.tan()),
            "acos" => |args| Some(args.get(0)?.acos()),
            "asin" => |args| Some(args.get(0)?.asin()),
            "atan" => |args| Some(args.get(0)?.atan()),
            "cosh" => |args| Some(args.get(0)?.cosh()),
            "sinh" => |args| Some(args.get(0)?.sinh()),
            "tanh" => |args| Some(args.get(0)?.tanh()),
            _ => None?
        })
    }
}

impl Functions for i64 {
    fn get(name: &str) -> Option<Func<Self>> {
        Some(match name {
            "add" => |args| Some(args.get(0)? + args.get(1)?),
            "sub" => |args| Some(args.get(0)? - args.get(1)?),
            "mul" => |args| Some(args.get(0)? * args.get(1)?),
            "div" => |args| Some(args.get(0)? / args.get(1)?),
            "pow" => |args| Some(args.get(0)?.pow(*args.get(1)? as u32)),
            "log" => |args| Some(args.get(0)?.ilog(*args.get(1)?).into()),

            "abs" => |args| Some(args.get(0)?.abs()),
            "neg" => |args| Some(args.get(0)?.neg()),

            _ => None?
        })
    }
}

impl Functions for i32 {
    fn get(name: &str) -> Option<Func<Self>> {
        Some(match name {
            "add" => |args| Some(args.get(0)? + args.get(1)?),
            "sub" => |args| Some(args.get(0)? - args.get(1)?),
            "mul" => |args| Some(args.get(0)? * args.get(1)?),
            "div" => |args| Some(args.get(0)? / args.get(1)?),
            "pow" => |args| Some(args.get(0)?.pow(*args.get(1)? as u32)),
            "log" => |args| Some(args.get(0)?.ilog(*args.get(1)?) as i32),

            "abs" => |args| Some(args.get(0)?.abs()),
            "neg" => |args| Some(args.get(0)?.neg()),

            _ => None?
        })
    }
}