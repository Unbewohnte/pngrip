/*
The MIT License (MIT)

Copyright © 2022 Kasyanov Nikolay Alexeyevich (Unbewohnte)

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use std::io::{Read, Write};
use std::path::Path;

#[derive(Debug)]
struct Position {
    start: usize,
    end: usize,
}

// Reads data from specified start_index position,
// if valid png bytes were found - returns exact positions of an image
fn rip_png(data: &[u8], start_index: usize) -> Option<Position> {
    const PNG_IDENTIFIER: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0xD, 0xA, 0x1A, 0xA];
    const PNG_END_IDENTIFIER: [u8; 8] = [0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82];

    if data.len() < PNG_IDENTIFIER.len() + PNG_END_IDENTIFIER.len() ||
        start_index + PNG_IDENTIFIER.len() + PNG_END_IDENTIFIER.len() > data.len() {
        return None;
    }

    let mut position: Position = Position{
        start: usize::MAX,
        end: usize::MAX,
    };

    for i in start_index..data.len() {
        // start index
        if i < data.len() - PNG_IDENTIFIER.len() && position.start == usize::MAX {
            if data[i..i + PNG_IDENTIFIER.len()] == PNG_IDENTIFIER {
                position.start = i;
            }
        }

        // end index
        if i <= data.len() - PNG_END_IDENTIFIER.len() && position.end == usize::MAX {
            if data[i..i + PNG_END_IDENTIFIER.len()] == PNG_END_IDENTIFIER {
                position.end = i + PNG_END_IDENTIFIER.len();
            }
        }

        if position.start != usize::MAX && position.end != usize::MAX {
            break;
        }
    }

    if position.start == usize::MAX || position.end == usize::MAX || position.end <= position.start {
        return None;
    }

    return Some(position);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("pngrip v0.1.3\nUSAGE: pngrip [DESTINATION] [FILES]...");
        std::process::exit(0);
    }

    // handle DESTINATION argument
    let mut destination_path = Path::new(&args[1]);
    match destination_path.exists() {
        true => {
            if !destination_path.is_dir() {
                // destination exists, but is not a directory
                destination_path = Path::new(".");
                eprintln!("[ERROR] Provided destination path is not a directory ! Saving to current directory...");
            }
        }
        false => {
            // destination does not exist, create it
            match std::fs::create_dir_all(&destination_path) {
                Ok(_) => {}
                Err(e) => {
                    destination_path = Path::new(".");
                    eprintln!("[ERROR] Could not create destination directory: {}. Saving to current directory...", e);
                }
            }
        }
    }


    // go through all files and try to rip all PNGs
    for file_path_index in 0..args[2..].len() {
        let file_path: &Path = Path::new(&args[file_path_index]);

        print!("\n");

        if !file_path.exists() {
            println!("[ERROR] \"{}\" does not exist", file_path.display());
            continue;
        }

        // get file's metadata
        let file_metadata: std::fs::Metadata;
        match std::fs::metadata(file_path) {
            Ok(metadata) => {
                file_metadata = metadata;
            }

            Err(error) => {
                println!("[ERROR] Could not retrieve \"{}\"'s metadata: {}", file_path.display(), error);
                continue;
            }
        }

        // skip directories
        if file_metadata.is_dir() {
            println!("[INFO] Skipping directory \"{}\"...", file_path.display());
            continue;
        }

        println!("[INFO] Working with \"{}\"...", file_path.display());

        let mut file_contents: Vec<u8> = Vec::with_capacity(file_metadata.len() as usize);
        let mut file_handle: std::fs::File;
        match std::fs::File::open(file_path) {
            Ok(f_handle) => {
                file_handle = f_handle;
            }
            Err(error) => {
                println!("[ERROR] Could not open \"{}\": {}", file_path.display(), error);
                continue;
            }
        }

        match file_handle.read_to_end(&mut file_contents) {
            Ok(_) => {}
            Err(error) => {
                println!("[ERROR] Error reading \"{}\": {}", file_path.display(), error);
            }
        }

        let mut positions: Vec<Position> = Vec::new();
        let mut cursor_index: usize = 0;
        while (cursor_index as u64) < file_metadata.len() {
            match rip_png(&file_contents, cursor_index) {
                Some(pos) => {
                    cursor_index = pos.end;
                    positions.push(pos);
                }
                None => {
                    break;
                }
            }
        }

        let file_name: String;
        match file_path.file_name() {
            Some(fname) => {
                file_name = String::from(fname.to_string_lossy());
            }

            None => {
                eprintln!("[ERROR] Could not retrieve this file's name");
                continue;
            }
        }

        for position_index in 0..positions.len() {
            let output_file_name: String = format!("{}_{}.png", file_name, position_index);
            let mut output_file: std::fs::File;
            match std::fs::File::create(destination_path.join(&output_file_name)) {
                Ok(f) => {
                    output_file = f;
                }

                Err(error) => {
                    eprintln!("[ERROR] Error creating output file: {}", error);
                    continue;
                }
            }

            match output_file.write(&file_contents[positions[position_index].start..positions[position_index].end]) {
                Ok(_) => {
                    println!("[INFO] Outputted \"{}\"", output_file_name);
                }

                Err(error) => {
                    eprintln!("[ERROR] Could not write PNG to the output file: {}", error);
                    continue;
                }
            }
        }
    }

}