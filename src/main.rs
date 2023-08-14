use clap::{Arg, App};
use mathsolver::complex::Complex;
use mathsolver::equation::{Equation, CustomOperations, Node, ImplStandardOperations};
use plotters::coord::types::RangedCoordf64;
use plotters::prelude::*;
use plotters_bitmap::BitMapBackend;
use std::error::Error;
use std::path::Path;

type Chart<'a, 'b> = ChartContext<'a, BitMapBackend<'b>, Cartesian2d<RangedCoordf64, RangedCoordf64>>;
type Root<'a> = DrawingArea<BitMapBackend<'a>, plotters::coord::Shift>;
struct GraphSettings<'a> {
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


struct SubEqual;

impl CustomOperations for SubEqual {
    fn equal_f64(lhs: f64, rhs: f64) -> Node {
        Node::Real(lhs - rhs)
    }
}

fn marching_squares(width: usize, height: usize, value_grid: Vec<f64>) -> Vec<[(f64, f64); 2]> {

    let scale_x = |x: f64| -> f64 {
        (x / (width - 1) as f64) * 2.0
    };
    
    let scale_y = |y: f64| -> f64 {
        (y / (height - 1) as f64) * 2.0
    };
    
    let mut lines = Vec::new();

    for y in 0..height-1 {
        for x in 0..width-1 {
            let a = value_grid[x+y*width] < 0.0;
            let b = value_grid[x+1+y*width] < 0.0;
            let c = value_grid[x+(y+1)*width] < 0.0;
            let d = value_grid[x+1+(y+1)*width] < 0.0;
        
            let x = scale_x(x as f64) - 1.0;
            let y = scale_y(y as f64) - 1.0;

            let index = c as u32 + d as u32 * 2 + b as u32 * 4 + a as u32 * 8;

            match index {
                0b0000 | 0b1111 => {}
                0b0001 | 0b1110 => lines.push([(x, y + scale_y(0.5)), (x + scale_x(0.5), y + scale_y(1.0))]),
                0b0010 | 0b1101 => lines.push([(x + scale_x(0.5), y + scale_y(1.0)), (x + scale_x(1.0), y + scale_y(0.5))]),
                0b0011 | 0b1100 => lines.push([(x, y + scale_y(0.5)), (x + scale_x(1.0), y + scale_y(0.5))]),
                0b0100 => lines.push([(x + scale_x(0.5), y), (x + scale_x(1.0), y + scale_y(0.5))]),
                0b0101 => {
                    lines.push([(x, y + scale_y(0.5)), (x + scale_x(0.5), y)]);
                    lines.push([(x + scale_x(0.5), y + scale_y(1.0)), (x + scale_x(1.0), y + scale_y(0.5))]);
                },
                0b0110 | 0b1001 => lines.push([(x + scale_x(0.5), y + scale_y(0.0)), (x + scale_x(0.5), y + scale_y(1.0))]), // Vertical
                0b0111 | 0b1000 => lines.push([(x, y + scale_y(0.5)), (x + scale_x(0.5), y)]),
                0b1010 => {
                    lines.push([(x, y + scale_y(0.5)), (x + scale_x(0.5), y + scale_y(1.0))]);
                    lines.push([(x + scale_x(0.5), y), (x + scale_x(1.0), y + scale_y(0.5))]);
                },
                0b1011 => lines.push([(x + scale_x(0.5), y), (x + scale_x(1.0), y + scale_y(0.5))]),
                _ => panic!()
            }
        }
    }

    lines
}

fn plot(eq: &mut Equation, settings: &GraphSettings, chart: &mut Chart) -> Result<(), Box<dyn Error>> {
    
    let fidelity_w = settings.image_width as usize;
    let fidelity_h = settings.image_height as usize;
    
    let transform_x = |x: i32| -> f64 {
        ((x as f64) / fidelity_w as f64) * (settings.sim_window.1-settings.sim_window.0) / 2.0
    };
    
    let transform_y = |y: i32| -> f64 {
        ((y as f64) / fidelity_h as f64) * (settings.sim_window.3-settings.sim_window.2) / 2.0
    };

    let mut value_grid = vec![0.0; (fidelity_w*2+1)*(fidelity_h*2+1)];

    let mut total_index = 0;

    for j in -(fidelity_h as i32)..=fidelity_h as i32 {
        let y = transform_y(j);
        for i in -(fidelity_w as i32)..=fidelity_w as i32 {
            let x = transform_x(i);
            match eq.call_on_custom::<SubEqual>(&[("x", x), ("y", y)]) {
                Node::Bool(val) => {
                    if val {
                        chart.plotting_area().draw_pixel((x, y), &RGBAColor(0, 0, 0, 0.4))?;
                    }
                },
                Node::Real(val) => {
                    value_grid[total_index] = val;
                    total_index += 1;
                }
                _ => panic!()
            }
        }
    }

    let lines = marching_squares(fidelity_w * 2 + 1, fidelity_h * 2 + 1, value_grid);

    lines.into_iter().try_for_each(|lines| -> Result<(), Box<dyn Error>> {
        chart.draw_series(
            LineSeries::new(
                lines,
                BLACK.stroke_width(2)
            )
        )
        .map(|_| ())
        .map_err(|e| Box::new(e) as Box<dyn Error>)
    })?;

    Ok(())

}

fn plot_x(eq: &mut Equation, settings: &GraphSettings, chart: &mut Chart) -> Result<(), Box<dyn Error>> {
    let fidelity = settings.image_width / 8;

    chart.draw_series(LineSeries::new(
        (-(fidelity as i32)..=fidelity as i32)
            .map(|i| ((i as f64) / fidelity as f64) * (settings.sim_window.1-settings.sim_window.0) / 2.0)
            .map(|x| {
                (x, eq.call_on(&[("x", x)]).as_f64().unwrap())
            }),
        BLACK.stroke_width(2),
    ))?;

    Ok(())
}

fn plot_y(eq: &mut Equation, settings: &GraphSettings, chart: &mut Chart) -> Result<(), Box<dyn Error>> {
    let fidelity = settings.image_height / 8;

    chart.draw_series(LineSeries::new(
        (-(fidelity as i32)..=fidelity as i32)
            .map(|i| ((i as f64) / fidelity as f64) * (settings.sim_window.3-settings.sim_window.2) / 2.0)
            .map(|y| {
                (eq.call_on(&[("y", y)]).as_f64().unwrap(), y)
            }),
        BLACK.stroke_width(2),
    ))?;

    Ok(())
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
        .arg(Arg::with_name("xmax")
            .long("xmax")
            .short('X')
            .value_name("XMAX")
            .takes_value(true)
            .default_value("1.0")
            .help("Sets the maximum X value of the simulation window"))
        .arg(Arg::with_name("ymin")
            .long("ymin")
            .short('y')
            .value_name("YMIN")
            .takes_value(true)
            .default_value("-1.0")
            .help("Sets the minimum Y value of the simulation window"))
        .arg(Arg::with_name("ymax")
            .long("ymax")
            .short('Y')
            .value_name("YMAX")
            .takes_value(true)
            .default_value("1.0")
            .help("Sets the maximum Y value of the simulation window"))
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
    let mut graph = create_graph(&graph_settings, &root)?;

    plot(&mut eq, &graph_settings, &mut graph)?;
    //plot_x(&mut eq, &graph_settings, &mut graph);

    root.present()?;

    println!("{}", Path::new(path).canonicalize()?.as_os_str().to_str().unwrap());

    Ok(())
}