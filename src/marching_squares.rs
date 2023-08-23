pub fn marching_squares(width: usize, height: usize, value_grid: Vec<f64>) -> Vec<[(f64, f64); 2]> {

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
            
            let index = c as u32 + d as u32 * 2 + b as u32 * 4 + a as u32 * 8;
        
            let x = scale_x(x as f64) - 1.0;
            let y = scale_y(y as f64) - 1.0;
            
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