use std::fs;

pub fn get_file_contents(path: String) -> String {
    let content = fs::read_to_string(path)
        .expect("Something went wrong reading the file");

    return content;
}

pub fn write_to_file(path: String, content: String) {
    fs::write(path, content)
        .expect("Something went wrong writing the file");
}
