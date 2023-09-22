use std::{fs::File, path::Path};

mod loaders;
mod parsers;

use parsers::parameter_parser::get_parameters;
use parsers::file_parser::{files_to_bytes, bytes_to_chunks};
use loaders::file_loader::{get_file, list_dir_files};
use loaders::file_writer::save_bytes;

fn post_argument_parsing(input: &String, output: &String, granularity: &String) {
    // clear output folder
    std::fs::remove_dir_all(output).unwrap();
    std::fs::create_dir(output).unwrap();

    let input_path = Path::new(input);
    let output_path = Path::new(output);
    let granularity = granularity.parse::<i32>().unwrap();

    let current_time = std::time::SystemTime::now();

    let mut files: Vec<File> = Vec::new();

    let dir_entries = list_dir_files(input_path);

    for dir_entry in dir_entries {
        files.push(get_file(dir_entry));
    }

    let bytes = files_to_bytes(files);

    let chunks = bytes_to_chunks(bytes, granularity as usize);

    save_bytes(chunks, output_path);

    let elapsed_time = current_time.elapsed().unwrap();

    println!("Elapsed time:");
    println!("{} microseconds", elapsed_time.as_micros());
    println!("which is ~");
    println!("{} milliseconds", elapsed_time.as_millis());
    println!("which is ~");
    println!("{} seconds", elapsed_time.as_secs());

}

fn main() {
    let args = get_parameters();
    post_argument_parsing(&args[0], &args[1], &args[2]);
}
