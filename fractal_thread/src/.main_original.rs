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


fn rendersnowflakeside(p1x: f64, p1y: f64, p2x: f64, p2y: f64, n: i64, mut img: &mut RgbImage){
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
        rendersnowflakeside(p1x, p1y, mid1x, mid1y, n2, &mut img);
        rendersnowflakeside(mid2x, mid2y, p2x, p2y, n2, &mut img);
        rendersnowflakeside(mid1x, mid1y, heightx, heighty, n2, &mut img);
        rendersnowflakeside(heightx, heighty, mid2x, mid2y, n2, &mut img);
        //
    }
}

fn main() {
	let rezscale = 10;
    let mut img = RgbImage::new(640 * rezscale, 480 * rezscale);
    let rezscale = rezscale as f64;  // nao precisa mas do valor inteiro
    let nrec = 5;
    rendersnowflakeside(270.0, 211.13249, 320.0, 297.73503, nrec, &mut img);
    println!("one side done!");
    rendersnowflakeside(370.0, 211.13249, 270.0, 211.13249, nrec, &mut img);
    println!("one side done!");
    rendersnowflakeside(320.0, 297.73503, 370.0, 211.13249, nrec, &mut img);
    println!("one side done!");
    rendersnowflakeside(270.0 * rezscale, 211.13249 * rezscale, 320.0 * rezscale, 297.73503 * rezscale, nrec, &mut img);
    println!("one side done!");
    rendersnowflakeside(370.0 * rezscale, 211.13249 * rezscale, 270.0 * rezscale, 211.13249 * rezscale, nrec, &mut img);
    println!("one side done!");
    rendersnowflakeside(320.0 * rezscale, 297.73503 * rezscale, 370.0 * rezscale, 211.13249 * rezscale, nrec, &mut img);
    println!("one side done!");

    img.save("output.png").unwrap();
}