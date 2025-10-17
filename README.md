# fileorg 
A simple and fast CLI tool for organising files using custom tags.

## Features
- Move files using short tags instead of long folder paths
- Persistent tag storage in ~/.file_organiser/tags.json
- Supports relative and absolute file paths
- Safe file moving with validation
- Minimal, fast and written in Rust 

## Installation

### Option 1 – Install using Cargo (recommended)
Requires Rust installed from https://rustup.rs  
Run:

cargo install --git https://github.com/M4R5-PH0B05/file-organiser-cli.git

Then run it from anywhere with:

fileorg

### Option 2 – Build from source
git clone https://github.com/M4R5-PH0B05/file-organiser-cli.git
cd file-organiser-cli
cargo build --release
./target/release/fileorg

## Requirements
- Rust 1.70+
- macOS / Linux / Windows
- Cargo (for installation)

## Contributing
Pull requests welcome! Fork the repo and improve it if you like.

Built by @mars_phobos 
