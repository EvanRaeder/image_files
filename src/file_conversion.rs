// Remove the invalid statement
use image::ImageBuffer;
use indicatif::{MultiProgress, ProgressBar};

use std::fs::File;
use std::io::{BufReader, Read};


use crate::{get_progress_style, SEPARATOR, CHUNK_SIZE, STOP_CODE, MAX_THREADS};

//\\Encode the file into the image//\\
pub fn convert_file(in_file: &str) {
    // Get the file path
    let size = std::fs::metadata(in_file).unwrap().len();
    let file = std::path::Path::new(in_file);
    let file_name = file.file_name().unwrap().to_str().unwrap().to_owned();
    let dir_name = file.file_name().unwrap().to_str().unwrap().to_owned().replace(".", "_");
    std::fs::create_dir_all(&dir_name).unwrap();
    // Open the file for reading
    let mut file = BufReader::new(File::open(file).unwrap());
    let mut buffer = vec![0; CHUNK_SIZE];
    let mut i = 0;
    // Create a progress bar
    let style = get_progress_style();
    let m = MultiProgress::new();
    let pb = m.add(ProgressBar::new(size/CHUNK_SIZE as u64));
    pb.set_style(style);
    loop {
        // Read a chunk of the file
        let bytes_read = file.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        pb.set_message(format!("Encoding {}", file_name));
        // Process the chunk
        let chunk = buffer[..bytes_read].to_vec();
        chunks.push((chunk, i));
        if chunks.len() >= MAX_THREADS {
            chunks.into_par_iter().for_each(|(chunk, i)| {
                let file_name = format!("{}{}{}", dir_name, SEPARATOR, file_name);
                let file_name = file_name + "{" + &i.to_string() + "}" + ".png";
                let img = encode_data(chunk, m.clone());
                img.save(&file_name).unwrap();
            });
            chunks = Vec::new();

        }
        pb.inc(1);
        i += 1;
    }
    pb.finish_with_message(format!("Read {} chunks", chunks.len()));

    // complete the last ops
    chunks.into_par_iter().for_each(|(chunk, i)| {
        let file_name = format!("{}{}{}", dir_name, SEPARATOR, file_name);
        let file_name = file_name + "{" + &i.to_string() + "}" + ".png";
        let img = encode_data(chunk,m.clone());
        img.save(&file_name).unwrap();
        // Update the progress bar
        pb.set_message(format!("Saved {}", file_name));
        pb.inc(1);
        i += 1;
    }
    pb.finish_with_message(format!("Saved to: {}", dir_name));
}

//\\Encode the data into the image//\\
fn encode_data(mut data: Vec<u8>,m:MultiProgress) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    //add a binary stop code to the data
    data.push(STOP_CODE);
    //get data into vecs of 4 bytes
    let data = data.chunks(4).map(|chunk| {
        let mut byte = [0; 4];
        for (i, bit) in chunk.iter().enumerate() {
            byte[i] = *bit;
        }
        byte
    }).collect::<Vec<[u8; 4]>>();
    let length = f64::sqrt(data.len() as f64);
    let width = f64::ceil(data.len() as f64 / length);
    assert!(data.len() <= (length * width) as usize);// IMPORTANT SANITY CHECK
    let img = image::DynamicImage::new_rgb8(length as u32, width as u32);
    //create a new image buffer
    let style = get_progress_style();
    let pb2 = m.add(ProgressBar::new(data.len() as u64));
    pb2.set_style(style);
    let mut img: ImageBuffer<image::Rgba<u8>, Vec<u8>> = img.to_rgba8();
    //for each pixel in the image buffer set values of rgba to the four u8s
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let index = (x + y * length as u32) as usize;
        if index < data.len() {
            pixel[0] = data[index][0];
            pixel[1] = data[index][1];
            pixel[2] = data[index][2];
            pixel[3] = data[index][3];
            pb2.inc(1);
        }
    }
    pb2.finish_and_clear();
    img
}
