extern crate image;

use image::RgbImage;

const SIX_F: f64 = 6 as f64;
const THREE_F: f64 = 3 as f64;
const TWO_F: f64 = 2.0 as f64;
// let SQRT_3: f64 = (3 as f64).sqrt();

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

// TODO: refatorar o cohdigo pra usar enum
// enum Gambs {
//     arc(Arc<Mutex<RgbImage>>),
//     img(&mut RgbImage)
// }

fn render_snow_flake_side_single(p1x: f64, p1y: f64, p2x: f64, p2y: f64, n: i64, mut img: &mut RgbImage, pot4: i32){
    if n == 0{
        draw_line(&mut img, p1x as i64, p1y as i64, p2x as i64, p2y as i64);
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
        render_snow_flake_side_single(p1x, p1y, mid1x, mid1y, n2, &mut img, pot4-1);
        render_snow_flake_side_single(mid2x, mid2y, p2x, p2y, n2, &mut img, pot4-1);
        render_snow_flake_side_single(mid1x, mid1y, heightx, heighty, n2, &mut img, pot4-1);
        render_snow_flake_side_single(heightx, heighty, mid2x, mid2y, n2, &mut img, pot4-1);
        //
    }
}

fn render_snow_flake_side_multi(p1x: f64, p1y: f64, p2x: f64, p2y: f64, n: i64, arc: Arc<Mutex<RgbImage>>, pot4: i32){
    if n == 0 {
        let mut img = arc.lock().unwrap();
        draw_line(&mut *img, p1x as i64, p1y as i64, p2x as i64, p2y as i64);
    }
    else {
        let n2 = n - 1;
        // let deltax = p2x - p1x;
        // let deltay = p2y - p1y;
        let deltaxper = (p2x - p1x) / (THREE_F);
        let deltayper = (p2y - p1y) / (THREE_F);
        let mid1x = p1x + deltaxper;
        let mid1y = p1y + deltayper;
        let mid2x = p1x + ((TWO_F) * deltaxper);
        let mid2y = p1y + ((TWO_F) * deltayper);
        let SQRT_3 = (THREE_F).sqrt();
        // let heightxxsum = (THREE_F) * p1x + (THREE_F) * p2x;
        let heightxxsum = (THREE_F) * (p1x + p2x);
        // let heightxysum = SQRT_3 * p1y - SQRT_3 * p2y;
        // let heightxysum = SQRT_3 * (p1y - p2y);
        let heightxysum = SQRT_3 * (p1y - p2y);
        let heightx = (heightxxsum + heightxysum) / (SIX_F);
        // let heightyysum = (THREE_F) * p1y + (THREE_F) * p2y;
        let heightyysum = (THREE_F) * (p1y + p2y);
        // let heightyxsum = SQRT_3 * p2x - SQRT_3 * p1x;
        let heightyxsum = SQRT_3 * (p2x - p1x);
        let heighty = (heightyxsum + heightyysum) / (SIX_F);
        
        if pot4 > 0 {
            // println!("Paralelo {}", n2);
            // thread::sleep( std :: time :: Duration :: from_millis(10000) );
            let to_pass = arc.clone(); let h1 = std::thread::spawn(move || render_snow_flake_side_multi(p1x, p1y, mid1x, mid1y, n2, to_pass.clone(), pot4-1) );
            let to_pass = arc.clone(); let h2 = std::thread::spawn(move || render_snow_flake_side_multi(mid2x, mid2y, p2x, p2y, n2, to_pass.clone(), pot4-1 ));
            let to_pass = arc.clone(); let h3 = std::thread::spawn(move || render_snow_flake_side_multi(mid1x, mid1y, heightx, heighty, n2, to_pass.clone(), pot4-1));
            let to_pass = arc.clone(); let h4 = std::thread::spawn(move || render_snow_flake_side_multi(heightx, heighty, mid2x, mid2y, n2, to_pass.clone(), pot4-1));
            h1.join().unwrap(); h2.join().unwrap();
            h3.join().unwrap(); h4.join().unwrap();
            // usar threads
        }
        else {
            // println!("Sequencial {}", n2);
            // thread::sleep( std :: time :: Duration :: from_millis(10000) );
            // println!("{:?}", n);
            let mut img = &mut * (arc.lock().unwrap());
            render_snow_flake_side_single(p1x, p1y, mid1x, mid1y, n2, &mut img, pot4-1);
            render_snow_flake_side_single(mid2x, mid2y, p2x, p2y, n2, &mut img, pot4-1);
            render_snow_flake_side_single(mid1x, mid1y, heightx, heighty, n2, &mut img, pot4-1);
            render_snow_flake_side_single(heightx, heighty, mid2x, mid2y, n2, &mut img, pot4-1);
            // let mut h = vec![std::thread:::spawn(||)];
            // for i in 2..5 {
            //     h.push_back(std::thread:::spawn(||));
            // }
            // // let v = vec![p1x, p1y, mid1x, mid1y, n2, mid2x, mid2y, p2x, p2y, n2, mid1x, mid1y, heightx, heighty, n2, heightx, heighty, mid2x, mid2y, n2];
            // render_snow_flake_side_multi(p1x, p1y, mid1x, mid1y, n2, img);
            // render_snow_flake_side_multi(mid2x, mid2y, p2x, p2y, n2, img);
            // render_snow_flake_side_multi(mid1x, mid1y, heightx, heighty, n2, img);
            // render_snow_flake_side_multi(heightx, heighty, mid2x, mid2y, n2, img);
        }//
    }
}

// fn take_ref_mut_arc(_img: &mut RgbImage){
//     println!("Ha! Passagem bem sucedida");
// }

use std::thread;
// use std::sync::mpsc;  // mpsc: multiple producer, single consumer
use std::sync::{Arc, Mutex};

fn main() {
	for &rezscale in [40, 60,  80].iter() {
	    for i in 15..18 {
	        let img = RgbImage::new(640 * rezscale, 480 * rezscale);
	        println!("rezscale: {}", rezscale);
	        let rezscale_int = rezscale;
	        let rezscale = rezscale as f64;  // nao precisa mas do valor inteiro
	        let nrec = i;
	        println!("Recursoes: {}", nrec);
	        let arc = Arc::new( Mutex::new(img) );      // preparando para as threads

	        // let mut _img = RgbImage::new(640 * rezscale, 480 * rezscale);

	        // let data = Arc::new(Mutex::new(img.clone()));
	        let to_pass = arc.clone();

	        let pot4 = 2;
	        let h1 = thread::spawn(move || {
	            render_snow_flake_side_multi(270.0 * rezscale, 211.13249 * rezscale, 320.0 * rezscale, 297.73503 * rezscale, nrec, to_pass.clone()/*Arc::clone(&arc)*/, pot4);
	        });

	        let to_pass = arc.clone();
	        let h2 = thread::spawn(move || {
	            render_snow_flake_side_multi(370.0 * rezscale, 211.13249 * rezscale, 270.0 * rezscale, 211.13249 * rezscale, nrec, to_pass.clone()/*Arc::clone(&arc)*/, pot4 );
	        });
	        let to_pass = arc.clone();
	        let h3 = thread::spawn(move || {
	            render_snow_flake_side_multi(320.0 * rezscale, 297.73503 * rezscale, 370.0 * rezscale, 211.13249 * rezscale, nrec, to_pass.clone()/*Arc::clone(&arc)*/ , pot4);
	        });

	        
	        h1.join().unwrap();
	        println!("one side done!");
	        
	        h2.join().unwrap();
	        println!("one side done!");
	        
	        h3.join().unwrap();
	        println!("one side done!");

	        println!("Vai escrever...");
	        (*(arc.lock().unwrap())).save(rezscale_int.to_string() + &nrec.to_string() +"output.png").unwrap();
	        println!("Escreveu");
	    }
	}
}