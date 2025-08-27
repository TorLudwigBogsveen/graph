use std::str::FromStr;

use godot::{classes::{ILine2D, Line2D}, prelude::*};

mod calc;
mod parser;
mod squares;
mod cubes;
use crate::{calc::*, parser::Parser, squares::marching_squares};

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    fn on_level_init(_level: InitLevel) {
        godot_print!("Hello rust!\n")
    }
}

#[derive(GodotClass)]
#[class(base=Line2D)]
struct LineGrapher {
    base: Base<Line2D>,
    #[export]
    inner_position: Vector2,
    #[export]
    inner_size: Vector2,
    #[export]
    size: Vector2,
    #[export]
    resolution: u32,
    #[export]
    expr: GString,
}

#[godot_api]
impl LineGrapher {
    #[func]
    fn eval(&mut self) {
        type E = f32;
        let expr = self.expr.to_string();
        let expr: String = expr.chars().filter(|ch: &char| !ch.is_whitespace()).map(|ch| ch.to_ascii_lowercase()).collect();
        let evaluator: Option<Evaluator<E>> = Parser::new(&expr).parse();
        if let Some(e) = evaluator {
            self.graph_x(&|x| e.eval(&mut[("x", x as E)]).unwrap_or(Default::default()) as f64);
        } else {
            godot_error!("Error parsing expr!");
        }
    }
}

impl LineGrapher {
    fn graph_x(&mut self, f: &dyn Fn(f64) -> f64) {
        let resolution = self.resolution;
        let inner_position = self.inner_position;
        let inner_size = self.inner_size;
        let size = self.size;
        let mut base = self.base_mut();
        for i in 0..=resolution {
            let x = inner_position.x as f64 + i as f64 * (inner_size.x as f64 / resolution as f64);
            let y = f(x) as f32;
            let x = x as f32;
            base.add_point(Vector2 { x: (x / inner_size.x) * size.x, y: (y / inner_size.y) * size.y });
        }
    }
}

#[godot_api]
impl ILine2D for LineGrapher {
     fn init(base: Base<Line2D>) -> Self {
        let grapher = Self {
            inner_position: Vector2 { x: 0., y: 0. },
            inner_size: Vector2 { x: 1., y: 1. },
            size: Vector2 { x: 600., y: 400. },
            resolution: 64,
            expr: GString::from_str("0").unwrap(),
            base,
        };

        
        grapher
    }
}

#[derive(GodotClass)]
#[class(base=Node2D)]
struct ImplicitLineGrapher {
    base: Base<Node2D>,
    #[export]
    inner_position: Vector2,
    #[export]
    inner_size: Vector2,
    #[export]
    size: Vector2,
    #[export]
    resolution: u32,
    #[export]
    lhs_expr: GString,
    #[export]
    rhs_expr: GString,
    #[var]
    lines: PackedVector2Array,
}

#[godot_api]
impl ImplicitLineGrapher {
    #[func]
    fn eval(&mut self) {
        type E = f32;
        let expr1 = self.lhs_expr.to_string();
        let expr1: String = expr1.chars().filter(|ch: &char| !ch.is_whitespace()).map(|ch| ch.to_ascii_lowercase()).collect();
        let e1: Option<Evaluator<E>> = Parser::new(&expr1).parse();
        let expr2 = self.rhs_expr.to_string();
        let expr2: String = expr2.chars().filter(|ch: &char| !ch.is_whitespace()).map(|ch| ch.to_ascii_lowercase()).collect();
        let e2: Option<Evaluator<E>> = Parser::new(&expr2).parse();
        if let (Some(e1), Some(e2)) = (e1, e2) {
            self.graph_implicit(
                &|x, y| e1.eval(&mut[("x", x as E), ("y", y as E)]).unwrap_or(Default::default()) as f64,
                &|x, y| e2.eval(&mut[("x", x as E), ("y", y as E)]).unwrap_or(Default::default()) as f64
            );
        } else {
            godot_error!("Error parsing expr!");
        }
    }
}

impl ImplicitLineGrapher {
    fn graph_implicit(&mut self, f: &dyn Fn(f64, f64) -> f64, g: &dyn Fn(f64, f64) -> f64) {
        let resolution = self.resolution;
        let inner_position = self.inner_position;
        let inner_size = self.inner_size;
        let size = self.size;

        let mut grid = vec![0.; (resolution*resolution).try_into().unwrap()];

        let to_math = |x, y| {(
            inner_position.x as f64 + (x as f64) * (inner_size.x as f64 / resolution as f64),
            inner_position.y as f64 + (y as f64) * (inner_size.y as f64 / resolution as f64)
        )};

        let from_math = |x, y| {(
            ((x as f32 + 0.0) / 2.0) * size.x,
            ((y as f32 + 0.0) / 2.0) * size.y,
        )};

        for i in 0..resolution {
            for j in 0..resolution {
                let (x, y) = to_math(i, j);
                grid[(j+i*resolution) as usize] = f(x, y) - g(x, y);
            }
        }
        let lines = marching_squares(resolution as usize, resolution as usize, grid);
        let mut n_lines = PackedVector2Array::new();
        for [(y, x), (y2, x2)] in lines {
            let (x, y) = from_math(x, y);
            n_lines.push(Vector2 { x, y });
            let (x, y) = from_math(x2, y2);
            n_lines.push(Vector2 { x, y });
        }

        self.set_lines(n_lines);
    }
}

#[godot_api]
impl INode2D for ImplicitLineGrapher {
     fn init(base: Base<Node2D>) -> Self {
        let grapher = Self {
            inner_position: Vector2 { x: 0., y: 0. },
            inner_size: Vector2 { x: 1., y: 1. },
            size: Vector2 { x: 600., y: 400. },
            resolution: 64,
            lhs_expr: GString::from_str("0").unwrap(),
            rhs_expr: GString::from_str("0").unwrap(),
            lines: PackedVector2Array::new(),
            base,
        };

        
        grapher
    }
}

#[cfg(test)]
mod test {
    use crate::parser::*;

    #[test]
    fn t() {
        println!("{:#?}", Parser::<f32>::new("6").parse());
        println!("{:#?}", Parser::<f32>::new("a").parse());
        println!("{:#?}", Parser::<f32>::new("add(6)").parse());
        println!("{:#?}", Parser::<f32>::new("add(6,4)").parse());
        println!("{:#?}", Parser::<f32>::new("add(6,sub(1,4))").parse());
        println!("{:#?}", Parser::<f32>::new("add(mul(4,2),5)").parse());
        println!("{:#?}", Parser::<f32>::new("add(add(1,2),add(3,4))").parse());
    }

    #[test]
    fn t2() {
        let eval = |expr, args| Parser::<f32>::new(expr).parse().unwrap().eval(args).unwrap();
        assert_eq!(eval("6", &[]), 6.);
        assert_eq!(eval("add(6,4)", &[]), 10.);
        assert_eq!(eval("add(6,sub(1,4))", &[]), 3.);
        assert_eq!(eval("add(mul(4,2),5)", &[]), 13.);
        assert_eq!(eval("add(add(1,2),add(3,4))", &[]), 10.);
    }

    #[test]
    fn t3() {
        let eval = |expr, args| Parser::<f64>::new(expr).parse().unwrap().eval(args).unwrap();
        println!("{}", eval("deriv(mul(x,x),x)", &[("x", 5.)]));
    }

    #[test]
    fn t4() {
        let eval = |expr, args| Parser::<f64>::new(expr).parse().unwrap().eval(args).unwrap();
        println!("{}", eval("int(sin(x),x,0,1,1000)", &[("x", 5.)]));
    }

    #[test]
    fn t5() {
        let eval = |expr, args| Parser::<f64>::new(expr).parse().unwrap().eval(args).unwrap();
        println!("{}", eval("mul(x,x)", &[("x", 0.75), ("y", 0.5)]));
        println!("{}", eval("y", &[("x", 0.75), ("y", 0.5)]));
        println!("{}", eval("mul(x,x)", &[("y", 0.5), ("x", 0.75)]));
        println!("{}", eval("y", &[("y", 0.5), ("x", 0.75)]));
    }
}