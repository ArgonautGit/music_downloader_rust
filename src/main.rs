mod music;

use std::fs;
use sscanf::sscanf;

fn main() {
    // let _test_song = music::Song::new("https://www.youtube.com/watch?v=dQw4w9WgXcQ");

    // music::file::list_songs_directory();

    let paths = fs::read_dir("./music").expect("Failed to read directory!");

    let mut song_files = Vec::new();
    for i in paths {
        let filename = i.expect("Could not get filename!").path()
        .display()
        .to_string();
        
        let filename = sscanf!(filename, "./music/{}", String).expect("Failed to parse filename!");
        song_files.push(filename);
    } println!();
}
