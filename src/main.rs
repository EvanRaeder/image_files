use std::{f64};

fn file_size(bits: f64) -> (f64, f64) {
    let size = bits/32.0;
    let length = f64::ceil(f64::sqrt(size));
    let width = f64::ceil(f64::sqrt(size));
    (length, width)
}


fn main() {    
    //get data from the zip file
    let data = std::fs::read("test.zip").unwrap();
    let length_bit = data.len();
    println!("size: {}", length_bit);
    let (length, width) = file_size(length_bit as f64);
    println!("Length: {}, Width: {}", length, width);
    let img = image::DynamicImage::new_rgb8(length as u32, width as u32);
    //add a binary stop code to the data
    let mut data = data.to_vec();
    data.push(0b11111111);
    //make sure the data is a multiple of 32 bits
    while data.len() % 32 != 0 {
        data.push(0);
    }
    // create a new vector of 8bit chunks
    let mut data = data.chunks(8).map(|chunk| {
        let mut byte = 0;
        for (i, bit) in chunk.iter().enumerate() {
            byte |= (*bit as u8) << i;
        }
        byte
    }).collect::<Vec<u8>>();
    //get data into vecs of 4 u8s
    let mut data = data.chunks(4).map(|chunk| {
        let mut byte = [0; 4];
        for (i, bit) in chunk.iter().enumerate() {
            byte[i] = *bit;
        }
        byte
    }).collect::<Vec<[u8; 4]>>();
    println!("{:?}", data.len());
    //allign the data to the length and width of the image

    //create a new image buffer
    let mut img = img.to_rgba8();
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
    //save the image buffer to a png file
    img.save("test.png").unwrap();
    println!("DONE SAVE");


}
