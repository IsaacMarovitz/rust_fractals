use num::complex::Complex;

pub fn julia(c: [f64; 2], z: [f64; 2], escape_radius: u32, max_iterations: u32) -> u32 {
    let mut iterations: u32 = 1;
    let mut zx: f64 = z[0];
    let mut zy: f64 = z[1];

    while zx * zx + zy * zy < (escape_radius * escape_radius) as f64 && iterations < max_iterations {
        let xtemp = zx * zx - zy * zy;
        zy = 2.0 as f64 * zx * zy + c[1];
        zx = xtemp as f64 + c[0];

        iterations += 1;
    }

    return iterations;
}

pub fn mandelbrot(c: Complex<f64>, z: Complex<f64>, iterations: u32, max_iterations: u32) -> u32 {
    let z_new:Complex<f64> = z*z + c;

    if z_new.norm() > 2.0 || iterations >= max_iterations {
        return iterations;
    } else {
        let iterations = iterations + 1;
        return mandelbrot(c, z_new, iterations, max_iterations);
    }
}