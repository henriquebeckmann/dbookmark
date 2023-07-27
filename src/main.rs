use regex::Regex;
use std::env;
use std::process::{exit, Command};
use std::str;

fn main() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("bookmark ls --sort name | awk 'NF > 0' | dmenu -i -l 12 -p 'bookmark:'")
        .output()
        .expect("failed to execute process");

    if !output.status.success() {
        if let Some(code) = output.status.code() {
            println!("command exited with status code: {}", code);
        } else {
            println!("command process was terminated by a signal");
        }
        exit(1);
    }

    let output_str = str::from_utf8(&output.stdout).unwrap().trim_end();
    let url_re = Regex::new(r"^https?://").unwrap();
    let browser = env::var("BROWSER").unwrap_or("firefox".to_string());

    if output_str == "Id                 Name             URL                                   Group           Tags" {
        return;
    }

    if url_re.is_match(output_str) {
        Command::new(&browser)
            .arg(output_str)
            .status()
            .expect("Failed to launch the browser");
    } else {
        let re = Regex::new(r"\s{2,}").unwrap();
        let fields: Vec<&str> = re.split(output_str).collect();

        if fields.len() < 4 {
            let search_url = format!("https://duckduckgo.com/?q={}", output_str);
            Command::new(&browser)
                .arg(&search_url)
                .status()
                .expect("Failed to launch the browser");
        } else {
            let name = fields.get(1).expect("Expected at least 2 fields");
            let url = fields.get(2).expect("Expected at least 3 fields");
            let group = fields.get(3).expect("Expected at least 4 fields");

            if group == &"search" {
                let search_term = Command::new("sh")
                    .arg("-c")
                    .arg(format!("echo \"\" | dmenu -p \"{}\":", name))
                    .output()
                    .expect("failed to execute process")
                    .stdout;

                let search_term_str = str::from_utf8(&search_term).unwrap().trim_end();
                let new_url = format!("{}{}", url, search_term_str);

                Command::new(&browser)
                    .arg(&new_url)
                    .status()
                    .expect("Failed to launch the browser");
            } else {
                Command::new(&browser)
                    .arg(url)
                    .status()
                    .expect("Failed to launch the browser");
            }
        }
    }
}
