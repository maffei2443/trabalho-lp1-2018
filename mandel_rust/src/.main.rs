extern crate rand;



extern crate num_complex;
extern crate image;
use std::fs::File;
use num_complex::Complex;

// Taken from: https://rosettacode.org/wiki/Mandelbrot_set#Rust

use std::path::Path;
use std::io::Write;

pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}
 
pub struct PPM {
    height: u32,
    width: u32,
    data: Vec<u8>,
}
// taken from: https://rosettacode.org/wiki/Bitmap/Write_a_PPM_file#Rust 
impl PPM {
    pub fn new(height: u32, width: u32) -> PPM {
        let size = 3 * height * width;
        let buffer = vec![0; size as usize];
        PPM { height: height, width: width, data: buffer }
    }
 
    fn buffer_size(&self) -> u32 {
        3 * self.height * self.width
    }
 
    fn get_offset(&self, x: u32, y: u32) -> Option<usize> {
        let offset = (y * self.width * 3) + (x * 3);
        if offset < self.buffer_size() {
            Some(offset as usize)
        } else {
            None
        }
    }
 
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<RGB> {
        match self.get_offset(x, y) {
            Some(offset) => {
                let r = self.data[offset];
                let g = self.data[offset + 1];
                let b = self.data[offset + 2];
                Some(RGB {r: r, g: g, b: b})
            },
            None => None
        }
    }
 
    pub fn set_pixel(&mut self, x: u32, y: u32, color: RGB) -> bool {
        match self.get_offset(x, y) {
            Some(offset) => {
                self.data[offset] = color.r;
                self.data[offset + 1] = color.g;
                self.data[offset + 2] = color.b;
                true
            },
            None => false
        }
    }
 
    pub fn write_file(&self, filename: &str) -> ::std::io::Result<()> {
        let path = Path::new(filename);
        let mut file = try!(File::create(&path));
        let header = format!("P6 {} {} 256\n", self.width, self.height);
        try!(file.write(header.as_bytes()));
        try!(file.write(&self.data));
        Ok(())
    }
}

fn main() {
    let max_iterations = 2000u16;
    let img_side = 1000u32;
    // let mut mat = [[true; img_side]; img_side];
    let mut modd: u8 = 20;
    let cxmin = -1.5f32;
    let cxmax = 0.7f32;
    let cymin = -1f32;
    let cymax = 1f32;
    let scalex = (cxmax - cxmin) / img_side as f32;
    let scaley = (cymax - cymin) / img_side as f32;
 
    // Create a new ImgBuf
    // let mut imgbuf = image::ImageBuffer::new(img_side, img_side);
    let mut imgbuf = image::ImageBuffer::new(img_side, img_side);
    let mut ppm = PPM::new(img_side, img_side);
    // Calculate for each pixel
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let cx = cxmin + x as f32 * scalex;
        let cy = cymin + y as f32 * scaley;
        // println!("cx:{} cy:{}", cx, cy);
        let c = Complex::new(cx, cy);
        let mut z = Complex::new(0f32, 0f32);
 
        let mut i = 0;
        for t in 0..max_iterations {
            if z.norm() > 2.0 {
                ppm.set_pixel(x, y, RGB{r:0,g:0,b:0});
                break;
            }
            else if t+1 == max_iterations {
                ppm.set_pixel(x, y, RGB{r:255% (modd+24) , g: 255%(modd+11) , b:255%(modd+12) });
            }
            modd = rand::random();
            // println!("Modd is {}", modd);	
            match modd {
                0 => modd = 42,
                // 240..=300 => modd -=24,
                230..=255 => modd -=24,
                _ => modd=modd,
            }
            z = z * z + c;
            i = t;
        }
 
        *pixel = image::Luma([i as u8]);
    }
    
    // Save image
    let file_name = format!("_ppm_mandel_iter{}_res{}.ppm", max_iterations.to_string(), img_side.to_string());
    println!("{}", file_name);
    match ppm.write_file(&file_name) {
    	Ok(res) => println!("{:?}", res),
    	Err(_) =>(),
    }
    // let fout = &mut File::create("fractal.png").unwrap();
    // image::ImageLuma8(imgbuf).save(fout, image::PNG).unwrap();
}