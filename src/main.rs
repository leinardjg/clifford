mod defaults;

use clap::Parser;
use defaults::MAX_HISTORY_COUNT;
use std::fs::{ read_to_string, write };
use std::path::Path;

static CONFIG_FILE_NAME : &str = "./clifford.config";
static CLIFFORD_FILE_NAME : &str = "./clifford.";
static CLIFFORD_FILE_EXTENSION : &str = ".data";
static CLIFFORD_SAVE_FILE_NAME : &str = "./clifford.save";

#[derive(Parser)]
struct Args {
    /// optional string to process
    string : Option<String>
}

fn get_filename(index : i32) -> String {
    CLIFFORD_FILE_NAME.to_string() + &index.to_string() + &CLIFFORD_FILE_EXTENSION.to_string()
}

fn into_command(arguments : Args) {
    let max_history_count = match get_file_value_int(CONFIG_FILE_NAME, "MAX_HISTORY_COUNT") {
        Some(number) => number,
        None => MAX_HISTORY_COUNT
    };

    let filename = match get_file_value_int(CLIFFORD_SAVE_FILE_NAME, "CURRENT_INDEX") {
        Some(value) => {
            get_filename(value % max_history_count)
        },
        None => {
            CLIFFORD_FILE_NAME.to_string() + &(0.to_string()) + &CLIFFORD_FILE_EXTENSION.to_string()
        }
    };

    let content = match arguments.string {
        Some(text) => text,
        None => "".to_string(),
    };

    let _ = write(filename, content);
    
}

fn outof_command() {

    let index = match get_file_value_int(CLIFFORD_SAVE_FILE_NAME, "CURRENT_INDEX") {
        Some(number) => {
            if number - 1 > 0 {
                number
            } else {
                0
            }
        },
        None => 0
    };

    let filename = CLIFFORD_FILE_NAME.to_string() + &(index).to_string() + &CLIFFORD_FILE_EXTENSION.to_string();

    if !Path::new(&filename).exists() {
        return println!("");
    }

    let content = read_to_string(filename).expect("Error");
    println!("{}", content)
}

fn parse_arguments(arguments : Args) {

    if arguments.string.is_some() {
        into_command(arguments);
    } else {
        outof_command();
    }

}

#[allow(dead_code)]
fn get_file_value_str(filename : &str, varname : &str) -> Option<String> {

    if !Path::new(filename).exists() {
        return None
    }
    
    for line in read_to_string(filename).unwrap().lines() {
        let mut split = line.splitn(2, "=");

        // match name
        let name: &str = match split.next() {
            Some(text) => text.trim(),
            None => "",
        };

        if name != varname {
            continue;
        }
        
        // match value
        match split.next() {
            Some(text) => return Some(text.trim().to_owned()),
            None => return None
        }

    }
    None
}

#[allow(dead_code)]
fn get_file_value_int(filename : &str, varname : &str) -> Option<i32> {

    if !Path::new(filename).exists() {
        return None
    }

    for line in read_to_string(filename).unwrap().lines() {

        let mut split = line.splitn(2, "=");

        // match name
        let name = match split.next() {
            Some(text) => text.trim(),
            None => return None
        };

        if name != varname {
            println!("{}", name);
            continue;
        }
        
        // match value
        let value_string = match split.next() {
            Some(text) => text.trim(),
            None => return None
        };

        match value_string.parse::<i32>() {
            Ok(number) => return Some(number),
            Err(_) => return None,
        };

    }
    None
}

fn main() {
    let arguments: Args = Args::parse();
    parse_arguments(arguments)
}
