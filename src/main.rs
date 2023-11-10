use image::{RgbImage, Rgb};
use std::time::SystemTime;
use std::io;
use std::io::Write;
use std::f64;
use std::fs;
use indicatif::{ProgressBar, ProgressStyle};
use crossterm::{ExecutableCommand, terminal};
use std::process::Command;
use num::complex::Complex;

mod colour;
mod fractals;

fn input(message: &str, failure_message: &str) -> u32 {
    let mut input_received = false;
    let mut return_int = 0;

    while !input_received {
        print!("{}", message);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input!");
        input = input.trim().to_string();
        match input.parse::<u32>() {
            Ok(n) => {
                return_int = n; 
                input_received = true;
            },
            Err(_) => {
                println!("{}", failure_message);
            },
        }
    }
    return_int
}

fn main() {
    io::stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();

    let x_size = input("Input x size: ", "Only input integers!");
    let y_size = input("Input y size: ", "Only input integers!");
    let x_limits: [f64; 2] = [-2.0, 2.0];
    let y_limits: [f64; 2] = [-2.0, 2.0];
    let escape_radius = 10;
    let max_iterations = 255;
    let mut img = RgbImage::new(x_size, y_size);
    let start_time = SystemTime::now();

    let max: f64 = f64::consts::PI * 2f64;
    let step = 0.01;
    let mut current: f64 = 0.0;
    let mut i: u32 = 0;
    let pb = ProgressBar::new((max/step) as u64);

    match fs::remove_dir_all("./imgs") {
        Ok(_) => {},
        Err(_) => {},
    }

    match fs::create_dir_all("./imgs") {
        Ok(_) => {},
        Err(msg) => {
            panic!(msg);
        },
    }
    
    io::stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();

    // Render Video
    println!("Rendering a {:?} x {:?} animation of the Julia Set", x_size, y_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})").expect("Failed to make progress style.")
        .progress_chars("=>-"));
    
    while current < max {
        pb.set_position(i as u64);
        for y in 0..y_size {
            let cy = y as f64 * (y_limits[1] - y_limits[0]) / y_size as f64 + y_limits[0];
            for x in 0..x_size {
                let cx = x as f64 * (x_limits[1] - x_limits[0]) / x_size as f64 + x_limits[0];
                let julia_num: u32 = fractals::julia([current.cos(), current.sin()], [cx, cy], escape_radius, max_iterations);
                img.put_pixel(x, y, Rgb(colour::hsl_to_rgb(julia_num as f32 * 15.0 / 255.0 * 360.0, 100.0, 50.0)));
            }
        }
        img.save("./imgs/".to_owned() + &i.to_string() + ".png").expect("Image failed to save.");
        i += 1;
        current = current + step;
    }
    pb.finish();
    println!("Finished generating frames");
    println!("Beginning video generation");

    match Command::new("ffmpeg")
            .args(&["-framerate", "60", "-i", "./imgs/%d.png", "-pix_fmt", "yuv420p", "Julia.mp4", "-y"])
            .output() {
        Ok(_) => {
            println!("Finished generating video");
            println!("Finished Julia Set in {:.1} seconds", start_time.elapsed().unwrap().as_secs_f32());
        },
        Err(_) => {
            println!("Failed to make video! Do you have ffmpeg installed to PATH on your system?");
        },
    } 
    match fs::remove_dir_all("./imgs") {
        Ok(_) => {},
        Err(_) => {},
    }

    // Render Julia Set Image
    println!("Rendering image of the Julia Set");
    let pb = ProgressBar::new(x_size as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})").expect("Failed to make progress style.")
        .progress_chars("=>-"));
    let start_time = SystemTime::now();
    let mut img = RgbImage::new(x_size, y_size);
    let x_limits: [f64; 2] = [-1.5, 1.5];
    let y_limits: [f64; 2] = [-1.5, 1.5];
    
    for y in 0..y_size {
        pb.set_position(y as u64);
        let cy = y as f64 * (y_limits[1] - y_limits[0]) / y_size as f64 + y_limits[0];
        for x in 0..x_size {
            let cx = x as f64 * (x_limits[1] - x_limits[0]) / x_size as f64 + x_limits[0];
            let julia_num: u8 = fractals::julia([-0.7, 0.27015], [cx, cy], escape_radius, max_iterations) as u8;
            img.put_pixel(x, y, Rgb([julia_num, julia_num, julia_num]));
        }
    }
    img.save("Julia.png").expect("Image failed to save.");
    pb.finish();
    println!("Finished Julia Set in {:.1} seconds", start_time.elapsed().unwrap().as_secs_f32());

    // Render Mandelbrot
    println!("Rendering image of the Mandelbrot Set");
    let pb = ProgressBar::new(x_size as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})").expect("Failed to make progress style.")
        .progress_chars("=>-"));
    let start_time = SystemTime::now();
    let x_limits: [f64; 2] = [0.12, 0.22];
    let y_limits: [f64; 2] = [-0.65, -0.55];
    let mut img = RgbImage::new(x_size, y_size);
    
    for y in 0..y_size {
        pb.set_position(y as u64);
        let cy = y as f64 * (y_limits[1] - y_limits[0]) / y_size as f64 + y_limits[0];
        for x in 0..x_size {
            let cx = x as f64 * (x_limits[1] - x_limits[0]) / x_size as f64 + x_limits[0];
            let c: Complex<f64> = Complex::new(cx, cy);
            let mandelbrot_num = fractals::mandelbrot(c, Complex::new(0.0, 0.0), 1, max_iterations);
            let final_num = mandelbrot_num.to_string().chars().last().unwrap().to_string().parse::<u32>().unwrap();

            let purple = Rgb([159, 0, 255]);
            let green = Rgb([0, 178, 51]);
            let yellow = Rgb([255, 237, 0]);
            let red = Rgb([255, 63, 49]);
            let blue = Rgb([0, 205, 255]);

            if final_num == 0 {
                img.put_pixel(x, y, blue);
            } else if final_num == 1 {
                img.put_pixel(x, y, Rgb([255, 255, 255]));
            } else if final_num == 2 {
                img.put_pixel(x, y, green);
            } else if final_num == 3 {
                img.put_pixel(x, y, yellow);
            } else if final_num == 4 {
                img.put_pixel(x, y, red);
            } else if final_num == 5 {
                img.put_pixel(x, y, blue);
            } else if final_num == 6 {
                img.put_pixel(x, y, purple);
            } else if final_num == 7 {
                img.put_pixel(x, y, green);
            } else if final_num == 8 {
                img.put_pixel(x, y, yellow);
            } else if final_num == 9 {
                img.put_pixel(x, y, red);
            } 
        }
    }
    img.save("Mandelbrot.png").expect("Image failed to save.");
    pb.finish();
    println!("Finished Mandelbrot Set in {:.1} seconds", start_time.elapsed().unwrap().as_secs_f32());
}