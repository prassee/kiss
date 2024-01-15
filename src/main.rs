use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
    process::Command,
};

use serde_json::Value;

const KITTY_PATH: &str = "/tmp/kitty-session.kitty";

/**
Entry point for the application
*/
fn main() {
    println!("welcome to KISS(KItty Session Saver)");
    let cmd = Command::new("kitty")
        .args(&["@", "ls"])
        .output()
        .expect("cannot execute the kitty command");
    let cmd_output = String::from_utf8_lossy(&cmd.stdout).to_string();
    remove_kitty_file();
    parse_kitty_session(cmd_output.as_str());
}

/**
Parse kitty session for the given `data`
*/
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
    println!("kitty session written to - {:?}", KITTY_PATH);
    append_to_file(config);
}

/**
Append tab config to the created config file
*/
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
Removes previous version of the file if exist.
*/
fn remove_kitty_file() {
    let file_exists = Path::new(KITTY_PATH).exists();
    if file_exists {
        std::fs::remove_file(KITTY_PATH).expect("file cannot be deleted");
    }
}
