use std::str::FromStr;

use godot::{classes::{ILine2D, IMeshInstance3D, Line2D, MeshInstance3D, SurfaceTool, mesh::PrimitiveType}, prelude::*};

mod squares;
mod cubes;
use math::{calc::*, parser::Parser};

use crate::squares::marching_squares;

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

#[derive(GodotClass)]
#[class(base=MeshInstance3D)]
struct ImplicitSurfaceGrapher {
    base: Base<MeshInstance3D>,
    #[export]
    inner_position: Vector3,
    #[export]
    inner_size: Vector3,
    #[export]
    size: Vector3,
    #[export]
    resolution: u32,
    #[export]
    lhs_expr: GString,
    #[export]
    rhs_expr: GString,
}

#[godot_api]
impl ImplicitSurfaceGrapher {
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
                &|x, y, z| e1.eval(&mut[("x", x as E), ("y", y as E), ("z", z as E)]).unwrap_or(Default::default()) as f64,
                &|x, y, z| e2.eval(&mut[("x", x as E), ("y", y as E), ("z", z as E)]).unwrap_or(Default::default()) as f64
            );
        } else {
            godot_error!("Error parsing expr!");
        }
    }
}

impl ImplicitSurfaceGrapher {
    fn graph_implicit(&mut self, f: &dyn Fn(f64, f64, f64) -> f64, g: &dyn Fn(f64, f64, f64) -> f64) {
        let resolution = self.resolution;
        let inner_position = self.inner_position;
        let inner_size = self.inner_size;
        let size = self.size;

        let mut grid = vec![0.; (resolution*resolution*resolution).try_into().unwrap()];

        let to_math = |x, y, z| {(
            inner_position.x as f64 + (x as f64) * (inner_size.x as f64 / resolution as f64),
            inner_position.y as f64 + (y as f64) * (inner_size.y as f64 / resolution as f64),
            inner_position.z as f64 + (z as f64) * (inner_size.z as f64 / resolution as f64),
        )};

        let from_math = |x, y, z| {(
            ((x as f32 + 0.0) / 2.0) * size.x,
            ((y as f32 + 0.0) / 2.0) * size.y,
            ((z as f32 + 0.0) / 2.0) * size.z,
        )};

        for i in 0..resolution {
            for j in 0..resolution {
                for l in 0..resolution {
                    let (x, y, z) = to_math(i, j, l);
                    //godot_print!("{} - {} = {}", f(x, y, z), g(x, y, z), f(x, y, z) - g(x, y, z));
                    grid[(l+j*resolution+i*resolution*resolution) as usize] = f(x, y, z) - g(x, y, z);
                }
            }
        }
        let triangles = cubes::marching_cubes(resolution as usize, resolution as usize, resolution as usize, grid);

        let mut surface_tool = SurfaceTool::new_gd();
        surface_tool.begin(PrimitiveType::TRIANGLES);
        surface_tool.set_color(Color::RED);
        
        for [(z1, y1, x1), (z2, y2, x2), (z3, y3, x3)] in triangles {
            let (x1, y1, z1) = from_math(x1, y1, z1);
            let (x2, y2, z2) = from_math(x2, y2, z2);
            let (x3, y3, z3) = from_math(x3, y3, z3);
            surface_tool.set_uv(Vector2 { x: 0., y: 0. });
            surface_tool.add_vertex(Vector3 {x: x1, y: y1, z: z1});
            surface_tool.set_uv(Vector2 { x: 1., y: 0. });
            surface_tool.add_vertex(Vector3 {x: x3, y: y3, z: z3});
            surface_tool.set_uv(Vector2 { x: 1., y: 1. });
            surface_tool.add_vertex(Vector3 {x: x2, y: y2, z: z2});
        }

        surface_tool.generate_normals();

        match surface_tool.commit() {
            Some(mesh) => {
                self.base_mut().set_mesh(&mesh);
            },
            _ => {
                godot_error!("Failed to create mesh!");
            }
        }

    }
}

#[godot_api]
impl IMeshInstance3D for ImplicitSurfaceGrapher {
     fn init(base: Base<MeshInstance3D>) -> Self {
        let grapher = Self {
            inner_position: Vector3 { x: 0., y: 0., z: 0. },
            inner_size: Vector3 { x: 1., y: 1., z: 1. },
            size: Vector3 { x: 400., y: 400., z: 400. },
            resolution: 64,
            lhs_expr: GString::from_str("0").unwrap(),
            rhs_expr: GString::from_str("0").unwrap(),
            base,
        };
        
        grapher
    }
}