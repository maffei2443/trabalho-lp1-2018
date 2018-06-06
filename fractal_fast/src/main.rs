extern crate image;

use image::RgbImage;

const SIX_F: f64 = 6 as f64;
const THREE_F: f64 = 3 as f64;
#[allow(dead_code)]
const NINE_F: f64 = 9 as f64;
#[allow(dead_code)]
const TWELVE_F: f64 = 9 as f64;
const TWO_F: f64 = 2 as f64;

// const SEN60: f64 = 0.8660254037;
// const COS60: f64 = 0.5;

// let sqrt_3: f64 = (3 as f64).sqrt();

struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}
use std::fmt;

impl<T: fmt::Display> fmt::Display for Point<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "x: {}, y: {}", self.x, self.y)  // coloca dados no buffer
    }
}

fn draw_line(vet: &mut Vec<Point<u32>>, mut x0: i64, mut y0: i64, x1: i64, y1: i64) {

    // Create local variables for moving start point
    // let mut x0 = x0;
    // let mut y0 = y0;

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
        vet.push(Point::new(x0 as u32, y0 as u32));
        // img.get_pixel_mut(x0 as u32, y0 as u32).data = [255, 255, 255];

        // Check end condition
        if x0 == x1 && y0 == y1 { break };

        // Store old error
        err2 = 2 * err;

        // Adjust error and start position
        if err2 > -dx { err -= dy; x0 += sx; }
        if err2 < dy { err += dx; y0 += sy; }
    }
}

// TODO: refatorar o cohdigo pra usar enum
// enum Gambs {
//     arc(Arc<Mutex<RgbImage>>),
//     img(&mut RgbImage)
// }

fn render_snow_flake_side(p1x: f64, p1y: f64, p2x: f64, p2y: f64, n: i64,  vet: &mut Vec<Point<u32>>){
    if n == 0{
        draw_line(vet, p1x as i64, p1y as i64, p2x as i64, p2y as i64);
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

        render_snow_flake_side(p1x, p1y, mid1x, mid1y, n2, vet);
        render_snow_flake_side(mid2x, mid2y, p2x, p2y, n2, vet);
        render_snow_flake_side(mid1x, mid1y, heightx, heighty, n2, vet);
        render_snow_flake_side(heightx, heighty, mid2x, mid2y, n2, vet);
        //
    }
}
use std::thread;
// use std::sync::mpsc;  // mpsc: multiple producer, single consumer
use std::sync::{Arc, Mutex};
const X_F: f64 = 1 as f64;
fn main() {
	let x = 16;
    let y = 1;
	for &rezscale in [10].iter() {
	    for i in x..(x+y) {
            // let img = RgbImage::new(640 * ( rezscale as f64 / X_F).ceil() as u32, 480 * (rezscale as f64 / X_F ).ceil() as u32);
	        let img = RgbImage::new(640 * rezscale, 480 * rezscale);
	        println!("rezscale: {}", rezscale);
	        let rezscale_int = rezscale;
	        let rezscale = rezscale as f64;  // nao precisa mas do valor inteiro
	        let nrec = i;
	        println!("Recursoes: {}", nrec);
	        let arc = Arc::new( Mutex::new(img) );      // preparando para as threads

	        // let mut _img = RgbImage::new(640 * rezscale, 480 * rezscale);

	        // let data = Arc::new(Mutex::new(img.clone()));
	        // let to_pass = arc.clone();
            let mut side1:Vec<Point<u32>> = vec![];
            let h1 = thread::spawn(move || {
                 render_snow_flake_side(270.0 * rezscale, 211.13249 * rezscale, 320.0 * rezscale, 297.73503 * rezscale, nrec, &mut side1/*Arc::clone(&arc)*/);
            });

            let mut side2:Vec<Point<u32>> = vec![];
            let h2 = thread::spawn(move || {
                render_snow_flake_side(370.0 * rezscale, 211.13249 * rezscale, 270.0 * rezscale, 211.13249 * rezscale, nrec, &mut side2/*Arc::clone(&arc)*/ );
            });

            let mut side3:Vec<Point<u32>> = vec![];
	        let h3 = thread::spawn(move || {
	             render_snow_flake_side(320.0 * rezscale, 297.73503 * rezscale, 370.0 * rezscale, 211.13249 * rezscale, nrec, &mut side3/*Arc::clone(&arc)*/ );
	        });

	        
	        h1.join().unwrap();
	        println!("h1 done!");
	        
	        h2.join().unwrap();
	        println!("h2 done!");
	        
	        h3.join().unwrap();
	        println!("h3 done!");

	        println!("Vai escrever...");
	        (*(arc.lock().unwrap())).save(rezscale_int.to_string()+ "_"  + &nrec.to_string() + "_"+ &X_F.to_string() +"output.png").unwrap();
	        println!("Escreveu");
	    }
	}
}