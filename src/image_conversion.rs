use std::{fs::OpenOptions, io::{BufWriter, Write}};

use image::ImageBuffer;
use indicatif::{MultiProgress, ProgressBar};

use crate::{get_progress_style, SEPARATOR, STOP_CODE};

//\\Decode the image into the file//\\
pub fn convert_img(input: &str) {
    //if the file is a png file
    if input.ends_with(".png") {
        let m = MultiProgress::new();
        let img = image::open(input).unwrap();
        let img: ImageBuffer<image::Rgba<u8>, Vec<u8>> = img.to_rgba8();
        let data = decode_img(img, m);
        let mut file_name = input.to_owned();
        file_name = file_name.split(SEPARATOR).collect::<Vec<&str>>().last().unwrap().to_owned().to_owned();
        file_name = file_name.split("{0}.").collect::<Vec<&str>>()[0].to_owned();
        std::fs::write(file_name, data).unwrap();
    }else {
        let dir = std::path::Path::new(input);
        let entries = std::fs::read_dir(dir).unwrap();
        //sort entries by the number in {}.png in the filename
        let mut entries = entries.map(|entry| entry.unwrap()).collect::<Vec<std::fs::DirEntry>>();
        if entries.len() > 1 { 
            entries.sort_by(|a, b| {
                let a = a.file_name().to_str().unwrap().split('{').collect::<Vec<&str>>()[1].to_owned();
                let b = b.file_name().to_str().unwrap().split('{').collect::<Vec<&str>>()[1].to_owned();
                let a = a.split('}').collect::<Vec<&str>>()[0].to_owned();
                let b = b.split('}').collect::<Vec<&str>>()[0].to_owned();
                let a = a.parse::<usize>().unwrap();
                let b = b.parse::<usize>().unwrap();
                a.cmp(&b)
            });
        }
        let m = MultiProgress::new();
        let style = get_progress_style();
        let pb = m.add(ProgressBar::new(entries.len() as u64));
        pb.set_style(style);
        let file_name = entries[0].file_name().to_str().unwrap().split("{0}.").collect::<Vec<&str>>()[0].to_owned();
        let file_name = file_name.split(SEPARATOR).collect::<Vec<&str>>().last().unwrap().to_owned();
        let out_file = OpenOptions::new().write(true).create(true).open(file_name).unwrap();
        let mut writer = BufWriter::new(out_file);
        for entry in entries {
            pb.set_message(format!("Decoding {}", entry.file_name().to_str().unwrap()));
            let path = entry.path();
            let img = image::open(path).unwrap();
            let img: ImageBuffer<image::Rgba<u8>, Vec<u8>> = img.to_rgba8();
            let data_chunk = decode_img(img,m.clone());
            //remove last 23 bytes from the data
            let data_chunk = &data_chunk[..data_chunk.len()-23];
            //data.extend(data_chunk);]
            writer.write_all(data_chunk).unwrap();
            pb.set_message(format!("Decoded {}", entry.file_name().to_str().unwrap()));
            pb.inc(1);
        }
        println!("Writing to: {:?}", file_name);
        //std::fs::write(file_name, data).unwrap();
        writer.flush().unwrap();
        pb.finish_with_message(format!("Decoded to: {:?}", file_name));
    }
}

//\\Decode the data//\\
fn decode_img(img: ImageBuffer<image::Rgba<u8>, Vec<u8>>,m: MultiProgress ) -> Vec<u8> {
    // Create a new vector of 4 u8s
    let mut data = Vec::new();
    let img_size = img.width() * img.height();
    let style = get_progress_style();
    let pb2 = m.add(ProgressBar::new(img_size as u64));
    pb2.set_style(style);
    // For each pixel in the image buffer get the rgba values and push them to the data vector
    for (_x, _y, pixel) in img.enumerate_pixels() {
        data.push([pixel[0], pixel[1], pixel[2], pixel[3]]);
        pb2.inc(1);
    }
    // Convert data into a vector of u8s
    let data = data.iter().flat_map(|pixel| pixel.iter().cloned()).collect::<Vec<u8>>();
    // Find the index of the last stop code at the end of the data
    let stop_index = data.iter().rposition(|&x| x == STOP_CODE).unwrap(); //could be a one liner
    // Remove the stop code and the extra bits
    let data = &data[..stop_index];
    pb2.set_message("Decoded");
    pb2.finish_and_clear();
    // Write the data back to the zip file
    data.to_vec()
}
    