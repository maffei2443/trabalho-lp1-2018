extern crate image;

use std::time::{SystemTime, UNIX_EPOCH};
use image::RgbImage;
const rezscale : i32 = 5;

const globx : usize = (640 * rezscale) as usize;
const globy : usize = (480 * rezscale) as usize;

fn draw_line(mat: &mut [[bool; globx]; globy], x0: i64, y0: i64, x1: i64, y1: i64) {

    // Create local variables for moving start point
    let mut x0 = x0;
    let mut y0 = y0;

    // Get absolute x/y offset
    let dx = if x0 > x1 { x0 - x1 } else { x1 - x0 };
    let dy = if y0 > y1 { y0 - y1 } else { y1 - y0 };

    // Get slopes
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    // Initialize error
    let mut err = if dx > dy { dx } else {-dy} / 2;
    let mut err2;

    loop {
        // Set pixel
        // img.get_pixel_mut(x0 as u32, y0 as u32).data = [255, 255, 255];
        mat[x0 as usize][y0 as usize] = true;
        // Check end condition
        if x0 == x1 && y0 == y1 { break };

        // Store old error
        err2 = 2 * err;

        // Adjust error and start position
        if err2 > -dx { err -= dy; x0 += sx; }
        if err2 < dy { err += dx; y0 += sy; }
    }
}

fn rendersnowflakeside(p1x: f64, p1y: f64, p2x: f64, p2y: f64, n: i64, mat: &mut [[bool; globx]; globy]){
    if n == 0{
        draw_line(mat, p1x as i64, p1y as i64, p2x as i64, p2y as i64);
    }
    else{
        let n2 = n - 1;
        let deltax = p2x - p1x;
        let deltay = p2y - p1y;
        let deltaxper = deltax / (3 as f64);
        let deltayper = deltay / (3 as f64);
        let mid1x = p1x + deltaxper;
        let mid1y = p1y + deltayper;
        let mid2x = p1x + ((2 as f64) * deltaxper);
        let mid2y = p1y + ((2 as f64) * deltayper);
        let sqrtof3 = (3 as f64).sqrt();
        let heightxxsum = (3 as f64) * p1x + (3 as f64) * p2x;
        let heightxysum = sqrtof3 * p1y - sqrtof3 * p2y;
        let heightx = (heightxxsum + heightxysum) / (6 as f64);
        let heightyysum = (3 as f64) * p1y + (3 as f64) * p2y;
        let heightyxsum = sqrtof3 * p2x - sqrtof3 * p1x;
        let heighty = (heightyxsum + heightyysum) / (6 as f64);
        rendersnowflakeside(p1x, p1y, mid1x, mid1y, n2, mat);
        rendersnowflakeside(mid2x, mid2y, p2x, p2y, n2, mat);
        rendersnowflakeside(mid1x, mid1y, heightx, heighty, n2, mat);
        rendersnowflakeside(heightx, heighty, mid2x, mid2y, n2, mat);
        //
    }
}

fn main() {
    // let rezscale = 10;
    let systime = SystemTime::now();
    let mut img = RgbImage::new(globx as u32, globy as u32);
    println!("SEQUENCIA");
    println!("rezscale: {}", rezscale);
    let rezscalef = rezscale as f64;  // nao precisa mas do valor inteiro
    let nrec = 15;
    println!("Recursoes: {}", nrec);
    let mut mat = [[false; globx]; globy];

    rendersnowflakeside(270.0 * rezscalef, 211.13249 * rezscalef, 320.0 * rezscalef, 297.73503 * rezscalef, nrec, &mut mat);
    println!("one side done!");
    rendersnowflakeside(370.0 * rezscalef, 211.13249 * rezscalef, 270.0 * rezscalef, 211.13249 * rezscalef, nrec, &mut mat);
    println!("one side done!");
    rendersnowflakeside(320.0 * rezscalef, 297.73503 * rezscalef, 370.0 * rezscalef, 211.13249 * rezscalef, nrec, &mut mat);
    println!("one side done!");
    
    for x in 0..globx {
        for y in 0..globy {
            if mat[y][x] {
                img.get_pixel_mut(x as u32, y as u32).data = [255, 255, 255];
            }
        }
    }
    
    println!("Vai escrever...");
    img.save((rezscalef as i32).to_string()  + &nrec.to_string() + "output.png").unwrap();
    println!("Escreveu");
        let newtime = SystemTime::now();
    let since_the_epoch = newtime.duration_since(systime)
        .expect("Time went backwards");
    println!("{:?}", since_the_epoch);
}