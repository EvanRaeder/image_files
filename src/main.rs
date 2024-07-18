use std::f64;

fn file_size(bytes: f64) -> (f64, f64) {
    let size = bytes/4.0;
    let length = f64::ceil(f64::sqrt(size));
    let width = f64::ceil(f64::sqrt(size));
    (length, width)
}


fn main() {    
    //get data from the zip file
    let mut data = std::fs::read("test.zip").unwrap();
    //get the length of the data in bits
    let length_bit = data.len() as f64;
    let (length, width) = file_size(length_bit as f64);
    println!("Length: {}, Width: {}", length, width);
    let img = image::DynamicImage::new_rgb8(length as u32, width as u32);
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
    //make sure len of data is smaller than number of pixels
    assert!(data.len() <= (length * width) as usize);
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


    //\\WRITE//\\

    //read the image buffer from the png file
    let img = image::open("test.png").unwrap();
    let img = img.to_rgba8();
    //create a new vector of 4 u8s
    let mut data = Vec::new();
    //for each pixel in the image buffer get the rgba values and push them to the data vector
    for (_x, _y, pixel) in img.enumerate_pixels() {
        data.push([pixel[0], pixel[1], pixel[2], pixel[3]]);
    }
    //convert data into a vector of u8s
    let data = data.iter().flat_map(|pixel| pixel.iter().cloned()).collect::<Vec<u8>>();
    //find the index of the last stop code at the end of the data
    let stop_index = data.iter().rposition(|&x| x == 0b11111111).unwrap();
    //remove the stop code and the extra bits
    let data = &data[..stop_index];
    //write the data back to the zip file
    std::fs::write("test2.zip", data).unwrap();
    println!("DONE WRITE");
}