use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, Write},
    path::Path,
    str,
};

use serde_json::Value;

const KITTY_PATH: &str = "/tmp/kitty-sesison.kitty";

fn main() {
    remove_kitty_file();
    let input = io::stdin()
        .lock()
        .lines()
        .fold("".to_string(), |acc, line| acc + &line.unwrap() + "\n");
    parse_kitty_session(&input);
}

fn parse_kitty_session(data: &str) {
    let values: Value = serde_json::from_str(data).unwrap();
    // create a file to stage the chagnes
    File::create(KITTY_PATH).expect("file not found");
    let mut config = String::new();
    values[0]["tabs"].as_array().unwrap().iter().for_each(|tab| {
        let tab_config = format!(
            "\nnew_tab {:} \nlayout {:} \ncd {:} \ntitle {:} \nlaunch --env KITTY_WINDOW_ID={:} --env PWD=/home/saipranav \nfocus\n",
            tab["title"].as_str().unwrap(), 
            tab["layout"].as_str().unwrap(), 
            tab["windows"][0]["cwd"].as_str().unwrap(), 
            tab["windows"][0]["title"].as_str().unwrap(),
            tab["windows"][0]["id"].as_number().unwrap().to_string()
        );
        config.push_str(&tab_config);
    });
    println!("{:?}", config);
    append_to_file(config);
}

/*
*
* append tab config to the created config file
* */
fn append_to_file(tab_config: String) {
    let mut data_file = OpenOptions::new()
        .append(true)
        .open(KITTY_PATH)
        .expect("output file doesnot exist");
    data_file
        .write(tab_config.as_bytes())
        .expect("write failed");
}
/**
* remove previous version of the file if exist
* */
fn remove_kitty_file() {
    let file_exists = Path::new(KITTY_PATH).exists();
    if file_exists {
        std::fs::remove_file(KITTY_PATH).expect("file cannot be deleted");
    }
}
