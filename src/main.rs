use indicatif::ProgressStyle;
use clap::Parser;

mod file_conversion;
mod image_conversion;

#[cfg(unix)]
static SEPARATOR: &str = "/";
#[cfg(windows)]
static SEPARATOR: &str = "\\";

static STOP_CODE: u8 = 0b11111111;
static CHUNK_SIZE: usize = 99900000;
static MAX_THREADS: usize = 20;

#[derive(Parser)]
#[clap(version = "0.9.4", author = "Evan R", about = "Encodes files into images and decodes images into files")]
struct Cli {
    #[clap(long)]
    dir: Option<String>,
    #[clap(long,short)]
    encode: Option<String>,
    e: Option<String>,
    #[clap(long,short)]
    decode: Option<String>,
    d: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    if let Some(dir) = cli.dir {
        let dir = dir.trim().replace('"', "").replace("'", "");
        std::env::set_current_dir(dir).unwrap();
    }
    if let Some(filename) = cli.encode {
        let filename = filename.trim().replace('"', "").replace("'", "");
        file_conversion::convert_file(&filename);
    } else if let Some(filename) = cli.decode {
        let filename = filename.trim().replace('"', "").replace("'", "");
        image_conversion::convert_img(&filename);
    } else {
        let args: Vec<String> = std::env::args().collect();  
        if args.len()  == 1{
            cli_start();
        }else {
            let dir = std::path::Path::new(&args[1]);
            if dir.is_dir() {
                if args.len() == 3 {
                    std::env::set_current_dir(args[2].trim()).unwrap();
                }
                image_conversion::convert_img(&args[1]);
            } else if dir.is_file() {
                if args.len() == 3 {
                    std::env::set_current_dir(args[2].trim()).unwrap();
                }
                file_conversion::convert_file(&args[1]);
            } 
        }
    }
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
fn cli_start() {
    println!("Welcome to the image file encoder/decoder");
    println!("continue with the cli below or run image_files.exe -h for help");
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