use sscanf;

/// Struct to represent songs. Most of their info can be derived from the url
/// using built-in function implementations.
#[derive(Debug)]
pub struct Song {
    title: String,
    album: String,
    artist: String,
    id: String,
    url: String,
}

#[allow(dead_code)]
impl Song {
    pub fn new(url: &str) -> Song {
        let mut song = Self::default();
        song.url = url.to_owned();

        song.get_info();

        return song;
    }

    fn get_info(&mut self) -> &mut Self {
        // Set up shell command.
        let command = format!(
            "yt-dlp --quiet --no-warnings --print \"id: %(id)s, title: %(title)s, artist: %(channel)s, album: %(album)s,\" {}",
            self.url
        );

        // Run command and record output.
        let output = shell::shell_command(&command);

        // Parse output.
        let (id, title, artist, album) = sscanf::sscanf!(
            output,
            "id: {}, title: {}, artist: {}, album: {},",
            String,
            String,
            String,
            String
        )
        .expect("Failed to parse song info!")
        .to_owned(); // Create owned `String`.

        // Write new values to struct (is there a easier way to do this?).
        self.id = id;
        self.title = title;
        self.artist = artist;
        self.album = album;

        return self;
    }
}

impl Default for Song {
    fn default() -> Song {
        return Song {
            title: "".to_owned(),
            album: "".to_owned(),
            artist: "".to_owned(),
            id: "".to_owned(),
            url: "".to_owned(),
        };
    }
}

/// Module to run bash commands and receive their output.
mod shell {

    /// # Description
    /// Takes a shell command and returns a `String` containing the output.
    /// # Example
    /// ```
    /// dbg!(shell_command("echo TEST"));
    /// ```
    pub fn shell_command(command: &str) -> String {
        use std::process::Command;

        // Run shell command and store it in `process`.
        let process = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("Failed to execute process.");

        // Take output from `process` and convert it to a `String`.
        let output = String::from_utf8_lossy(&process.stdout);
        // Take `output`, trim any newlines and spaces, and convert to `String`.
        let output = output.trim().to_string();

        return output;
    }

    pub fn _shell_command_stdout(_command: &str) /*-> String*/ {
        // TODO: Print out put to stdout.
    }
}

#[allow(dead_code)]
pub mod file {
    use super::shell;

    pub fn list_songs_directory() -> Vec<String> {
        use super::parse;

        // Prepare command to run.
        let command = "ls";
        // Run command.
        let output = shell::shell_command(command);

        return parse::parse_music_directory(&output);
    }
}

#[allow(dead_code)]
mod parse {
    /// Takes the output from ls and returns a vector of file names.
    pub fn parse_music_directory(ls_output: &str) -> Vec<String> {
        // Create `String` vector to hold the names of all the songs in the music directory.
        let mut song_files = Vec::new();
        // let mut current_word
        // for i in ls_output.chars() {
        //     while (i != '\'') || (i != '\n') { song_list.get }
        // };



        return song_files;
    }
}