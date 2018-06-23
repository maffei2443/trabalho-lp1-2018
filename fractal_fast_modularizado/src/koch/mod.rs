extern crate image;
use std::time::{SystemTime};
use self::image::RgbImage;
use std::thread;
// use std::sync::mpsc;  // mpsc: multiple producer, single consumer
use std::sync::{Arc, Mutex};
const REZSCALE : i32 = 1;
const GLOBX : usize = (640 * REZSCALE) as usize;
const GLOBY : usize = (480 * REZSCALE) as usize;



fn draw_line(mat: &mut [[bool; GLOBX]; GLOBY], mut x0: i64, mut y0: i64, x1: i64, y1: i64) {

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
        mat[y0 as usize][x0 as usize]=true;

        // Check end condition
        if x0 == x1 && y0 == y1 { break };

        // Store old error
        err2 = 2 * err;

        // Adjust error and start position
        if err2 > -dx { err -= dy; x0 += sx; }
        if err2 < dy { err += dx; y0 += sy; }
    }
}

fn render_snow_flake_side(p1x: f64, p1y: f64, p2x: f64, p2y: f64, n: i64,  mat: &mut [[bool; GLOBX]; GLOBY]){
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

        render_snow_flake_side(p1x, p1y, mid1x, mid1y, n2, mat);
        render_snow_flake_side(mid2x, mid2y, p2x, p2y, n2, mat);
        render_snow_flake_side(mid1x, mid1y, heightx, heighty, n2, mat);
        render_snow_flake_side(heightx, heighty, mid2x, mid2y, n2, mat);
        //
    }
}
#[allow(dead_code)]
fn render_snow_flake_side_pre(p1x: f64, p1y: f64, p2x: f64, p2y: f64, n: i64, arc: Arc<Mutex<[[bool; GLOBX]; GLOBY]>>){
    let mut mat = &mut *arc.lock().unwrap();
    render_snow_flake_side(p1x, p1y, p2x, p2y, n, &mut mat);
}

pub fn koch() -> [ [(i32, i32, i32); GLOBX]; GLOBY ]{
    let systime = SystemTime::now();
    let nrec = 15;  // NAO AUMMENTAR!
    // let mut img = RgbImage::new(GLOBX as u32, GLOBY as u32);
    let mut img = [ [(0, 0, 0); GLOBX]; GLOBY ];
    
    println!("rezscale: {}", REZSCALE);
    let rezscalef = REZSCALE as f64;  // nao precisa mas do valor inteiro
    println!("Recursoes: {}", nrec);
    let arc1 = Arc::new( Mutex::new( [[false; GLOBX]; GLOBY] ) );      // preparando para as threads
    let arc2 = Arc::new( Mutex::new( [[false; GLOBX]; GLOBY] ) );      // preparando para as threads
    let arc3 = Arc::new( Mutex::new( [[false; GLOBX]; GLOBY] ) );      // preparando para as threads

    let to_pass = arc1.clone();
    let h1 = thread::spawn(move || {
        render_snow_flake_side_pre(270.0 * rezscalef, 211.13249 * rezscalef, 320.0 * rezscalef, 297.73503 * rezscalef, nrec, to_pass.clone());
    });

    let to_pass = arc2.clone();
            
    let h2 = thread::spawn(move || {
        render_snow_flake_side_pre(370.0 * rezscalef, 211.13249 * rezscalef, 270.0 * rezscalef, 211.13249 * rezscalef, nrec, to_pass.clone() );
    });
            
    let to_pass = arc3.clone();

    let h3 = thread::spawn(move || {
        render_snow_flake_side_pre(320.0 * rezscalef, 297.73503 * rezscalef, 370.0 * rezscalef, 211.13249 * rezscalef, nrec, to_pass.clone());
    });

            
    h1.join().unwrap();         println!("h1 done!");
            
    h2.join().unwrap();         println!("h2 done!");
            
    h3.join().unwrap();         println!("h3 done!");

    let m1 = &*arc1.lock().unwrap();
    let m2 = &*arc2.lock().unwrap();
    let m3 = &*arc3.lock().unwrap();

    for x in 0..GLOBX {
        for y in 0..GLOBY{
            if m1[y][x] || m2[y][x] || m3[y][x]{
                img[x][y] = (255, 255, 255);
            }
        }
    }

    println!("Vai escrever...");
    // img.save(rezscale_int.to_string()+ "_"  + &nrec.to_string() + "_output.png").unwrap();
    
    println!("Escreveu");
    let newtime = SystemTime::now();
    let since_the_epoch = newtime.duration_since(systime)
        .expect("Time went backwards");
    println!("{:?}", since_the_epoch);
    img

}

pub fn gen_koch() {
    let systime = SystemTime::now();
    let nrec = 15;  // NAO AUMMENTAR!
    let mut img = RgbImage::new(GLOBX as u32, GLOBY as u32);
    // let mut img = [ [(0, 0, 0); GLOBX]; GLOBY ];
    
    println!("rezscale: {}", REZSCALE);
    let rezscalef = REZSCALE as f64;  // nao precisa mas do valor inteiro
    println!("Recursoes: {}", nrec);
    let arc1 = Arc::new( Mutex::new( [[false; GLOBX]; GLOBY] ) );      // preparando para as threads
    let arc2 = Arc::new( Mutex::new( [[false; GLOBX]; GLOBY] ) );      // preparando para as threads
    let arc3 = Arc::new( Mutex::new( [[false; GLOBX]; GLOBY] ) );      // preparando para as threads

    let to_pass = arc1.clone();
    let h1 = thread::spawn(move || {
        render_snow_flake_side_pre(270.0 * rezscalef, 211.13249 * rezscalef, 320.0 * rezscalef, 297.73503 * rezscalef, nrec, to_pass.clone());
    });

    let to_pass = arc2.clone();
            
    let h2 = thread::spawn(move || {
        render_snow_flake_side_pre(370.0 * rezscalef, 211.13249 * rezscalef, 270.0 * rezscalef, 211.13249 * rezscalef, nrec, to_pass.clone() );
    });
            
    let to_pass = arc3.clone();

    let h3 = thread::spawn(move || {
        render_snow_flake_side_pre(320.0 * rezscalef, 297.73503 * rezscalef, 370.0 * rezscalef, 211.13249 * rezscalef, nrec, to_pass.clone());
    });

            
    h1.join().unwrap();         println!("h1 done!");
            
    h2.join().unwrap();         println!("h2 done!");
            
    h3.join().unwrap();         println!("h3 done!");

    let m1 = &*arc1.lock().unwrap();
    let m2 = &*arc2.lock().unwrap();
    let m3 = &*arc3.lock().unwrap();

    for x in 0..GLOBX {
        for y in 0..GLOBY{
            if m1[y][x] || m2[y][x] || m3[y][x]{
                img.get_pixel_mut(x as u32, y as u32).data = [255, 255, 255];
            }
        }
    }

    println!("Vai escrever...");
    // img.save(rezscale_int.to_string()+ "_"  + &nrec.to_string() + "_output.png").unwrap();
    
    println!("Escreveu");
    let newtime = SystemTime::now();
    let since_the_epoch = newtime.duration_since(systime)
        .expect("Time went backwards");
    println!("{:?}", since_the_epoch);

}