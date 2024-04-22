//  SPDX-License-Identifier: GPL-3.0-only
/*  Build tool with support for git tags, wrapping make.
 *  Copyright (C) 2024  gioninjo
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, version 3 of the License.
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use image::{imageops, open, DynamicImage, GenericImageView, Pixels, Rgba};
use std::env;
use std::str::FromStr;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Missing argument.");
        print_usage(&args[0]);

        return Err("exit now");
    }

    let desired_width: u32;
    let desired_height: u32;

    match parse_pair(&args[2], 'x') {
        Some((width, height)) => {
            desired_height = height;
            desired_width = width;
        }
        None => {
            println!("Invalid desired dimensions!!");
            print_usage(&args[0]);
            return Err("exit now");
        }
    }
    let mut scale = *b"@%#*+=-:. "; 

    let file_path: &String = &args[1];
    let img = open(file_path).expect("Failed to open image at {file_path}");

    if args.len() == 4 {
        match args[3].as_str() {
            "--dark" => scale.reverse(),
            _ => {
                print_usage(&args[0]);
                return Err("unknown flag used!!");
            }
        }
    }

    let img_scale: Vec<char> = String::from_utf8(Vec::from(scale))
        .unwrap_or(String::new())
        .chars()
        .collect();

    match asciify(desired_width, desired_height, img_scale, img) {
        Ok(ascii_string) => println!("{}", ascii_string),
        Err(e) => return Err(e),
    };

    Ok(())
}

fn asciify(
    desired_width: u32,
    desired_height: u32,
    img_scale: Vec<char>,
    img: DynamicImage
) -> Result<String, &'static str> {
    let (width, height) = img.dimensions();
    let chunk_width = width / desired_width;
    let chunk_height = height / desired_height;

    // round up the image
    let resized_img: DynamicImage = DynamicImage::resize(
        &img,
        chunk_width * desired_width,
        chunk_height * desired_height,
        imageops::FilterType::Lanczos3,
    );

    let pixels: Pixels<'_, DynamicImage> = resized_img.pixels();

    let mut gray_shades: Vec<u32> = vec![];

    for (_, _, Rgba(p)) in pixels {
        let [r, g, b, _a] = p;
        let grayshade = ((r as u16) + (g as u16) + (b as u16)) / 3;
        gray_shades.push(grayshade as u32);
    }

    let dim = resized_img.dimensions();

    match create_ascii_string(
        desired_width,
        desired_height,
        &gray_shades,
        &img_scale,
        &chunk_width,
        &chunk_height,
        &dim,
    ) {
        Ok(result) => return Ok(result),
        Err(e) => return Err(e),
    }
}

fn create_ascii_string(
    desired_width: u32,
    desired_height: u32,
    gray_shades: &Vec<u32>,
    img_scale: &Vec<char>,
    chunk_width: &u32,
    chunk_height: &u32,
    (width, _height): &(u32, u32),
) -> Result<String, &'static str> {
    let mut ascii_string = String::new();
    for line in 0..desired_height {
        for column in 0..desired_width {
            let start_x: u32 = column * chunk_width;
            let start_y: u32 = line * chunk_height;
            let end_x: u32 = start_x + chunk_width;
            let end_y: u32 = start_y + chunk_height;

            let chunk_shade = calculate_chunk_shade(
                gray_shades,
                width,
                chunk_width,
                chunk_height,
                start_x,
                end_x,
                start_y,
                end_y,
            );
            match calculate_associated_char(img_scale, chunk_shade) {
                Ok(shade) => ascii_string.push(shade),
                Err(e) => return Err(e),
            }
        }
        ascii_string.push('\n');
    }

    Ok(ascii_string)
}

fn calculate_chunk_shade(
    gray_shades: &Vec<u32>,
    width: &u32,
    chunk_width: &u32,
    chunk_height: &u32,
    start_x: u32,
    end_x: u32,
    start_y: u32,
    end_y: u32,
) -> u32 {
    let mut chunk_shades_sum: u32 = 0;

    for pixel_y in start_y..end_y {
        for pixel_x in start_x..end_x {
            let mut pixel_position = (pixel_y * width + pixel_x) as usize;
            if pixel_position >= gray_shades.len() {
                pixel_position = gray_shades.len() - 1;
            }
            chunk_shades_sum = chunk_shades_sum + gray_shades[pixel_position];
        }
    }

    // calculate average shade in the chunk
    chunk_shades_sum / (chunk_height * chunk_width)
}

fn calculate_associated_char(
    img_scale: &Vec<char>,
    chunk_shade: u32,
) -> Result<char, &'static str> {
    if chunk_shade <= 256 {
        let mut char_index: usize = (chunk_shade * (img_scale.len() as u32) / 256) as usize;
        if char_index >= img_scale.len() {
            char_index = img_scale.len() - 1;
        }
        match img_scale.get(char_index) {
            Some(res) => return Ok(res.clone()),
            None => return Err("Selected char for shade does not exists"),
        }
    }

    println!("shade ---> {}", chunk_shade);

    Err("Chunk shade number > 256!!!")
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}


fn print_usage(arg_zero: &str) -> () {
    println!("Usage: {} <image path> <WIDTHxHEIGHT> (OPTIONAL)--dark", arg_zero)
}
