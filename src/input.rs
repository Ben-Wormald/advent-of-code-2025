use reqwest::{blocking::Client, header::{COOKIE, HeaderMap, HeaderValue, USER_AGENT}};
use std::{env, fs::{read_to_string, File}, io::Write};

const URL: &str = "https://adventofcode.com";
const YEAR: &str = "2025";

pub fn get_input(day: &str, test: bool) -> String {
    match test {
        true => read_file("test").expect("no test input found!"),
        false => read_file(day).unwrap_or_else(|| fetch_and_save(day)),
    }
}

fn read_file(file_name: &str) -> Option<String> {
    let path = format!("./input/{file_name}.txt");
    read_to_string(path).ok()
}

fn fetch_and_save(file_name: &str) -> String {
    let client = get_client();

    let day = file_name.replace("day_0", "").replace("day_", "");
    let url = format!("{URL}/{YEAR}/day/{day}/input");

    let input = client.get(url).send().unwrap().text().unwrap();

    let path = format!("./input/{file_name}.txt");
    let mut file = File::create(path).unwrap();
    file.write_all(input.as_bytes()).ok();

    input
}

fn get_client() -> Client {
    let session_cookie = env::var("SESSION_COOKIE").expect("no SESSION_COOKIE set!");
    let session_cookie = format!("session={session_cookie}");

    let mut headers = HeaderMap::new();
    headers.insert(
        COOKIE,
        HeaderValue::from_str(&session_cookie).unwrap(),
    );
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("https://github.com/Ben-Wormald/advent-of-code-2024"),
    );

    Client::builder()
        .default_headers(headers)
        .build().unwrap()
}
