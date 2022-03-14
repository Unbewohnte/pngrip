/*
The MIT License (MIT)

Copyright © 2022 Kasyanov Nikolay Alexeyevich (Unbewohnte)

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use std::io::{Read, Write};
use std::path::Path;

/// extracts png images from file that path points to, saving every image to destination directory.
/// If an error occurs - returns immediately.
fn rip_png(path: &Path, destination: &Path) {
    let png_identifier: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
    let iend_identifier: [u8; 4] = [73, 69, 78, 68];

    let filename;
    match path.file_name() {
        Some(name) => {
            filename = String::from(name.to_string_lossy());
        }
        None => {
            eprintln!("[ERROR] Could not get filename from \"{}\"", path.display());
            return;
        }
    }

    println!("[INFO] Ripping PNGs from \"{}\"...", filename);

    let mut file;
    match std::fs::File::open(path) {
        Ok(f) => {
            file = f;
        }

        Err(e) => {
            eprintln!("[ERROR] On opening \"{}\": {}", filename, e);
            return;
        }
    }

    let mut file_bytes: Vec<u8> = Vec::new();
    match file.read_to_end(&mut file_bytes) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("[ERROR] On reading \"{}\": {}", filename, e);
            return;
        }
    }

    let mut image_count: u64 = 0;

    let mut start_pos: usize = 0;
    let mut last_pos: usize = 0;
    let mut end_pos: usize = 0;
    for i in 0..file_bytes.len()-iend_identifier.len() {
        if i < file_bytes.len() - png_identifier.len() - iend_identifier.len()
            && file_bytes[i..i + png_identifier.len()] == png_identifier {
            start_pos = i;
        }

        if file_bytes[i..i + iend_identifier.len()] == iend_identifier {
            end_pos = i + iend_identifier.len();
        }

        if start_pos < end_pos && start_pos != last_pos {
            last_pos = start_pos;
            image_count += 1;
            // println!("[INFO] Found PNG at {}->{} ({} bytes)", start_pos, end_pos, end_pos - start_pos);

            let mut ripped_image_file;
            let ripped_image_filename = format!("{}_{}.png", filename, image_count);
            match std::fs::File::create(destination.join(&ripped_image_filename)) {
                Ok(f) => {
                    ripped_image_file = f;
                }
                Err(e) => {
                    eprintln!("[ERROR] On creating \"{}\": {}", &ripped_image_filename, e);
                    return;
                }
            }

            match ripped_image_file.write_all(&mut file_bytes[start_pos..end_pos]) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("[ERROR] On writing to \"{}\": {}", ripped_image_filename, e);
                    return;
                }
            }
        }
    }


    println!("[INFO] Ripped {} images from \"{}\" in total", image_count, filename);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("USAGE: pngrip [DESTINATION] [FILES]...");
        std::process::exit(0);
    }

    // handle DESTINATION argument
    let mut destination = Path::new(&args[1]);
    match destination.exists() {
        true => {
            if !destination.is_dir() {
                // destination exists, but is not a directory
                destination = Path::new(".");
                eprintln!("[ERROR] Provided destination path is not a directory ! Saving to current directory...");
            }
        }
        false => {
            // destination does not exist, create it
            match std::fs::create_dir_all(&destination) {
                Ok(_) => {}
                Err(e) => {
                    destination = Path::new(".");
                    eprintln!("[ERROR] Could not create destination directory: {}. Saving to current directory...", e);
                }
            }
        }
    }


    // go through all files and try to rip all PNGs
    for file_to_check in &args[2..] {
        let path = Path::new(file_to_check);
        if !path.exists() {
            eprintln!("[ERROR] \"{}\" does not exist", file_to_check);
            continue;
        }

        rip_png(path, destination);
    }

}