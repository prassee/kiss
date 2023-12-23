use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, Write},
    str,
};

use serde_json::Value;

const KITTY_PATH: &str = "/tmp/kitty-sesison.kitty";

fn main() {
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
    values[0]["tabs"].as_array().unwrap().iter().for_each(|x| {
        let tab_config = format!(
            "\nnew_tab {:} \nlayout {:} \ncd {:} \ntitle {:} \nlaunch --env KITTY_WINDOW_ID={:} --env PWD=/home/saipranav \nfocus\n",
            x["title"].as_str().unwrap(), x["layout"].as_str().unwrap(), x["windows"][0]["cwd"].as_str().unwrap(), x["windows"][0]["title"].as_str().unwrap(),x["windows"][0]["id"].as_number().unwrap().to_string()
        );
        append_to_file(tab_config);
    });
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
