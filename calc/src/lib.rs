use std::str::FromStr;

use godot::{classes::{ILine2D, Line2D}, prelude::*};

mod calc;
mod parser;
use crate::{calc::*, parser::Parser};

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
        type E = i32;
        let evaluator: Option<Evaluator<E>> = Parser::new(&self.expr.to_string()).parse();
        if let Some(e) = evaluator {
            self.graph_x(&|x| e.eval(vec![x as E]).unwrap_or(Default::default()) as f64);
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
            godot_print!("{}: {}", x, y);
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
        assert_eq!(eval("6", vec![]), 6.);
        assert_eq!(eval("add(6,4)", vec![]), 10.);
        assert_eq!(eval("add(6,sub(1,4))", vec![]), 3.);
        assert_eq!(eval("add(mul(4,2),5)", vec![]), 13.);
        assert_eq!(eval("add(add(1,2),add(3,4))", vec![]), 10.);
    }
}