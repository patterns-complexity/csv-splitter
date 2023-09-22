use std::{path::Path, io::Write};

pub fn save_bytes(chunks: Vec<Vec<u8>>, output_path: &Path) {
    let mut chunk_count = 0;
    for chunk in chunks {
        let mut output_file = std::fs::File::create(output_path.join(format!("{}.csv", chunk_count))).unwrap();
        output_file.write(&chunk).unwrap();
        chunk_count += 1;
    }
}
