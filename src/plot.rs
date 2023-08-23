use mathsolver::equation::{Equation, Node, ImplStandardOperations};
use plotters::prelude::*;

use std::error::Error;

use crate::{GraphSettings, Chart, SubEqual, marching_squares, Chart3D, marching_cubes};


pub fn plot(eq: &mut Equation, settings: &GraphSettings, chart: &mut Chart) -> Result<(), Box<dyn Error>> {
    
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

pub fn plot_x(eq: &mut Equation, settings: &GraphSettings, chart: &mut Chart) -> Result<(), Box<dyn Error>> {
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

pub fn plot_y(eq: &mut Equation, settings: &GraphSettings, chart: &mut Chart) -> Result<(), Box<dyn Error>> {
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

pub fn plot_3d(eq: &mut Equation, settings: &GraphSettings, chart: &mut Chart3D) -> Result<(), Box<dyn Error>> {
    let fidelity_w = settings.image_width as usize / 20;
    let fidelity_h = settings.image_height as usize / 20;
    
    let transform_x = |x: i32| -> f64 {
        ((x as f64) / fidelity_w as f64) * (settings.sim_window.1-settings.sim_window.0) / 2.0
    };
    
    let transform_y = |y: i32| -> f64 {
        ((y as f64) / fidelity_h as f64) * (settings.sim_window.3-settings.sim_window.2) / 2.0
    };

    let mut value_grid = vec![0.0; (fidelity_w*2+1)*(fidelity_w*2+1)*(fidelity_h*2+1)];

    let mut total_index = 0;

    // (-(fidelity_w as i32)..=fidelity_w as i32).into_par_iter().map(|l| {
    //     let z = transform_x(l);
    //     (-(fidelity_h as i32)..=fidelity_h as i32).map(|j| {
    //         let y = transform_y(j);
    //         (-(fidelity_w as i32)..=fidelity_w as i32).filter_map(move |i| {
    //             let x = transform_x(i);
    //             match eq.call_on(&[("x", x), ("y", y), ("z", z)]) {
    //                 Node::Bool(val) => {
    //                     if val {
    //                         Some(RGBAColor(0, 0, 0, 0.4))
    //                     } else {
    //                         None
    //                     }
    //                 },
    //                 Node::Real(val) => {
    //                     //value_grid[total_index] = val;
    //                     //total_index += 1;
    //                     None
    //                 }
    //                 _ => panic!()
    //             }
    //         })
    //     })
    // });

    for l in -(fidelity_w as i32)..=fidelity_w as i32 {
        let z = transform_x(l);
        for j in -(fidelity_h as i32)..=fidelity_h as i32 {
            let y = transform_y(j);
            for i in -(fidelity_w as i32)..=fidelity_w as i32 {
                let x = transform_x(i);
                match eq.call_on_custom::<SubEqual>(&[("x", x), ("y", y), ("z", z)]) {
                    Node::Bool(val) => {
                        if val {
                            chart.plotting_area().draw_pixel((x, y, z), &RGBAColor(0, 0, 0, 0.4))?;
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
    }

    let triangles = marching_cubes(fidelity_w * 2 + 1, fidelity_h * 2 + 1, fidelity_w * 2 + 1, value_grid);

    chart.draw_series(
        triangles.into_iter().map(|triangle| Polygon::new(triangle, RGBAColor(0, 0, 0, 0.4).stroke_width(2))),
    )?;

    Ok(())

}
