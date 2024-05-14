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

use image::open;
use asciiforger::images::asciify;
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

    match asciify(desired_width, desired_height, &img_scale, &img) {
        Ok(ascii_string) => println!("{}", ascii_string),
        Err(e) => return Err(e),
    };

    Ok(())
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
