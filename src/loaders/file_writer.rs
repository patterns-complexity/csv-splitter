use std::path::Path;
use tokio::{fs::File, io::AsyncWriteExt};

pub async fn save_bytes(files: Vec<Vec<Vec<u8>>>, output_path: &Path) -> () {
    let mut file_counter: usize = 0;
    for file in files {
        file_counter += 1;
        let mut chunk_counter: usize = 0;
        for chunk in file {
            chunk_counter += 1;
            let mut file = match File::create(
                output_path.join(
                    format!(
                        "file_{}_chunk_{}.csv",
                        file_counter,
                        chunk_counter
                    )
                )
            ).await {
                Ok(file) => file,
                Err(e) => panic!("Error: {}", e),
            };

            match file.write_all(&chunk).await {
                Ok(_) => (),
                Err(e) => panic!("Error: {}", e),
            }
        }
    }
}
