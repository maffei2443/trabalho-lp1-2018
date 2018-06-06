extern crate image;

use image::RgbImage;

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
        // falta fazer a transformacao linear!
        // img.get_pixel_mut( (x0 as f64 *COS60 - y0 as f64 *SEN60 ) as u32, ( y0 as f64 *COS60 + x0 as f64 *SEN60) as u32).data = [255, 255, 255];
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

fn render_snow_flake_side_pre(p1x: f64, p1y: f64, p2x: f64, p2y: f64, n: i64, arc: Arc<Mutex<Vec<Point<u32>>>>){
    let mut vet = &mut *arc.lock().unwrap();
    render_snow_flake_side(p1x, p1y, p2x, p2y, n, &mut vet);
}

// fn take_ref_mut_arc(_img: &mut RgbImage){
//     println!("Ha! Passagem bem sucedida");
// }

use std::thread;
// use std::sync::mpsc;  // mpsc: multiple producer, single consumer
use std::sync::{Arc, Mutex};
fn main() {
    let x = 13;  // NAO AUMMENTAR! 
    let y = 1;
	for &rezscale in [10].iter() {
	    for i in x..(x+y) {
	        let mut img = RgbImage::new(640 * rezscale, 480 * rezscale);
	        println!("rezscale: {}", rezscale);
	        let rezscale_int = rezscale;
	        let rezscale = rezscale as f64;  // nao precisa mas do valor inteiro
	        let nrec = i;
	        println!("Recursoes: {}", nrec);
            let arc1 = Arc::new( Mutex::new( Vec::new() ) );      // preparando para as threads
            let arc2 = Arc::new( Mutex::new( Vec::new() ) );      // preparando para as threads
	        let arc3 = Arc::new( Mutex::new( Vec::new() ) );      // preparando para as threads

	        // let mut _img = RgbImage::new(640 * rezscale, 480 * rezscale);

	        // let data = Arc::new(Mutex::new(img.clone()));
	        // let to_pass = arc.clone();

            let to_pass = arc1.clone();
            let h1 = thread::spawn(move || {
                 render_snow_flake_side_pre(270.0 * rezscale, 211.13249 * rezscale, 320.0 * rezscale, 297.73503 * rezscale, nrec, to_pass.clone());
            });

	        let to_pass = arc2.clone();
            
            let h2 = thread::spawn(move || {
                render_snow_flake_side_pre(370.0 * rezscale, 211.13249 * rezscale, 270.0 * rezscale, 211.13249 * rezscale, nrec, to_pass.clone() );
            });
	        
            let to_pass = arc3.clone();

	        let h3 = thread::spawn(move || {
	             render_snow_flake_side_pre(320.0 * rezscale, 297.73503 * rezscale, 370.0 * rezscale, 211.13249 * rezscale, nrec, to_pass.clone());
	        });

	        
	        h1.join().unwrap();	        println!("h1 done!");
	        
	        h2.join().unwrap();	        println!("h2 done!");
	        
	        h3.join().unwrap();	        println!("h3 done!");

            let v1 = &*arc1.lock().unwrap();
            let v2 = &*arc2.lock().unwrap();
            let v3 = &*arc3.lock().unwrap();

            for i in v1.iter() {
                img.get_pixel_mut(i.x as u32, i.y as u32).data = [255, 255, 255];
            }

            for i in v2.iter() {
             img.get_pixel_mut(i.x as u32, i.y as u32).data = [255, 255, 255];   
            }

            for i in v3.iter() {
                img.get_pixel_mut(i.x as u32, i.y as u32).data = [255, 255, 255];
            }

	        println!("Vai escrever...");
	        img.save(rezscale_int.to_string()+ "_"  + &nrec.to_string() + "_output.png").unwrap();
	        println!("Escreveu");
	    }
	}
}
