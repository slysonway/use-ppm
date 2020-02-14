extern crate ppm;
use std::path::Path;

fn main() {
    let p = ppm::Pixel::new(255, 255, 255);
    let p1 = ppm::Pixel::new(30, 128, 255);
    let mut p2 = ppm::Pixel::new(100, 50, 75);

    println!("p: {}", p.display());


    println!("p1: {}", p1.display());

    println!("p2: {}", p2.display());

    p2 = !p2;
    println!("Reverted p2: {}", p2.display());
    
    if p1.eq(&p2) {
        println!("p1 == p2");
    } else {
        println!("p1 != p2");
    }

    let p3 = ppm::Pixel::new(1, 1, 1);
    if p1.eq(&p3) {
        println!("p1 == p3");
    } else {
        println!("p1 != p3");
    }

    println!("p en grayscale : {}", p.grayscale());
    println!("p1 en grayscale : {}", p1.grayscale());
    println!("p2 en grayscale : {}", p2.grayscale());

    let open_path = Path::new("sample.ppm");
    let image = ppm::new_with_file(open_path);
    match image {
        Some(image) => {
            println!("Picture print : ");
            println!("widht : {}, height : {}", image.width(), image.height());
            for bp in image.buffer() {
                println!("{}", bp.display());
            }
        },
        None => println!("No Image !")
    }
    
    let image2 = ppm::new_with_file(open_path);
    println!("Inverted picture print : ");
    match image2 {
        Some(image2) => { 
            let inverted_image = ppm::Image::invert(&image2);
            println!("widht : {}, height : {}", inverted_image.width(), inverted_image.height());
            for p in inverted_image.buffer() {
               println!("{}", p.display());
            }
        },
        None => println!("No Image !")
    }


    let mut colors: Vec<ppm::Pixel> = Vec::new();
    let w:u32 = 500;
    let h:u32 = 500;
    for _i in 0..(w*h) {
        let c: ppm::Pixel = ppm::Pixel::new(255, 0, 255);
        colors.push(c);
    }
    let image3: ppm::Image = ppm::Image::new(colors, w, h); 

    println!("Write Image in a file");
    let save_path = Path::new("sample2.ppm");
    ppm::Image::save(&image3, save_path);
    println!("File written !");

    println!("Write the inverted Image in a file");
    let image4: ppm::Image = ppm::Image::invert(&image3);
    let save_path = Path::new("sample3.ppm");
    ppm::Image::save(&image4, save_path);
    println!("Inverted file written !");

    println!("Write the grayscale Image in a file");
    let image5: ppm::Image = ppm::Image::grayscale(&image4);
    let save_path = Path::new("example4.ppm");
    ppm::Image::save(&image5, save_path);
    println!("Grayscale file written !");
}
