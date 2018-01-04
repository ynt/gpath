extern crate regex;

use std::env;
use regex::Regex;

fn main() {
    print!("{}", get_gopath());
}

fn get_gopath() -> String {
    let mut res = String::new();

    if let Ok(path) = env::current_dir() {
        if let Some(path) = path.to_str() {
            if let Ok(re) = Regex::new("(?P<pre_path>.*)/src(?P<last_path>.*)") {
                if let Some(result) = re.captures(&path) {
                    res = "GOPATH=".to_owned() + &result["pre_path"];
                }
            }
        }
    };

    res
}
