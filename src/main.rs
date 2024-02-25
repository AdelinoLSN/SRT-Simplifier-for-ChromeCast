use std::fs;

const SRT_TAGS: [&str; 2] = [
    r"{\an8}",
    r"{=0}"
];

fn main() {
    let input_file = String::from("files/input/test.srt");
    let simplified_file = String::from("files/simplified/test.srt");
    let _output_file = String::from("files/output/test.srt");

    simplify_srt(input_file.clone(), simplified_file.clone());
}

fn simplify_srt(input_file: String, simplified_file: String) {
    let content = get_file_contents(input_file);

    let mut simplified_content = String::new();

    for line in content.lines() {
        let mut new_line = line.to_string();
        for tag in SRT_TAGS.iter() {
            new_line = new_line.replace(tag, "");
        }
        simplified_content.push_str(&new_line);
        simplified_content.push_str("\n");
    }

    write_to_file(simplified_file, simplified_content);
}

fn get_file_contents(path: String) -> String {
    let content = fs::read_to_string(path)
        .expect("Something went wrong reading the file");

    return content;
}

fn write_to_file(path: String, content: String) {
    fs::write(path, content)
        .expect("Something went wrong writing the file");
}
