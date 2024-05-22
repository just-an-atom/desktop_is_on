use std::fs;
use std::path::Path;

fn main() {
    let env_path = Path::new(".env");

    if env_path.exists() {
        let contents = fs::read_to_string(env_path).expect("Could not read .env file");

        for line in contents.lines() {
            if let Some((key, value)) = line.split_once('=') {
                println!("cargo:rustc-env={}={}", key, value);
            }
        }
    }
}
