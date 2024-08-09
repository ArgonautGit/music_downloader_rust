struct Song {
    title: String,
    album: String,
    artist: String,
    id: String,
    url: String,
}

impl Song {
    pub fn new(url: &str) -> Song {
        // TODO

        Song { title: "".to_owned(), album: "".to_owned(), artist: "".to_owned(), id: "".to_owned(), url: "".to_owned() }
    }

    fn get_info(&self) {
        bash::shell_command(&self.url);
    }
}

mod bash {
    pub fn shell_command(command: &str) {
        std::process::Command::new(command);
    }

    fn shell_command_stdout(command: &str) {
        // TODO
    }

}