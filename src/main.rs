use indicatif::ProgressStyle;

mod file_conversion;
mod image_conversion;

#[cfg(unix)]
static SEPARATOR: &str = "/";
#[cfg(windows)]
static SEPARATOR: &str = "\\";

pub static STOP_CODE: u8 = 0b11111111;
static CHUNK_SIZE: usize = 99900000;

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


fn main() {
    // Check if unix or windows
    // If there are args provided if -e encode else if -d decode the given filename
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 {
        if args[1] == "-e" {
            if args[3] != "" {
                std::env::set_current_dir(args[3].trim()).unwrap();
            }
            file_conversion::convert_file(&args[2]);
        } else if args[1] == "-d" {
            if args[3] != "" {
                std::env::set_current_dir(args[3].trim()).unwrap();
            }
            image_conversion::convert_img(&args[2]);
        } else { // Enter Drag and Drop mode
            // Check if arg[1] is an existing directory
            let dir = std::path::Path::new(&args[1]);
            if dir.is_dir() {
                if args.len() == 3 {
                    std::env::set_current_dir(args[2].trim()).unwrap();
                }
                image_conversion::convert_img(&args[1]);
            }else if dir.is_file() {
                if args.len() == 3 {
                    std::env::set_current_dir(args[2].trim()).unwrap();
                }
                file_conversion::convert_file(&args[1]);
            }else {
                println!("Invalid directory or file");
                println!("Usage: image_files.exe -e <filename> or image_files.exe -d <filename> or specify a working directory by using image_files.exe -d/e <filename> <directory>");
            }
        }
    }
    else {
        // Start with UI
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
            file_conversion::convert_file(&filename);
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
            image_conversion::convert_img(&filename);
            return;
        } else {
            println!("Invalid choice");
        }
    }
}