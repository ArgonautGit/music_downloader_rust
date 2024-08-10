struct Song {
    title: String,
    album: String,
    artist: String,
    id: String,
    url: String,
}

impl Song {
    pub fn new(url: &str) -> Song {
        const DEFAULT: String = String::new();
        let mut song = Song {
            url: url.into(),
            // Initialize the rest into nothing (default values)
            title: DEFAULT,
            album: DEFAULT,
            artist: DEFAULT,
            id: DEFAULT,
        };

        song.get_info();

        return song;
    }

    fn get_info(&mut self) {
        bash::shell_command(&self.url);

        // TODO: write new info to struct.
    }
}

pub mod bash {
    pub fn shell_command(command: &str) {
        use std::process::Command;

        let process = Command::new(command)
            .output()
            .expect("Failed to execute process.");

        let output = String::from_utf8_lossy(&process.stdout);
        let output = output.trim();

        dbg!(output);
    }

    fn _shell_command_stdout(_command: &str) {
        // TODO: P
    }
}
