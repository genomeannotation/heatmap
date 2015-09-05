extern crate image;

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

use image::{
    GenericImage,
    ImageBuffer,
    Luma,
};

fn main() {
    println!("Loading matrix...");

    let stdin = io::stdin();
    let mut matrix: Vec<Vec<f64>> = Vec::new();
    for line in stdin.lock().lines() {
        let mut row = Vec::new();
        let line = line.unwrap();
        for cell in line.trim_left().trim_right().split('\t') {
            let cell_val: f64 = cell.parse().unwrap();
            row.push(cell_val);
        }
        matrix.push(row);
    }

    // Build row and column sums
    println!("Building sum vectors...");
    let size = matrix.len();
    let mut row_max = vec![0.0; size];
    let mut col_max = vec![0.0; size];

    for y in 0..size {
        for x in 0..size {
            row_max[y] = f64::max(row_max[y], matrix[y][x]);
            col_max[x] = f64::max(col_max[x], matrix[y][x]);
        }
    }

    // Build the image
    println!("Building image...");

    let mut img = ImageBuffer::new(size as u32, size as u32);

    for y in 0..size {
        for x in 0..size {
            let max_cell =
                if y > x {
                    col_max[y]
                } else {
                    row_max[x]
                };
            let cell = matrix[y][x]/max_cell * 255.0;
            img.put_pixel(x as u32, y as u32, Luma([cell as u8]));
        }
    }

    // Save the image as “fractal.png”
    let ref mut out_file = File::create(&Path::new("heatmap.png")).unwrap();

    // We must indicate the image’s color type and what format to save as
    let _ = image::ImageLuma8(img).save(out_file, image::PNG);
}
