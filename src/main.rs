use std::fs;

#[derive(Clone)]
struct Subtitle {
    number: String,
    times: (Time, Time),
    texts: Vec<String>,
}

#[derive(Clone)]
struct Time {
    hours: i32,
    minutes: i32,
    seconds: i32,
    milliseconds: i32,
}

// Implement Debug trait for Time struct
impl std::fmt::Debug for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02}:{:02}:{:02},{:03}", self.hours, self.minutes, self.seconds, self.milliseconds)
    }
}

// Create a global constant that contains the srt tags to be removed
const SRT_TAGS: [&str; 2] = [
    r"{\an8}",
    r"{=0}"
];

fn main() {
    let input_file = String::from("files/input/test.srt");
    let simplified_file = String::from("files/simplified/test.srt");
    let output_file = String::from("files/output/test.srt");

    simplify_srt(input_file.clone(), simplified_file.clone());

    merge_subtitles(simplified_file.clone(), output_file.clone());
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

fn merge_subtitles(simplified_file: String, output_file: String) {
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

fn convert_line_to_times(line: String) -> (Time, Time) {
    let times: Vec<&str> = line.split(" --> ").collect();

    let start_time = convert_time_to_struct(times[0].to_string());
    let end_time = convert_time_to_struct(times[1].to_string());

    return (start_time, end_time);
}

fn convert_time_to_struct(time: String) -> Time {
    let time_splitted_comma = time.split(",").collect::<Vec<&str>>();
    let time_splitted_two_points = time_splitted_comma[0].split(":").collect::<Vec<&str>>();

    let hours = time_splitted_two_points[0].parse::<i32>().unwrap();
    let minutes = time_splitted_two_points[1].parse::<i32>().unwrap();
    let seconds = time_splitted_two_points[2].parse::<i32>().unwrap();
    let milliseconds = time_splitted_comma[1].parse::<i32>().unwrap();

    let time = Time {
        hours: hours,
        minutes: minutes,
        seconds: seconds,
        milliseconds: milliseconds,
    };

    return time;
}

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

fn empty_time() -> (Time, Time) {
    return (Time { hours: 0, minutes: 0, seconds: 0, milliseconds: 0 }, Time { hours: 0, minutes: 0, seconds: 0, milliseconds: 0 });
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

    return subtitles_merged;
}

fn has_overlap_times(current_times: (Time, Time), next_times: (Time, Time)) -> bool {
    let current_start_time_in_milliseconds = time_to_milliseconds(current_times.0);
    let current_end_time_in_milliseconds = time_to_milliseconds(current_times.1);

    let next_start_time_in_milliseconds = time_to_milliseconds(next_times.0);
    let next_end_time_in_milliseconds = time_to_milliseconds(next_times.1);

    let c_s_t = current_start_time_in_milliseconds;
    let c_e_t = current_end_time_in_milliseconds;
    let n_s_t = next_start_time_in_milliseconds;
    let n_e_t = next_end_time_in_milliseconds;

    if (c_s_t <= n_s_t && n_s_t <= c_e_t) || (c_s_t <= n_e_t && n_e_t <= c_e_t) {
        return true;
    }

    return false;
}

fn time_to_milliseconds(time: Time) -> i32 {
    return (time.hours * 3600000) + (time.minutes * 60000) + (time.seconds * 1000) + time.milliseconds;
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

fn get_min_max_times(current_times: (Time, Time), next_times: (Time, Time)) -> (Time, Time) {
    let current_start_time_in_milliseconds = time_to_milliseconds(current_times.0);
    let current_end_time_in_milliseconds = time_to_milliseconds(current_times.1);

    let next_start_time_in_milliseconds = time_to_milliseconds(next_times.0);
    let next_end_time_in_milliseconds = time_to_milliseconds(next_times.1);

    let min_start_time_in_milliseconds = std::cmp::min(current_start_time_in_milliseconds, next_start_time_in_milliseconds);
    let max_end_time_in_milliseconds = std::cmp::max(current_end_time_in_milliseconds, next_end_time_in_milliseconds);

    let min_start_time = milliseconds_to_time(min_start_time_in_milliseconds);
    let max_end_time = milliseconds_to_time(max_end_time_in_milliseconds);

    return (min_start_time, max_end_time);
}

fn milliseconds_to_time(milliseconds: i32) -> Time {
    let hours = milliseconds / 3600000;
    let minutes = (milliseconds % 3600000) / 60000;
    let seconds = ((milliseconds % 3600000) % 60000) / 1000;
    let milliseconds = ((milliseconds % 3600000) % 60000) % 1000;

    let time = Time {
        hours: hours,
        minutes: minutes,
        seconds: seconds,
        milliseconds: milliseconds,
    };

    return time;
}
