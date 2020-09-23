extern crate clml_rs;
use std::fs::{ read_to_string, write };
use clml_rs::{ clml };

fn main() {
    let input = match read_to_string("./src/help.clml") {
        Ok(v) => v,
        Err(e) => panic!(format!("Failed to read ./src/help.clml: {}", e)),
    };
    let output = clml(&input);
    match write("./src/.help.clml", output) {
        Ok(_) => (),
        Err(e) => panic!(format!("Failed to write ./src/.help.clml: {}", e)),
    }
}

