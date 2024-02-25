mod subtitle;
mod file;
mod time;
use subtitle::*;

fn main() {
    let input_file = String::from("files/input/test.srt");
    let simplified_file = String::from("files/simplified/test.srt");
    let output_file = String::from("files/output/test.srt");

    simplify_srt(input_file.clone(), simplified_file.clone());

    merge_subtitles(simplified_file.clone(), output_file.clone());
}
