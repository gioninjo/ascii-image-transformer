
pub fn asciify_u8(
    desired_width: u32,
    desired_height: u32,
    img_scale: Vec<char>,
    img: &[u8],
    img_width: u32,
    img_height: u32,
) -> Result<String, &'static str> {
    let chunk_width = img_width / desired_width;
    let chunk_height: u32 = img_height / desired_height;

    // // round up the image
    // let resized_img: DynamicImage = DynamicImage::resize(
    //     &img,
    //     chunk_width * desired_width,
    //     chunk_height * desired_height,
    //     imageops::FilterType::Lanczos3,
    // );

    // let pixels: Pixels<'_, DynamicImage> = resized_img.pixels();

    let mut gray_shades: Vec<u32> = vec![];

    for row in 0..img_height {
        for pixel_start in 0..(img_width / 4) {
            let mut shade: u32 = 0;
            for rgb in 0..3 {
                let position = row * img_width + pixel_start * 4 + rgb;
                print!(" {}", position);
                shade += img[position as usize] as u32;
            }
            gray_shades.push(shade / 3);
        }
    }

    println!("{}", gray_shades.len());


    // for (_, _, Rgba(p)) in pixels {
    //     let [r, g, b, _a] = p;
    //     let grayshade = ((r as u16) + (g as u16) + (b as u16)) / 3;
    //     gray_shades.push(grayshade as u32);
    // }

    // let dim = resized_img.dimensions();

    match create_ascii_string(
        desired_width,
        desired_height,
        &gray_shades,
        &img_scale,
        &chunk_width,
        &chunk_height,
        &(img_width, img_height),
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