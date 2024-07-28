use std::f64;
use image::ImageBuffer;

#[cfg(unix)]
fn separator() -> &'static str {
    "/"
}
#[cfg(unix)]
fn zip_extension() -> &'static str {
    ".tar.gz"
}
#[cfg(windows)]
fn separator() -> &'static str {
    "\\"
}
#[cfg(windows)]
fn zip_extension() -> &'static str {
    ".zip"
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
    //get the file path
    let file = std::path::Path::new(in_file);
    let file_name = file.file_name().unwrap().to_str().unwrap().to_owned();
    let dir_name = file.file_name().unwrap().to_str().unwrap().to_owned().split('.').collect::<Vec<&str>>()[0].to_owned();
    std::fs::create_dir_all(&dir_name).unwrap();
    let data = std::fs::read(file).unwrap();
    //split data into 100 mb chunks
    let data = data.chunks(99900000).map(|chunk| chunk.to_vec()).collect::<Vec<Vec<u8>>>();
    for (i, chunk) in data.iter().enumerate() {
        let file_name = file_name.clone();
        let file_name = file_name + "{" + &i.to_string() + "}" + ".png";
        let file_name = dir_name.clone() + separator() + &file_name;
        println!("Writing to: {}", file_name);
        let img = encode_data(chunk.clone());
        img.save(file_name).unwrap();
    }
}
fn encode_data(mut data: Vec<u8>) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    //get the length of the data in bits
    let length_bit = data.len() as f64;
    let (length, width) = file_size(length_bit as f64);
    println!("Length: {}, Width: {}", length, width);
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
    println!("Data length: {}", data.len()); 
    assert!(data.len() <= (length * width) as usize);// DO NOT REMOVE UNDER ANY CIRCUMSTANCES
    let length = f64::sqrt(data.len() as f64);
    let width = f64::ceil(data.len() as f64 / length);
    let img = image::DynamicImage::new_rgb8(length as u32, width as u32);
    //create a new image buffer
    let mut img: ImageBuffer<image::Rgba<u8>, Vec<u8>> = img.to_rgba8();
    //for each pixel in the image buffer set values of rgba to the four u8s
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let index = (x + y * length as u32) as usize;
        if index < data.len() {
            pixel[0] = data[index][0];
            pixel[1] = data[index][1];
            pixel[2] = data[index][2];
            pixel[3] = data[index][3];
        }
    }
    println!("DONE WRITE");
    img
}

//\\Decode the image into the file//\\
fn convert_img(input: &str) {
    //if the file is a png file
    if input.ends_with(".png") {
        let img = image::open(input).unwrap();
        let img: ImageBuffer<image::Rgba<u8>, Vec<u8>> = img.to_rgba8();
        let data = decode_img(img);
        let mut file_name = input.to_owned();
        file_name = file_name.split(separator()).collect::<Vec<&str>>().last().unwrap().to_owned().to_owned();
        file_name = file_name.split("{0}.").collect::<Vec<&str>>()[0].to_owned();
        std::fs::write(file_name, data).unwrap();
        return;
    }
    //if the file is a directory
    else {
        let dir = std::path::Path::new(input);
        let mut data = Vec::new();
        let entries = std::fs::read_dir(dir).unwrap();
        //sort entries by the number in {}.png in the filename
        let mut entries = entries.map(|entry| entry.unwrap()).collect::<Vec<std::fs::DirEntry>>();
        let file_name = entries[0].file_name().to_str().unwrap().split("{0}.").collect::<Vec<&str>>()[0].to_owned();
        let file_name = file_name.split(separator()).collect::<Vec<&str>>().last().unwrap().to_owned();
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
        for entry in entries {
            let path = entry.path();
            let img = image::open(path).unwrap();
            let img: ImageBuffer<image::Rgba<u8>, Vec<u8>> = img.to_rgba8();
            let data_chunk = decode_img(img);
            //remove last 23 bytes from the data
            let data_chunk = &data_chunk[..data_chunk.len()-23];
            data.extend(data_chunk);
        }
        println!("Writing to: {:?}", file_name);
        std::fs::write(file_name, data).unwrap();
    }
}
fn decode_img(img: ImageBuffer<image::Rgba<u8>, Vec<u8>> ) -> Vec<u8> {
    //create a new vector of 4 u8s
    let mut data = Vec::new();
    //for each pixel in the image buffer get the rgba values and push them to the data vector
    for (_x, _y, pixel) in img.enumerate_pixels() {
        data.push([pixel[0], pixel[1], pixel[2], pixel[3]]);
    }
    //convert data into a vector of u8s
    let data = data.iter().flat_map(|pixel| pixel.iter().cloned()).collect::<Vec<u8>>();
    //find the index of the last stop code at the end of the data
    let stop_index = data.iter().rposition(|&x| x == 0b11111111).unwrap(); //could be a one liner
    //remove the stop code and the extra bits
    let data = &data[..stop_index];
    //write the data back to the zip file
    data.to_vec()
}
    
fn main() {
    //check if unix or windows
    //if there are args provided if -e encode else if -d decode the given filename
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        if args[1] == "-e" {
            convert_file(&args[2]);
        } else if args[1] == "-d" {
            //dont name zip
            convert_img(&args[2]);
        }
        }
        else {
            //show the user how to use the program
            println!("Usage: image_files.exe -e <filename> or image_files.exe -d <filename>");
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
            let filename = filename.trim();
            convert_file(filename);
            return;
        } else if choice == "d" {
            println!("Enter the path to filename to decode");
            let mut filename = String::new();
            std::io::stdin().read_line(&mut filename).unwrap();
            let filename = filename.trim();
            convert_img(filename);
            return;
        } else {
            println!("Invalid choice");
        }
    }
}