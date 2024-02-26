mod subtitle;
mod file;
mod time;

fn main() {
    let input_directory = String::from("files/input");
    let simplified_directory = String::from("files/simplified");
    let output_directory = String::from("files/output");

    let files = file::get_srt_file_names(input_directory.clone());
    // Filter files removing the files test.srt and .gitkeep
    let files = files
        .iter()
        .filter(|file| file != &&String::from("files/input/test.srt"))
        .collect::<Vec<&String>>();

    for file in files {
        let input_file = format!("{}/{}", input_directory.clone(), file);
        let simplified_file = format!("{}/{}", simplified_directory.clone(), file);
        let output_file = format!("{}/{}", output_directory.clone(), file);

        subtitle::simplify_srt(input_file.clone(), simplified_file.clone());
        subtitle::merge_subtitles(simplified_file.clone(), output_file.clone());
    }
}
