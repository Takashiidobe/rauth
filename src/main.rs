use google_authenticator::GoogleAuthenticator;
use shellexpand;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

static FILE_PATH: &str = "~/.config/rauth.csv";

fn main() -> io::Result<()> {
    let path = shellexpand::tilde(FILE_PATH);
    let file = match File::open(&*path) {
        Ok(f) => f,
        _ => panic!(
            "{path} does not exist, please create {path}",
            path = FILE_PATH
        ),
    };
    let reader = BufReader::new(file);

    let authenticator = GoogleAuthenticator::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let split_line: Vec<&str> = line.split(":").collect();

        let secret: String = split_line[1]
            .to_string()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();

        let code = authenticator.get_code(&secret, 0).unwrap();
        if authenticator.verify_code(&secret, &code, 1, 0) {
            println!("{}: {}", &split_line[0], &code);
        }
    }

    Ok(())
}
