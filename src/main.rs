use std::{f64, fs::{File, OpenOptions}, io::{BufReader, BufWriter, Read, Write}};
use image::ImageBuffer;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

#[cfg(unix)]
fn separator() -> &'static str {
    "/"
}
#[cfg(windows)]
fn separator() -> &'static str {
    "\\"
}

fn get_progress_style() -> ProgressStyle {
    let style_result = ProgressStyle::default_bar()
        .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({eta})");
    match style_result {
        Ok(style) => style,
        Err(err) => {
            eprintln!("Error creating progress style: {}", err);
            ProgressStyle::default_bar()
        }
    }
}

//\\Get the correct image size for the file//\\
fn file_size(bytes: f64) -> (f64, f64) {
    let size = bytes/4.0;
    let length = f64::ceil(f64::sqrt(size));
    let width = f64::ceil(f64::sqrt(size));
    (length, width)
}
//\\Encode the file into the image//\\
fn convert_file(in_file: &str) {
    let chunk_size = 99900000;
    // Get the file path
    let size = std::fs::metadata(in_file).unwrap().len();
    let file = std::path::Path::new(in_file);
    let file_name = file.file_name().unwrap().to_str().unwrap().to_owned();
    let dir_name = file.file_name().unwrap().to_str().unwrap().to_owned().replace(".", "_");

    std::fs::create_dir_all(&dir_name).unwrap();
    // Open the file for reading
    let mut file = BufReader::new(File::open(file).unwrap());
    let mut buffer = vec![0; chunk_size];
    let mut i = 0;
    let style = get_progress_style();
    let m = MultiProgress::new();
    let pb = m.add(ProgressBar::new(size/chunk_size as u64));
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
        let file_name = format!("{}{}{}", dir_name, separator(), file_name);
        let file_name = file_name + "{" + &i.to_string() + "}" + ".png";
        let img = encode_data(chunk,m.clone());
        img.save(&file_name).unwrap();
        pb.set_message(format!("Saved {}", file_name));
        pb.inc(1);
        i += 1;
    }
    pb.finish_with_message(format!("Saved to: {}", dir_name));
}
fn encode_data(mut data: Vec<u8>,m:MultiProgress) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    //get the length of the data in bits
    let length_bit = data.len() as f64;
    let (length, width) = file_size(length_bit as f64);
    //add a binary stop code to the data
    data.push(0b11111111);
    //get data into vecs of 4 bytes
    let data = data.chunks(4).map(|chunk| {
        let mut byte = [0; 4];
        for (i, bit) in chunk.iter().enumerate() {
            byte[i] = *bit;
        }
        byte
    }).collect::<Vec<[u8; 4]>>();
    assert!(data.len() <= (length * width) as usize);// IMPORTANT SANITY CHECK
    let length = f64::sqrt(data.len() as f64);
    let width = f64::ceil(data.len() as f64 / length);
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
//\\Decode the image into the file//\\
fn convert_img(input: &str) {
    //if the file is a png file
    if input.ends_with(".png") {
        let m = MultiProgress::new();
        let img = image::open(input).unwrap();
        let img: ImageBuffer<image::Rgba<u8>, Vec<u8>> = img.to_rgba8();
        let data = decode_img(img, m);
        let mut file_name = input.to_owned();
        file_name = file_name.split(separator()).collect::<Vec<&str>>().last().unwrap().to_owned().to_owned();
        file_name = file_name.split("{0}.").collect::<Vec<&str>>()[0].to_owned();
        std::fs::write(file_name, data).unwrap();
        return;
    }
    //if the file is a directory
    else {
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
        let file_name = file_name.split(separator()).collect::<Vec<&str>>().last().unwrap().to_owned();
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
fn decode_img(img: ImageBuffer<image::Rgba<u8>, Vec<u8>>,m: MultiProgress ) -> Vec<u8> {
    //create a new vector of 4 u8s
    let mut data = Vec::new();
    let img_size = img.width() * img.height();
    let style = get_progress_style();
    let pb2 = m.add(ProgressBar::new(img_size as u64));
    pb2.set_style(style);
    //for each pixel in the image buffer get the rgba values and push them to the data vector
    for (_x, _y, pixel) in img.enumerate_pixels() {
        data.push([pixel[0], pixel[1], pixel[2], pixel[3]]);
        pb2.inc(1);
    }
    //convert data into a vector of u8s
    let data = data.iter().flat_map(|pixel| pixel.iter().cloned()).collect::<Vec<u8>>();
    //find the index of the last stop code at the end of the data
    let stop_index = data.iter().rposition(|&x| x == 0b11111111).unwrap(); //could be a one liner
    //remove the stop code and the extra bits
    let data = &data[..stop_index];
    pb2.set_message("Decoded");
    pb2.finish_and_clear();
    //write the data back to the zip file
    data.to_vec()
}
    
fn main() {
    //check if unix or windows
    //if there are args provided if -e encode else if -d decode the given filename
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 {
        if args[1] == "-e" {
            if args[3] != "" {
                std::env::set_current_dir(args[3].trim()).unwrap();
            }
            convert_file(&args[2]);
        } else if args[1] == "-d" {
            if args[3] != "" {
                std::env::set_current_dir(args[3].trim()).unwrap();
            }
            convert_img(&args[2]);
        } else {
            //check if arg[1] is an existing directory
            let dir = std::path::Path::new(&args[1]);
            if dir.is_dir() {
                if args.len() == 3 {
                    std::env::set_current_dir(args[2].trim()).unwrap();
                }
                convert_img(&args[1]);
            }else if dir.is_file() {
                if args.len() == 3 {
                    std::env::set_current_dir(args[2].trim()).unwrap();
                }
                convert_file(&args[1]);
            }else {
                println!("Invalid directory or file");
                println!("Usage: image_files.exe -e <filename> or image_files.exe -d <filename> or specify a working directory by using image_files.exe -d/e <filename> <directory>");
            }
        }
    }
    else {
        //start with UI
        println!("Welcome to the image file encoder/decoder");
        println!("Choose (e)ncode or (d)ecode");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();
        if choice == "e" {
            println!("Enter path to the filename to encode");
            let mut filename = String::new();
            std::io::stdin().read_line(&mut filename).unwrap();
            let filename = filename.trim().replace('"', "").replace("'", "");
            println!("Enter a working directory leave blank for current directory");
            let mut dir = String::new();
            std::io::stdin().read_line(&mut dir).unwrap();
            let dir = dir.trim().replace('"', "").replace("'", "");
            if dir != "" {
                std::env::set_current_dir(dir).unwrap();
            }
            convert_file(&filename);
            return;
        } else if choice == "d" {
            println!("Enter the path to filename to decode");
            let mut filename = String::new();
            std::io::stdin().read_line(&mut filename).unwrap();
            let filename = filename.trim().replace('"', "").replace("'", "");
            println!("Enter a working directory leave blank for current directory");
            let mut dir = String::new();
            std::io::stdin().read_line(&mut dir).unwrap();  
            let dir = dir.trim().replace('"', "").replace("'", "");
            if dir != "" {
                std::env::set_current_dir(dir).unwrap();
            }
            convert_img(&filename);
            return;
        } else {
            println!("Invalid choice");
        }
    }
}