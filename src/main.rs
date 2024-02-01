use clap::{Arg, App};
use mathsolver::equation::{Equation, CustomOperations, Node};
use plotters::coord::ranged3d::Cartesian3d;
use plotters::coord::types::RangedCoordf64;
use plotters::prelude::*;
use plotters_bitmap::BitMapBackend;
//use rayon::prelude::*;
use std::error::Error;
use std::path::Path;

mod marching_squares;
mod marching_cubes;
mod plot;

use marching_squares::marching_squares;
use marching_cubes::marching_cubes;
use plot::*;

pub type Chart<'a, 'b> = ChartContext<'a, BitMapBackend<'b>, Cartesian2d<RangedCoordf64, RangedCoordf64>>;
pub type Chart3D<'a, 'b> = ChartContext<'a, BitMapBackend<'b>, Cartesian3d<RangedCoordf64, RangedCoordf64, RangedCoordf64>>;

pub type Root<'a> = DrawingArea<BitMapBackend<'a>, plotters::coord::Shift>;
pub struct GraphSettings<'a> {
    path: &'a str,
    image_width: u32,
    image_height: u32,
    sim_window: (f64, f64, f64, f64)
}

fn create_root<'a>(settings: &GraphSettings<'a>) -> Result<Root<'a>, Box<dyn Error>> {
    let root = BitMapBackend::new(settings.path, (settings.image_width, settings.image_height)).into_drawing_area();
    root.fill(&WHITE)?;
    Ok(root)
}

fn create_graph<'a, 'b>(settings: &GraphSettings<'a>, root: &'a Root<'b>) -> Result<Chart<'a, 'b>, Box<dyn Error>> {
    let mut chart = ChartBuilder::on(root)
        .margin(10)
        .set_all_label_area_size(30)
        .build_cartesian_2d(settings.sim_window.0..settings.sim_window.1, settings.sim_window.2..settings.sim_window.3)?;

    chart
        .configure_mesh()
        .label_style(("sans-serif", 15).into_font().color(&BLACK))
        .axis_style(&BLACK)
        .draw()?;

    Ok(chart)
}

fn create_graph_3d<'a, 'b>(settings: &GraphSettings<'a>, root: &'a Root<'b>) -> Result<Chart3D<'a, 'b>, Box<dyn Error>> {
    let mut chart = ChartBuilder::on(root)
        .margin(10)
        .set_all_label_area_size(30)
        .build_cartesian_3d(settings.sim_window.0..settings.sim_window.1, settings.sim_window.2..settings.sim_window.3, settings.sim_window.0..settings.sim_window.1)?;

    chart.with_projection(|mut pb| {
        pb.yaw = 0.5;
        pb.scale = 0.9;
        pb.into_matrix()
    });

    chart
        .configure_axes()
        .label_style(("sans-serif", 15).into_font().color(&BLACK))
        .max_light_lines(3)
        .draw()?;

    Ok(chart)
}
struct SubEqual;

impl CustomOperations for SubEqual {
    fn equal_f64(lhs: f64, rhs: f64) -> Node {
        Node::Real(lhs - rhs)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Graph App")
        .version("1.0")
        .author("Ludwig Bogsveen")
        .arg(Arg::with_name("width")
            .short('w')
            .long("width")
            .value_name("WIDTH")
            .takes_value(true)
            .default_value("600")
            .help("Sets the image width"))
        .arg(Arg::with_name("height")
            .short('h')
            .long("height")
            .value_name("HEIGHT")
            .takes_value(true)
            .default_value("600")
            .help("Sets the image height"))
        .arg(Arg::with_name("xmin")
            .long("xmin")
            .short('x')
            .value_name("XMIN")
            .takes_value(true)
            .default_value("-1.0")
            .help("Sets the minimum X value of the simulation window"))
            .allow_hyphen_values(true)
        .arg(Arg::with_name("xmax")
            .long("xmax")
            .short('X')
            .value_name("XMAX")
            .takes_value(true)
            .default_value("1.0")
            .help("Sets the maximum X value of the simulation window"))
            .allow_hyphen_values(true)
        .arg(Arg::with_name("ymin")
            .long("ymin")
            .short('y')
            .value_name("YMIN")
            .takes_value(true)
            .default_value("-1.0")
            .help("Sets the minimum Y value of the simulation window"))
            .allow_hyphen_values(true)
        .arg(Arg::with_name("ymax")
            .long("ymax")
            .short('Y')
            .value_name("YMAX")
            .takes_value(true)
            .default_value("1.0")
            .help("Sets the maximum Y value of the simulation window"))
            .allow_hyphen_values(true)
        .arg(Arg::with_name("equation")
            .long("equation")
            .short('e')
            .value_name("EQUATION")
            .takes_value(true)
            .default_value("x^pi=y")
            .help("The equation to graph"))
        .arg(Arg::with_name("path")
            .long("path")
            .short('p')
            .value_name("PATH")
            .takes_value(true)
            .default_value("images/graph.png")
            .help("The path to save the graphs in"))
        .get_matches();

    let width = matches.value_of("width").unwrap().parse().unwrap();
    let height = matches.value_of("height").unwrap().parse().unwrap();
    let xmin = matches.value_of("xmin").unwrap().parse().unwrap();
    let xmax = matches.value_of("xmax").unwrap().parse().unwrap();
    let ymin = matches.value_of("ymin").unwrap().parse().unwrap();
    let ymax = matches.value_of("ymax").unwrap().parse().unwrap();
    let eq = matches.value_of("equation").unwrap();
    let path = matches.value_of("path").unwrap();

    let graph_settings = GraphSettings {
        path,
        image_width: width,
        image_height: height,
        sim_window: (xmin, xmax, ymin, ymax),
    };

    let mut eq = Equation::new(eq);

    //plot(&mut eq, &graph_settings)?;

    let root = create_root(&graph_settings)?;
    let mut graph = create_graph_3d(&graph_settings, &root)?;

    //plot_3d(&mut eq, &graph_settings, &mut graph)?;
    //plot_x(&mut eq, &graph_settings, &mut graph);

    //plot_3d(&mut Equation::new("x^2+z^2=0.15"), &graph_settings, &mut graph)?;
    plot_3d(&mut Equation::new("x^2+z^2=0.15"), &graph_settings, &mut graph)?;
    plot_3d(&mut Equation::new("(x+0.5)^2+(y+0.7)^2+z^2=0.15"), &graph_settings, &mut graph)?;
    plot_3d(&mut Equation::new("(x-0.5)^2+(y+0.7)^2+z^2=0.15"), &graph_settings, &mut graph)?;
    plot_3d(&mut Equation::new("x^2+(y-0.9)^2+z^2=0.15"), &graph_settings, &mut graph)?;
    //plot_3d(&mut Equation::new("(0.15/0.15)*sqrt(x^2+z^2)=y"), &graph_settings, &mut graph)?;

    root.present()?;

    println!("{}", Path::new(path).canonicalize()?.as_os_str().to_str().unwrap());

    Ok(())
}