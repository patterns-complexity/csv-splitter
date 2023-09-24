use std::{fs::File, path::Path};

mod loaders;
mod parsers;
mod queue;

use parsers::parameter_parser::get_parameters;
use parsers::file_parser::{files_to_bytes, bytes_to_chunks};
use loaders::file_loader::{get_file, list_dir_files};
use loaders::file_writer::save_bytes;
use queue::queue_writer::add_to_queue;

use crate::queue::connector;

async fn post_chunking(chunks: Vec<Vec<Vec<u8>>>, queue_name: &String, amqp_url: &String) {
    if queue_name == &format!("") { return; }

    let current_time = std::time::SystemTime::now();

    let connector = connector::connect(amqp_url).await;

    add_to_queue(&chunks, &queue_name, &connector).await;

    let elapsed_time_queueing = current_time.elapsed().unwrap();

    println!("\nFeeding the rabbit:");
    println!("{} microseconds", elapsed_time_queueing.as_micros());
    println!("which is ~");
    println!("{} milliseconds", elapsed_time_queueing.as_millis());
    println!("which is ~");
    println!("{} seconds", elapsed_time_queueing.as_secs());
}

async fn post_argument_parsing(
    input: &String,
    output: &String,
    granularity: &String,
    use_queue: &String,
    amqp_url: &String
) {
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

    let chunks_copy = chunks.clone();

    save_bytes(
        chunks_copy,
        output_path
    ).await;

    let elapsed_time_chunking = current_time.elapsed().unwrap();

    println!("Chunking time:");
    println!("{} microseconds", elapsed_time_chunking.as_micros());
    println!("which is ~");
    println!("{} milliseconds", elapsed_time_chunking.as_millis());
    println!("which is ~");
    println!("{} seconds", elapsed_time_chunking.as_secs());

    post_chunking(chunks, use_queue, amqp_url).await;
}

#[tokio::main]
async fn main() {
    let args = get_parameters();
    post_argument_parsing(&args[0], &args[1], &args[2], &args[3], &args[4]).await;
}
