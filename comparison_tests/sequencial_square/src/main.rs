extern crate image;
// extern crate lit; 

use std::time::{SystemTime};
use image::RgbImage;

use std::thread;
use std::sync::{Arc, Mutex};
// lit é o nome deste pacote; está definido no Cargo.toml
// use lit::koch;
// use lit::mandelbrot;

const REZSCALE : i32 = 6;
const GLOBTX : usize = (480 * REZSCALE) as usize;
const GLOBTY : usize = (480 * REZSCALE) as usize;

fn render_tsquare(p1x: f64, p1y: f64, side: f64, n: i64,  mut mat: &mut [[bool; GLOBTX]; GLOBTY]){
    let n2 = n - 1;
    let plx = p1x - (side/2.0);
    let ply = p1y - (side/2.0);
    for i in (plx as usize)..((plx as usize)+(side as usize)){
        for j in (ply as usize)..((ply as usize)+(side as usize)){
            mat[j][i]=true;
        }
    }
    if  n > 0 {
        render_tsquare(plx,ply, side / 2.0 ,n2,&mut mat);
        render_tsquare(plx+side,ply, side / 2.0 ,n2,&mut mat);
        render_tsquare(plx,ply+side, side / 2.0 ,n2,&mut mat);
        render_tsquare(plx+side,ply+side, side / 2.0 ,n2,&mut mat);
    }
}

fn render_tsquarepre(p1x: f64, p1y: f64, side: f64, n: i64, nth: i64, mut arc: &mut [[bool; GLOBTX]; GLOBTY]){
    let n2 = n - 1;
    let plx = p1x - (side/2.0);
    let ply = p1y - (side/2.0);
    for i in (plx as usize)..((plx as usize)+(side as usize)){
        for j in (ply as usize)..((ply as usize)+(side as usize)){
            arc[j][i]=true;
        }
    }
    if  n > 0 {
        if nth == 0 {
            render_tsquare(plx,ply, side / 2.0 ,n2,&mut arc);
            render_tsquare(plx+side,ply, side / 2.0 ,n2,&mut arc);
            render_tsquare(plx,ply+side, side / 2.0 ,n2,&mut arc);
            render_tsquare(plx+side,ply+side, side / 2.0 ,n2,&mut arc);
        }
        else{
            render_tsquarepre(plx,ply, side / 2.0 ,n2,nth-1,&mut arc);
            render_tsquarepre(plx+side,ply, side / 2.0 ,n2,nth-1,&mut arc);
            render_tsquarepre(plx,ply+side, side / 2.0 ,n2,nth-1,&mut arc);
            render_tsquarepre(plx+side,ply+side, side / 2.0 ,n2,nth-1,&mut arc);
        }
    }
}

fn tsquare() {
    let systime = SystemTime::now();
    let nrec = 16;  // NAO AUMMENTAR!
    let nrecth = 1;
    let mut img = RgbImage::new(GLOBTX as u32, GLOBTY as u32);
    println!("REZSCALE: {}", REZSCALE);
    // const REZSCALE_int: i32 = REZSCALE;
    // let REZSCALEf = REZSCALE as f64;  // nao precisa mas do valor inteiro
    println!("Recursoes: {}", nrec);
    let mut arc1 = [[false; GLOBTX]; GLOBTY];      // preparando para as threads
    render_tsquarepre((GLOBTX as f64)/2.0, (GLOBTY as f64)/2.0 , (GLOBTX as f64)/2.0 ,nrec,nrecth, &mut arc1);
    //let mut m1 = [[false; GLOBTX]; GLOBTY];
    //render_tsquare(((GLOBTX as f64)/2.0),((GLOBTY as f64)/2.0),((GLOBTX as f64)/2.0),nrec,&mut m1);
    for x in 0..GLOBTX {
        for y in 0..GLOBTY{
            if arc1[y][x]{
                img.get_pixel_mut(x as u32, y as u32).data = [0, 0, 0];
            }
            else{
                img.get_pixel_mut(x as u32, y as u32).data = [255, 255, 255];
            }
        }
    }
    println!("Vai escrever...");
    img.save(REZSCALE.to_string()+ "_"  + &nrec.to_string() + "_outputtsquare.png").unwrap();
    println!("Escreveu");
    let newtime = SystemTime::now();
    let since_the_epoch = newtime.duration_since(systime)
        .expect("Time went backwards");
    println!("{:?}", since_the_epoch);
}

fn main() {
    tsquare();
}
