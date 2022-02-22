// You and your friends like to share YouTube links all throughout the day. You want to keep track
// of all the videos you watch in your own personal notepad, but you find that keeping the entire
// link is unneccessary. Keep the video ID (the combination of letters and numbers at the end of
// the link) in your notepad to slim down the URL.
//
// Task:
// Create a program that parses through a link, extracts and outputs the YouTube video ID.
//
// Input format:
// A string containing the URL to a YouTube video. Te format of the string can be in
// "https://www.youtube.com/watch?v=kbxkq_w51PM" or the shortened format
// "https://youtu.be/KMBBjzp5hdc" format.
//
// Output format:
// A string containing the extracted YouTube video ID.

use regex::Regex;

#[allow(dead_code)]
pub fn youtube_link_finder(param: &str) -> String {
    let regular_link_regex = Regex::new(r"^https://www.youtube.com/watch\?v=(.*)$").unwrap();
    let shortened_link_regex = Regex::new(r"^https://youtu.be/(.*)$").unwrap();

    match regular_link_regex.captures(param) {
        Some(caps) => caps.get(1).unwrap().as_str().to_owned(),
        None => {
            let caps = shortened_link_regex.captures(param).unwrap();
            caps.get(1).unwrap().as_str().to_owned()
        }
    }
}

#[test]
pub fn youtube_link_finder_test() {
    assert_eq!(youtube_link_finder("https://www.youtube.com/watch?v=kbxkq_w51PM"), "kbxkq_w51PM");
    assert_eq!(youtube_link_finder("https://youtu.be/KMBBjzp5hdc"), "KMBBjzp5hdc");
}
