# File Encode and Decode to .PNG

## Description

This project is aimed at converting files to images so they can be uploaded to hosting platforms. It includes functionality for encoding and decoding image files using buffer writing to optimize memory usage. Larger files are split into multiple images to allow them to be uploaded to most platforms.

## Features

- Bulk operations for managing multiple images at once
- Encode files to .PNG
- Decode .PNG files back to the original format
- Command-line interface
- Simple UI for user interaction

## PreBuilt
Binary releases for windows-latest can be found [here](https://github.com/EvanRaeder/image_files/releases).

## Installation

1. Clone the repository: `git clone https://github.com/EvanRaeder/image_files`
2. Navigate to the project directory: `cd .\image_files\`
3. Install dependencies: `cargo build --release`

## Usage

### Drag And Drop

Simply drag and drop the file you want to encode or the directory you want to decode.

![Drag and Drop GIF](https://raw.githubusercontent.com/EvanRaeder/image_files/main/assets/dandd.gif)

### Command-Line Interface

To use the command-line interface, run the executable with the appropriate flags:

```sh
image_files.exe -e <filename> <output&optional> # Encode the specified file
image_files.exe -d <filename> <output&optional> # Decode the specified file
```

### Application Interface

1. Double-click run image_files.exe
2. Follow the prompts to either (e)ncode or (d)ecode a file and drag and drop or copy in the files location.
3. Specify a new working directory for the app if the files should be saved elsewhere.

![Encode UI GIF](https://raw.githubusercontent.com/EvanRaeder/image_files/main/assets/encode.gif)