extern crate image;

use image::RgbImage;

fn draw_line(img: &mut RgbImage, x0: i64, y0: i64, x1: i64, y1: i64) {

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
        img.get_pixel_mut(x0 as u32, y0 as u32).data = [255, 255, 255];

        // Check end condition
        if x0 == x1 && y0 == y1 { break };

        // Store old error
        err2 = 2 * err;

        // Adjust error and start position
        if err2 > -dx { err -= dy; x0 += sx; }
        if err2 < dy { err += dx; y0 += sy; }
    }

}

const SIX_F: f64 = 6 as f64;
const THREE_F: f64 = 3 as f64;
const TWO_F: f64 = 2.0 as f64;
// let SQRT_3: f64 = (3 as f64).sqrt();

fn rendersnowflakeside(p1x: f64, p1y: f64, p2x: f64, p2y: f64, n: i64, img: &mut RgbImage){
    if n == 0 {
        draw_line(img, p1x as i64, p1y as i64, p2x as i64, p2y as i64);
    }
    else {
        let n2 = n - 1;
        let deltax = p2x - p1x;
        let deltay = p2y - p1y;
        let deltaxper = deltax / (THREE_F);
        let deltayper = deltay / (THREE_F);
        let mid1x = p1x + deltaxper;
        let mid1y = p1y + deltayper;
        let mid2x = p1x + ((TWO_F) * deltaxper);
        let mid2y = p1y + ((TWO_F) * deltayper);
        let SQRT_3 = (THREE_F).sqrt();
        // let heightxxsum = (THREE_F) * p1x + (THREE_F) * p2x;
        let heightxxsum = (THREE_F) * (p1x + p2x);
        // let heightxysum = SQRT_3 * p1y - SQRT_3 * p2y;
        let heightxysum = SQRT_3 * (p1y - p2y);
        let heightx = (heightxxsum + heightxysum) / (SIX_F);
        // let heightyysum = (THREE_F) * p1y + (THREE_F) * p2y;
        let heightyysum = (THREE_F) * (p1y + p2y);
        // let heightyxsum = SQRT_3 * p2x - SQRT_3 * p1x;
        let heightyxsum = SQRT_3 * (p2x - p1x);
        let heighty = (heightyxsum + heightyysum) / (SIX_F);
        
        if n > 13 {
            let h1 = std::thread::spawn(move ||);
            let h2 = std::thread::spawn(move ||);
            let h3 = std::thread::spawn(move ||);
            let h4 = std::thread::spawn(move ||);
            // usar threads
            let mut chan = 
        }
        else {
            // let v = vec![p1x, p1y, mid1x, mid1y, n2, mid2x, mid2y, p2x, p2y, n2, mid1x, mid1y, heightx, heighty, n2, heightx, heighty, mid2x, mid2y, n2];
            rendersnowflakeside(p1x, p1y, mid1x, mid1y, n2, img);
            rendersnowflakeside(mid2x, mid2y, p2x, p2y, n2, img);
            rendersnowflakeside(mid1x, mid1y, heightx, heighty, n2, img);
            rendersnowflakeside(heightx, heighty, mid2x, mid2y, n2, img);
        }//
    }
}

use std::thread;
use std::sync::mpsc;  // mpsc: multiple producer, single consumer
use std::sync::{Arc, Mutex};

fn main() {
	let rezscale = 10;
    let mut img = RgbImage::new(640 * rezscale, 480 * rezscale);

    let data = Arc::new(Mutex::new(img.clone()));
    // println!("{:?}", data);
    let rezscale = rezscale as f64;  // nao precisa mas do valor inteiro
    let nrec = 13;
    // rendersnowflakeside(270.0, 211.13249, 320.0, 297.73503, nrec, &mut img);
    // println!("one side done!");
    // rendersnowflakeside(370.0, 211.13249, 270.0, 211.13249, nrec, &mut img);
    // println!("one side done!");
    // rendersnowflakeside(320.0, 297.73503, 370.0, 211.13249, nrec, &mut img);
    // println!("one side done!");
    rendersnowflakeside(270.0 * rezscale, 211.13249 * rezscale, 320.0 * rezscale, 297.73503 * rezscale, nrec, &mut img);
    println!("one side done!");
    rendersnowflakeside(370.0 * rezscale, 211.13249 * rezscale, 270.0 * rezscale, 211.13249 * rezscale, nrec, &mut img);
    println!("one side done!");
    rendersnowflakeside(320.0 * rezscale, 297.73503 * rezscale, 370.0 * rezscale, 211.13249 * rezscale, nrec, &mut img);
    println!("one side done!");
    println!("Vai escrever...");
    img.save("output.png").unwrap();
    println!("Escreveu");
}