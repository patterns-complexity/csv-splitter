use std::{fs::File, io::Read};

pub fn files_to_bytes(files: Vec<File>) -> Vec<Vec<u8>> {
    let mut bytes: Vec<Vec<u8>> = Vec::new();

    for mut file in files {
        let mut file_bytes: Vec<u8> = Vec::new();

        let read_result = file.read_to_end(&mut file_bytes);

        match read_result {
            Ok(_) => (),
            Err(_) => panic!("Error reading file"),
        }

        bytes.push(file_bytes);
    }

    return bytes;
}

pub fn bytes_to_chunks(files: Vec<Vec<u8>>, granularity: usize) -> Vec<Vec<u8>> {
    let mut chunks: Vec<Vec<u8>> = Vec::new();

    for file in files {
        let mut line_count: usize = 0;
        let mut chunk: Vec<u8> = Vec::new();
        for byte in file {
            chunk.push(byte);
            if byte == 10 { 
                line_count += 1;
            }
            if line_count == granularity {
                chunks.push(chunk);
                chunk = Vec::new();
                line_count = 0;
            }
        }
        if chunk.len() > 0 {
            chunks.push(chunk);
        }
    }
    
    return chunks;
}
