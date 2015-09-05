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
    let stdin = io::stdin();
    let mut matrix: Vec<Vec<f64>> = Vec::new();
    let mut max_cell = 0.0;
    for line in stdin.lock().lines() {
        let mut row = Vec::new();
        let line = line.unwrap();
        for cell in line.trim_left().trim_right().split('\t') {
            let cell_val: f64 = cell.parse().unwrap();
            row.push(cell_val);
            max_cell = f64::max(max_cell, cell_val);
        }
        matrix.push(row);
    }

    let (width, height) = (matrix[0].len(), matrix.len());

    //Construct a new ImageBuffer with the specified width and height.
    let mut img = ImageBuffer::new(width as u32, height as u32);

    for y in 0..height {
        for x in 0..width {
            let cell = matrix[y][x]/max_cell * 255.0;
            img.put_pixel(x as u32, y as u32, Luma([cell as u8]));
        }
    }

    // Save the image as “fractal.png”
    let ref mut out_file = File::create(&Path::new("heatmap.png")).unwrap();

    // We must indicate the image’s color type and what format to save as
    let _ = image::ImageLuma8(img).save(out_file, image::PNG);
}
