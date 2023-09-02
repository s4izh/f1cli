use std::fs;
use std::io::prelude::*;
use std::path::Path;

pub fn fetch_or_cache(url: &str, cache_file: &str, cache_dir: &str) -> String {
    let cache = format!("{}/{}", cache_dir, cache_file);
    if !Path::new(&cache).exists() {
        let mut file = fs::File::create(&cache).unwrap();
        let body = reqwest::blocking::get(url).unwrap().text().unwrap();
        file.write_all(body.as_bytes()).unwrap();
    } else {
        println!("Using cache");
    }

    let mut content = String::new();
    let mut file = fs::File::open(&cache).unwrap();
    file.read_to_string(&mut content).unwrap();

    content
}
