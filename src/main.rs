use clap::{Arg, App};
use mathsolver::equation::Equation;
//use plotters::coord::types::RangedCoordf64;
use plotters::prelude::*;
use plotters_bitmap::BitMapBackend;
use std::error::Error;
use std::path::Path;

//type Chart<'a> = ChartContext<'a, BitMapBackend<'a>, Cartesian2d<RangedCoordf64, RangedCoordf64>>;
//type Root<'a> = DrawingArea<BitMapBackend<'a>, plotters::coord::Shift>;
struct GraphSettings<'a> {
    path: &'a str,
    image_width: u32,
    image_height: u32,
    sim_window: (f64, f64, f64, f64)
}

/*fn create_root<'a>(settings: &GraphSettings<'a>) -> Result<Root<'a>, Box<dyn Error>> {
    let root = BitMapBackend::new(settings.path, (settings.image_width, settings.image_height)).into_drawing_area();
    root.fill(&WHITE)?;
    Ok(root)
}

fn create_graph<'a>(settings: &GraphSettings<'a>, root: &'a Root<'a>) -> Result<Chart<'a>, Box<dyn Error>> {
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
}*/


fn plot(eq: &mut Equation, settings: &GraphSettings, /*chart: &Chart*/) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(settings.path, (settings.image_width, settings.image_height)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .set_all_label_area_size(30)
        .build_cartesian_2d(settings.sim_window.0..settings.sim_window.1, settings.sim_window.2..settings.sim_window.3)?;

    chart
        .configure_mesh()
        .label_style(("sans-serif", 15).into_font().color(&BLACK))
        .axis_style(&BLACK)
        .draw()?;
    
    let fidelity_w = settings.image_width;
    let fidelity_h = settings.image_height;

    for j in -(fidelity_h as i32)..=fidelity_h as i32 {
        let y = ((j as f64) / fidelity_h as f64) * (settings.sim_window.3-settings.sim_window.2) / 2.0;
        for i in -(fidelity_w as i32)..=fidelity_w as i32 {
            let x = ((i as f64) / fidelity_w as f64) * (settings.sim_window.1-settings.sim_window.0) / 2.0;

            let val = eq.call_on(&[("x", x), ("y", y)]);
            if val < 0.003 && val > -0.003{
                chart.plotting_area().draw_pixel((x, y), &BLACK)?;
            }
        }
    }

    /*let mut data = Vec::new();

    chart.draw_series(LineSeries::new(
        (-(fidelity as i32)..=fidelity as i32)
            .map(|i| ((i as f64) / fidelity as f64) * (settings.sim_window.1-settings.sim_window.0) + settings.sim_window.0)
            .map(|x| {
                (x, eq.call_on(&[("x", x)]))
            }),
        BLACK.stroke_width(3),
    ))?;*/

    root.present()?;

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
            .default_value("x^pi")
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

    plot(&mut eq, &graph_settings)?;

    /*let root = create_root(&graph_settings)?;

    {
        let graph = create_graph(&graph_settings, &root)?;
        plot(&mut eq, &graph_settings, &graph)?;
    }

    root.present()?;*/

    println!("{}", Path::new(path).canonicalize()?.as_os_str().to_str().unwrap());

    Ok(())
}