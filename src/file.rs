use std::fs;

pub fn get_srt_file_names(directory: String) -> Vec<String> {
    let paths = fs::read_dir(directory).unwrap();

    let mut files: Vec<String> = Vec::new();

    for path in paths {
        let path = path.unwrap().path();
        let filename = path.file_name().unwrap().to_str().unwrap().to_string();
        if filename.ends_with(".srt") {
            files.push(filename);
        }
    }

    return files;
}

pub fn get_file_contents(path: String) -> String {
    let content = fs::read_to_string(path)
        .expect("Something went wrong reading the file");

    return content;
}

pub fn write_to_file(path: String, content: String) {
    fs::write(path, content)
        .expect("Something went wrong writing the file");
}
