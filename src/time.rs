#[derive(Clone)]
pub struct Time {
    pub hours: i32,
    pub minutes: i32,
    pub seconds: i32,
    pub milliseconds: i32,
}

impl std::fmt::Debug for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02}:{:02}:{:02},{:03}", self.hours, self.minutes, self.seconds, self.milliseconds)
    }
}

pub fn time_to_milliseconds(time: Time) -> i32 {
    return (time.hours * 3600000) + (time.minutes * 60000) + (time.seconds * 1000) + time.milliseconds;
}

pub fn milliseconds_to_time(milliseconds: i32) -> Time {
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

pub fn convert_line_to_times(line: String) -> (Time, Time) {
    let times: Vec<&str> = line.split(" --> ").collect();

    let start_time = convert_time_to_struct(times[0].to_string());
    let end_time = convert_time_to_struct(times[1].to_string());

    return (start_time, end_time);
}

pub fn convert_time_to_struct(time: String) -> Time {
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

pub fn get_min_max_times(current_times: (Time, Time), next_times: (Time, Time)) -> (Time, Time) {
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

pub fn has_overlap_times(current_times: (Time, Time), next_times: (Time, Time)) -> bool {
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

pub fn empty_time() -> (Time, Time) {
    return (Time { hours: 0, minutes: 0, seconds: 0, milliseconds: 0 }, Time { hours: 0, minutes: 0, seconds: 0, milliseconds: 0 });
}
