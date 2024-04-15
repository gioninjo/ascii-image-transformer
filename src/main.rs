use std::str::FromStr;
use std::env;



use image::{self, DynamicImage, GenericImageView};

const DESIRED_HEIGHT: u32 = 64;
const DESIRED_WIDTH: u32 = 64;

fn main() {

    let img_scale: Vec<char> = String::from_str(" .:-=+*#%@").unwrap_or(String::new()).chars().collect();

    println!("{}", img_scale.len());

    let img = image::open("/home/gioninjo/projects/rust/ascii-image-transformer/src/res/cat-war.jpg").expect("Failed to open image");
    
    let (width, height) = img.dimensions();
    let chunk_width = width / DESIRED_WIDTH;
    let chunk_height = height / DESIRED_HEIGHT;
    let resized_img: DynamicImage = DynamicImage::resize(&img, chunk_width * DESIRED_WIDTH, chunk_height * DESIRED_HEIGHT, image::imageops::FilterType::Lanczos3);

    let pixels: image::Pixels<'_, image::DynamicImage> = resized_img.pixels();

    // let chunks = createChunks(pixels);


    let mut grayshades: Vec<u32> = vec![];

    for (_, _, image::Rgba(p)) in pixels {
        let [r, g, b, a] = p;
        let grayshade = ((r as u16) + (g as u16) + (b as u16)) / 3;
        grayshades.push(grayshade as u32);
    }

    let (width, height) = resized_img.dimensions();

    let mut chunks: Vec<char> = vec![];

    for line in 0..DESIRED_HEIGHT {
        for column in 0..DESIRED_WIDTH {
            let start_x: u32 = column * chunk_width;
            let start_y: u32 = line * chunk_height;
            let end_x: u32 = start_x + chunk_width + 1;
            let end_y: u32 = start_y + chunk_height + 1;
            
            let mut chunk_shades_sum: u32 = 0;

            let default = 0;

            for pixel_y in start_y..end_y {
                for pixel_x in start_x..end_x {
                    let mut pixel_position = (pixel_y*width + pixel_x) as usize;
                    if pixel_position >= grayshades.len() {
                        pixel_position = grayshades.len() - 1;
                    }
                    chunk_shades_sum = chunk_shades_sum + grayshades[pixel_position];
                }
            }

            let chunk_shade = chunk_shades_sum / (chunk_height * chunk_width);
            if chunk_shade <= 256 {
                let mut char_index: usize = (chunk_shade * (img_scale.len() as u32) / 256) as usize;
                if char_index >= img_scale.len() {
                    char_index = img_scale.len() - 1;
                }
                let associated_char = img_scale[char_index];
                chunks.push(associated_char);
            }
        }
        chunks.push('\n');
    }


    for char in chunks.iter() {
        print!("{} ", char)
    }
    

}


// fn createChunks(pixels: &Pixels, desired_height: &i32, desired_width: &i32) -> void {

    

// }