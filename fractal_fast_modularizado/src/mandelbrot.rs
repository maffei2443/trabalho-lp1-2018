extern crate rayon;
use self::rayon::prelude::*;


extern crate num_complex;
use self::num_complex::Complex;

extern crate image;
use self::image::RgbImage;

// Taken from: https://rosettacode.org/wiki/Mandelbrot_set#Rust

extern crate rand; // usado para colorir a imagem

const CXMIN: f32 = -1.5f32;
const CXMAX: f32 = 0.7f32;
const CYMIN: f32 = -1f32;
const CYMAX: f32 = 1f32;
const MAX_ITERATIONS: u64 = 1000u64;
const IMG_SIDE: u64 = 2000u64;
const SCALEX: f32 = (CXMAX - CXMIN) / IMG_SIDE as f32;
const SCALEY: f32 = (CYMAX - CYMIN) / IMG_SIDE as f32;


#[allow(dead_code)]
fn enumerate_vec(input: Vec<bool>) -> Vec<(usize, bool)>{
    input.par_iter() 
         .enumerate()
         .map(|(a, b)| (a, *b))         
         .collect()
}

#[allow(dead_code)]
fn belong_mandelbrot( x: f32, y: f32) -> bool {
    let mut z = Complex::new(0f32, 0f32);
    let c = Complex::new(CXMIN + x * SCALEX, CYMIN + y * SCALEY);
    for _ in 0..MAX_ITERATIONS {
        if z.norm() > 2.0 {
            return false;
        }
        z = z * z + c;
    }
    return true;
}

#[allow(dead_code)]
fn vet_mat_idx(a: usize) -> (u64, u64) {
 	let a = a as u64;
 	let x = a / IMG_SIDE;
 	// println!("x == {}\na == {}", x, a);
 	let y = if a > x * IMG_SIDE {  // Não entendi, mas sem isso dá overflow. Não era pra dar. Anyway..
 		a - x * IMG_SIDE - 1		
 	}
 	else {
 		0
 	};
 	// println!("Not panicked!");
 	// println!("Not panicked: {}", y);
 	(x as u64, y as u64)
}

#[allow(dead_code)]
fn generate_mandelbrot(v: Vec<(usize, bool)>) -> Vec<(usize, bool)>{
	v.par_iter()
	 .map(| ( a, _ ) | {
	 	let (x, y) = vet_mat_idx(*a);
 		(*a, belong_mandelbrot(x as f32, y as f32))
	 })
	 .collect()
}


#[allow(unused_must_use)]
#[allow(unused_assignments)]
pub fn get_mandelbrot() -> RgbImage {
    // let mut mat = [[true; IMG_SIDE]; IMG_SIDE];
    let mut _modd: u8 = 20;
	let _scalex = (CXMAX - CXMIN) / IMG_SIDE as f32;
	let _scaley = (CYMAX - CYMIN) / IMG_SIDE as f32;

	let v: Vec<bool> = vec![false; (IMG_SIDE * IMG_SIDE) as usize];
    // let v = 
    let v = generate_mandelbrot(enumerate_vec(v));
    let mut buf = RgbImage::new(IMG_SIDE as u32, IMG_SIDE as u32);

    let _u: Vec<()> = v.iter()
     .map(|(a, b)| {
     	let mut modd = rand::random();
        match modd {
                0...3 => modd = 3,
                4...67 => modd = 12,
                68...99 => modd = 66,
                // 240..=300 => modd -=24,
                100..=255 => modd -=70,
                _ => modd=modd,
        }
     	let (x, y) = vet_mat_idx(*a);
     	if *b == true {
     		buf.get_pixel_mut(x as u32, y as u32).data = 
     			[131, 55, 192];
     	}
     	else {
     		buf.get_pixel_mut(x as u32, y as u32).data = [0, 0, 0];
     	}
     })
     .collect();
    buf.save(format!("131,55,192_res_{}_{}mandel_colorido.png",
		     IMG_SIDE.to_string(), MAX_ITERATIONS.to_string()));
    println!("Escreveu mandelrot");
    buf
    
	// let v = enumerate_vec(v);
}