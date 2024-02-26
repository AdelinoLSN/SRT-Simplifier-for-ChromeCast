use crate::file::{get_file_contents, write_to_file};
use crate::time::{Time, convert_line_to_times, get_min_max_times, has_overlap_times, empty_time};

pub const SRT_TAGS: [&str; 2] = [
    r"{\an8}",
    r"{=0}"
];

#[derive(Clone)]
pub struct Subtitle {
    pub number: String,
    pub times: (Time, Time),
    pub texts: Vec<String>,
}

pub fn simplify_srt(input_file: String, simplified_file: String) {
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

pub fn merge_subtitles(simplified_file: String, output_file: String) {
    let content = get_file_contents(simplified_file);

    let subtitles: Vec<Subtitle> = content_to_subtitles(content);

    let subtitles: Vec<Subtitle> = merge_subtitles_with_overlap_times(subtitles);

    let mut new_content = String::new();

    for subtitle in subtitles.iter() {
        new_content.push_str(&subtitle.number);
        new_content.push_str("\n");
        new_content.push_str(&format!("{:?} --> {:?}", subtitle.times.0, subtitle.times.1));
        new_content.push_str("\n");
        for text in subtitle.texts.iter() {
            new_content.push_str(text);
            new_content.push_str("\n");
        }
        new_content.push_str("\n");
    }

    write_to_file(output_file, new_content);
}

fn content_to_subtitles(content: String) -> Vec<Subtitle> {
    let mut subtitles: Vec<Subtitle> = Vec::new();

    let mut iterator = 0;

    let mut number = String::new();
    let mut times: (Time, Time) = empty_time();
    let mut texts: Vec<String> = Vec::new();

    for line in content.lines() {
        match line {
            "" => {
                let subtitle = Subtitle {
                    number: number.clone(),
                    times: times.clone(),
                    texts: texts.clone(),
                };
                subtitles.push(subtitle);

                number = String::new();
                times = empty_time();
                texts = Vec::new();

                iterator = 0;
            },
            _ => {
                match iterator {
                    0 => {
                        number = line.to_string();
                    },
                    1 => {
                        times = convert_line_to_times(line.to_string());
                    },
                    _ => {
                        texts.push(line.to_string());
                    }
                }

                iterator += 1;
            }
        }
    }

    return subtitles;
}

fn merge_subtitles_with_overlap_times(subtitles: Vec<Subtitle>) -> Vec<Subtitle> {
    let mut subtitles_merged: Vec<Subtitle> = Vec::new();

    let mut iterator = 0;

    while iterator < (subtitles.len() - 1) {
        let current_subtitle = subtitles[iterator].clone();
        let next_subtitle = subtitles[iterator + 1].clone();

        let has_overlap = has_overlap_times(current_subtitle.clone().times, next_subtitle.clone().times);

        match has_overlap {
            true => {
                let merged_subtitle = merge_subtitles_in_one(current_subtitle.clone(), next_subtitle.clone());
                subtitles_merged.push(merged_subtitle);

                iterator += 2;
            },
            false => {
                subtitles_merged.push(current_subtitle);

                iterator += 1;
            }
        }
    }

    if iterator == (subtitles.len() - 1) {
        let last_subtitle = subtitles[iterator].clone();
        subtitles_merged.push(last_subtitle);
    }

    return subtitles_merged;
}

fn merge_subtitles_in_one(current_subtitle: Subtitle, next_subtitle: Subtitle) -> Subtitle {
    let number = current_subtitle.number.clone();
    let times = get_min_max_times(current_subtitle.times, next_subtitle.times);
    let mut texts: Vec<String> = Vec::new();

    for text in current_subtitle.texts.iter() {
        texts.push(text.to_string());
    }
    for text in next_subtitle.texts.iter() {
        texts.push(text.to_string());
    }

    let subtitle = Subtitle {
        number: number,
        times: times,
        texts: texts.clone(),
    };

    return subtitle;
}


#[allow(dead_code)]
fn print_subtitles(subtitles: Vec<Subtitle>) {
    for subtitle in subtitles.iter() {
        println!("{}", subtitle.number);
        println!("{:?}", subtitle.times);
        for text in subtitle.texts.iter() {
            println!("{}", text);
        }
        println!("");
    }
}
