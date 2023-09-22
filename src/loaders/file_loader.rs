use std::{fs::{File, DirEntry}, path::Path};

pub fn get_file(dir_entry: DirEntry) -> File {
    let file = match File::open(dir_entry.path()) {
        Ok(file) => file,
        Err(e) => panic!("Error: {}", e),
    };

    return file;
}

pub fn list_dir_files(path: &Path) -> Vec<DirEntry> {
    let path_dir = path.read_dir();
    let mut file_list: Vec<DirEntry> = Vec::new();

    let dir = match path_dir {
        Ok(dir) => dir,
        Err(e) => panic!("Error: {}", e),
    };

    for file in dir {
        let f = match file {
            Ok(file) => file,
            Err(e) => panic!("Error: {}", e),
        };
        // check if file is a .csv file
        if f.path().extension().unwrap() != "csv" {
            continue;
        }
        
        file_list.push(f);
    }

    return file_list;
}
