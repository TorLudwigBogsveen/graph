use std::{collections::HashMap, fmt::Debug, ops::Neg, vec::*};

pub type FunctionID = usize;
pub type VariableID = usize;

pub trait Function<T>: Debug {
    fn eval(&self, eval: &Evaluator<T>, vars: &mut[T], args: &[Instruction<T>]) -> Option<T>;
}

impl<T: Clone + Copy> Function<T> for fn(T) -> T {
    fn eval(&self, eval: &Evaluator<T>, vars: &mut [T], args: &[Instruction<T>]) -> Option<T> {
        if args.len() != 1 {
            return None;
        }

        let arg = eval.eval_inner(vars, &args[0]);

        Some((self)(arg?))
    }
}

impl<T: Clone + Copy> Function<T> for fn(T, T) -> T {
    fn eval(&self, eval: &Evaluator<T>, vars: &mut [T], args: &[Instruction<T>]) -> Option<T> {
        if args.len() != 2 {
            return None;
        }

        let arg0 = eval.eval_inner(vars, &args[0]);
        let arg1 = eval.eval_inner(vars, &args[1]);

        Some((self)(arg0?, arg1?))
    }
}

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
    functions: Vec<Box<dyn Function<T>>>,
    var_mapping: HashMap<String, VariableID>,
}

impl<T: Clone + Copy> Evaluator<T> {
    pub fn new(instructions: Instruction<T>, functions: Vec<Box<dyn Function<T>>>, var_mapping: HashMap<String, VariableID>) -> Evaluator<T> {
        Self {
            instructions,
            functions,
            var_mapping
        }
    }

    pub fn eval(&self, vars: &[(&str, T)]) -> Option<T> {
        let mut vars = vars
            .into_iter()
            .filter_map(|(k, v)| self.var_mapping
                .get(*k)
                .map(|id| (*id, *v))
            )
            .collect::<Vec<(usize, T)>>();
        vars.sort_by(|(k1, _),(k2,_)| k1.cmp(k2));
        let mut vars = vars.into_iter().map(|(_, v)| v).collect::<Vec<T>>();

        self.eval_inner(&mut vars, &self.instructions)
    }

    fn eval_inner(&self, vars: &mut [T], instruction: &Instruction<T>) -> Option<T> {
        match instruction {
            Instruction::Number(v) => Some(*v),
            Instruction::Variable(variable_id) => vars.get(*variable_id).map(|arg| *arg),
            Instruction::Call(function_id, function_args) => {
                self.functions.get(*function_id).map(|f| f.eval(self, vars, &function_args)).flatten()
            },
        }
    }
}

pub trait Functions {
    fn get(name: &str) -> Option<Box<dyn Function<Self>>>;
}

impl Functions for f32 {
    fn get(name: &str) -> Option<Box<dyn Function<Self>>> {
        let n = |_: f32, _: f32| 0.0f32;

        let f = match name {
            "add" => |a, b| a + b,
            "sub" => |a, b| a - b,
            "mul" => |a, b| a * b,
            "div" => |a, b| a / b,
            "pow" => |a, b| f32::powf(a, b),
            "log" => |a, b| f32::log(a,b),
            "deriv" => return Some(Box::new(Derivative)),
            "int" => return Some(Box::new(Integral)),
            _ => n
        };

        if f != n {
            return Some(Box::new(f))
        }

        let f = match name {
            "ln" => |a: f32| a.ln(),
            "abs" => |a: f32| a.abs(),
            "neg" => |a: f32| a.neg(),
            "exp" => |a: f32| a.exp(),

            "ceil" => |a: f32| a.ceil(),
            "floor" => |a: f32| a.floor(),
            "round" => |a: f32| a.round(),

            "cos" => |a: f32| a.cos(),
            "sin" => |a: f32| a.sin(),
            "tan" => |a: f32| a.tan(),
            "acos" => |a: f32| a.acos(),
            "asin" => |a: f32| a.asin(),
            "atan" => |a: f32| a.atan(),
            "cosh" => |a: f32| a.cosh(),
            "sinh" => |a: f32| a.sinh(),
            "tanh" => |a: f32| a.tanh(),
            _ => None?
        };

        Some(Box::new(f))
            
            //"deriv" => |args| args.last_chunk::<1>().map(|[f]| f.),
    }
}

impl Functions for f64 {
    fn get(name: &str) -> Option<Box<dyn Function<Self>>> {
        let n = |_: f64, _: f64| 0.0f64;

        let f = match name {
            "add" => |a, b| a + b,
            "sub" => |a, b| a - b,
            "mul" => |a, b| a * b,
            "div" => |a, b| a / b,
            "pow" => |a, b| f64::powf(a, b),
            "log" => |a, b| f64::log(a,b),
            "deriv" => return Some(Box::new(Derivative)),
            "int" => return Some(Box::new(Integral)),
            _ => n
        };

        if f != n {
            return Some(Box::new(f))
        }

        let f = match name {
            "ln" => |a: f64| a.ln(),
            "abs" => |a: f64| a.abs(),
            "neg" => |a: f64| a.neg(),
            "exp" => |a: f64| a.exp(),

            "ceil" => |a: f64| a.ceil(),
            "floor" => |a: f64| a.floor(),
            "round" => |a: f64| a.round(),

            "cos" => |a: f64| a.cos(),
            "sin" => |a: f64| a.sin(),
            "tan" => |a: f64| a.tan(),
            "acos" => |a: f64| a.acos(),
            "asin" => |a: f64| a.asin(),
            "atan" => |a: f64| a.atan(),
            "cosh" => |a: f64| a.cosh(),
            "sinh" => |a: f64| a.sinh(),
            "tanh" => |a: f64| a.tanh(),
            _ => None?
        };

        Some(Box::new(f))
            
    }
}

impl Functions for i64 {
    fn get(name: &str) -> Option<Box<dyn Function<Self>>> {
        let n = |_: i64, _: i64| 0;

        let f: fn(i64, i64) -> i64 = match name {
            "add" => |a, b| a + b,
            "sub" => |a, b| a - b,
            "mul" => |a, b| a * b,
            "div" => |a, b| a / b,
            "pow" => |a, b| i64::pow(a, b as u32),
            "log" => |a, b| i64::ilog(a,b) as i64,
            _ => n
        };

        if f != n {
            return Some(Box::new(f))
        }

        let f: fn(i64) -> i64 = match name {
            "abs" => |a| a.abs(),
            "neg" => |a| a.neg(),
            _ => None?
        };

        Some(Box::new(f))
    }
}

impl Functions for i32 {
    fn get(name: &str) -> Option<Box<dyn Function<Self>>> {
        let n = |_: i32, _: i32| 0;

        let f: fn(i32, i32) -> i32 = match name {
            "add" => |a, b| a + b,
            "sub" => |a, b| a - b,
            "mul" => |a, b| a * b,
            "div" => |a, b| a / b,
            "pow" => |a, b| i32::pow(a, b as u32),
            "log" => |a, b| i32::ilog(a,b) as i32,
            _ => n
        };

        if f != n {
            return Some(Box::new(f))
        }

        let f: fn(i32) -> i32 = match name {
            "abs" => |a| a.abs(),
            "neg" => |a| a.neg(),
            _ => None?
        };

        Some(Box::new(f))
    }
}

#[derive(Debug)]
struct Derivative;

impl Function<f32> for Derivative {
    fn eval(&self, eval: &Evaluator<f32>, vars: &mut[f32], args: &[Instruction<f32>]) -> Option<f32> {
        const H: f32 = 1e-6;

        if args.len() != 2 {
            return None;
        }

        let var = match args[1] {
            Instruction::Variable(var) => var,
            _ => return None
        };

        let d = vars[var];

        vars[var] = d + H;
        let a = eval.eval_inner(vars, &args[0])?;
        vars[var] = d - H;
        let b = eval.eval_inner(vars, &args[0])?;

        Some((a - b) / (H+H))
    }
}

impl Function<f64> for Derivative {
    fn eval(&self, eval: &Evaluator<f64>, vars: &mut[f64], args: &[Instruction<f64>]) -> Option<f64> {
        const H: f64 = 1e-8;

        if args.len() != 2 {
            return None;
        }

        let var = match args[1] {
            Instruction::Variable(var) => var,
            _ => return None
        };

        let d = vars[var];

        vars[var] = d + H;
        let a = eval.eval_inner(vars, &args[0])?;
        vars[var] = d - H;
        let b = eval.eval_inner(vars, &args[0])?;

        Some((a - b) / (H+H))
    }
}

#[derive(Debug)]
struct Integral;

impl Function<f64> for Integral {
    fn eval(&self, eval: &Evaluator<f64>, vars: &mut[f64], args: &[Instruction<f64>]) -> Option<f64> {
        // args: [expr, var_id, lower_bound, upper_bound, steps]
        if args.len() != 5 { return None; }

        let expr = &args[0];
        let var_idx = match &args[1] {
            Instruction::Variable(v) => *v,
            _ => return None,
        };

        let a = eval.eval_inner(vars, &args[2])?;
        let b = eval.eval_inner(vars, &args[3])?;
        let steps = eval.eval_inner(vars, &args[4])? as usize;

        if steps == 0 { return None; }

        let dx = (b - a) / steps as f64;
        let mut sum = 0.0;

        for i in 0..=steps {
            let x = a + i as f64 * dx;
            vars[var_idx] = x;

            let fx = eval.eval_inner(vars, expr)?;

            if i == 0 || i == steps {
                sum += fx / 2.0; // trapezoidal endpoints
            } else {
                sum += fx;
            }
        }

        vars[var_idx] = a; // restore variable
        Some(sum * dx)
    }
}

impl Function<f32> for Integral {
    fn eval(&self, eval: &Evaluator<f32>, vars: &mut[f32], args: &[Instruction<f32>]) -> Option<f32> {
        // args: [expr, var_id, lower_bound, upper_bound, steps]
        if args.len() != 5 { return None; }

        let expr = &args[0];
        let var_idx = match &args[1] {
            Instruction::Variable(v) => *v,
            _ => return None,
        };

        let a = eval.eval_inner(vars, &args[2])?;
        let b = eval.eval_inner(vars, &args[3])?;
        let steps = eval.eval_inner(vars, &args[4])? as usize;

        if steps == 0 { return None; }

        let dx = (b - a) / steps as f32;
        let mut sum = 0.0;

        for i in 0..=steps {
            let x = a + i as f32 * dx;
            vars[var_idx] = x;

            let fx = eval.eval_inner(vars, expr)?;

            if i == 0 || i == steps {
                sum += fx / 2.0; // trapezoidal endpoints
            } else {
                sum += fx;
            }
        }

        vars[var_idx] = a; // restore variable
        Some(sum * dx)
    }
}