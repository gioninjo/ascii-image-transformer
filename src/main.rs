use std::str::FromStr;

use image::{open, Pixels, imageops, Rgba, DynamicImage, GenericImageView};

const DESIRED_HEIGHT: u32 = 64;
const DESIRED_WIDTH: u32 = 128;

fn main() -> Result<(), &'static str>{

    let img_scale: Vec<char> = String::from_str(" .:-=+*#%@").unwrap_or(String::new()).chars().collect();

    println!("{}", img_scale.len());

    let img = open("src/res/monster&co_meme.jpg").expect("Failed to open image");
    
    let (width, height) = img.dimensions();
    let chunk_width = width / DESIRED_WIDTH;
    let chunk_height = height / DESIRED_HEIGHT;

    // round up the image
    let resized_img: DynamicImage = DynamicImage::resize(&img, chunk_width * DESIRED_WIDTH, chunk_height * DESIRED_HEIGHT, imageops::FilterType::Lanczos3);

    let pixels: Pixels<'_, DynamicImage> = resized_img.pixels();

    let mut gray_shades: Vec<u32> = vec![];

    for (_, _, Rgba(p)) in pixels {
        let [r, g, b, _a] = p;
        let grayshade = ((r as u16) + (g as u16) + (b as u16)) / 3;
        gray_shades.push(grayshade as u32);
    }

    let dim = resized_img.dimensions();

    let ascii_string: String;

    match create_ascii_string(&gray_shades, &img_scale, &chunk_width, &chunk_height, &dim) {
        Ok(result) => ascii_string = result,
        Err(e) => return Err(e)
    }


    print!("{}", ascii_string);
    

    Ok(())

}


fn create_ascii_string(gray_shades: &Vec<u32>, img_scale: &Vec<char>,chunk_width: &u32, chunk_height: &u32, (width, _height): &(u32, u32)) -> Result<String, &'static str> {
    let mut ascii_string = String::new();
    for line in 0..DESIRED_HEIGHT {
        for column in 0..DESIRED_WIDTH {
            let start_x: u32 = column * chunk_width;
            let start_y: u32 = line * chunk_height;
            let end_x: u32 = start_x + chunk_width;
            let end_y: u32 = start_y + chunk_height;

            let chunk_shade = calculate_chunk_shade(gray_shades, width, chunk_width, chunk_height, start_x, end_x, start_y, end_y);
            match calculate_associated_char(img_scale, chunk_shade) {
                Ok(shade) => ascii_string.push(shade),
                Err(e) => return Err(e)
            }
        }
        ascii_string.push('\n');
    }

    Ok(ascii_string)
}


fn calculate_chunk_shade(gray_shades: &Vec<u32>, width: &u32, chunk_width: &u32, chunk_height: &u32, start_x: u32, end_x: u32, start_y: u32, end_y: u32) -> u32 {
    let mut chunk_shades_sum: u32 = 0;

    for pixel_y in start_y..end_y {
        for pixel_x in start_x..end_x {
            let mut pixel_position = (pixel_y*width + pixel_x) as usize;
            if pixel_position >= gray_shades.len() {
                pixel_position = gray_shades.len() - 1;
            }
            chunk_shades_sum = chunk_shades_sum + gray_shades[pixel_position];
        }
    }

    // calculate average shade in the chunk
    chunk_shades_sum / (chunk_height * chunk_width)
}

fn calculate_associated_char(img_scale: &Vec<char>, chunk_shade: u32) -> Result<char, &'static str> {
    if chunk_shade <= 256 {
        let mut char_index: usize = (chunk_shade * (img_scale.len() as u32) / 256) as usize;
        if char_index >= img_scale.len() {
            char_index = img_scale.len() - 1;
        }
        match img_scale.get(char_index) {
            Some(res) => return Ok(res.clone()),
            None => return Err("Selected char for shade does not exists")
        }
    }

    println!("shade ---> {}", chunk_shade );

    Err("Chunk shade number > 256!!!")
}