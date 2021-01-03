extern crate clap;
extern crate rand;

use std::error::{ Error };
use std::fmt::{ self };
use std::fs::{ self, DirEntry };
use std::io::{ self };
use std::path::{ Path };
use std::process::{ self };

use clap::{ App, SubCommand };
use rand::{ thread_rng, prelude::{ * } };

fn entries<P: AsRef<Path>>(p: P) -> Result<Vec<DirEntry>, io::Error> {
    let mut to_return = Vec::new();
    for entry in fs::read_dir(p.as_ref())? {
        let entry = entry?;
        to_return.push(entry);
    }
    Ok(to_return)
}

fn warn<E>(e: E)
where
    E: fmt::Display
{
    eprintln!("\u{001b}[38;5;3mWarning:\u{001b}[38;5;0m {}", e);
}

fn error<E>(e: E) -> !
where
    E: Error,
{
    eprintln!("\u{001b}[38;5;1mError:\u{001b}[38;5;0m {}", e);
    process::exit(1);
}

fn add() -> Result<(), io::Error> {
    if Path::new("./.fshuf").exists() { warn("Shuffling already shuffled files."); }
    
    let mut rng = thread_rng();
    
    for entry in entries(".")? {
        fs::rename(
            entry.file_name(),
            &format!("{:04}-{}",
                (rng.gen::<f32>() * 9999.0).round() as u32,
                entry.file_name().to_string_lossy()))?;
    }
    
    fs::write("./.fshuf", "")?;
    
    Ok(())
}

fn rem() -> Result<(), io::Error> {
    if !Path::new("./.fshuf").exists() { warn("Unshuffling already unshuffled files."); }
    
    let entries = entries(".")?;
    
    let filtered = entries
        .iter()
        .filter(|entry| entry.file_name() != ".fshuf")
        .filter(|entry| match entry.file_name().to_string_lossy().get(0..4) {
            Some(slice) => slice.parse::<u32>().is_ok(),
            None => false,
        });
    
    for entry in filtered {
        fs::rename(
            entry.file_name(),
            entry.file_name().to_string_lossy().get(5..).unwrap())?;
    }
    
    fs::remove_file("./.fshuf")?;
    
    Ok(())
}

fn main() {
    let matches = App::new("fshuf")
        .author("Jack Johannesen")
        .version("3.0.0")
        .subcommand(SubCommand::with_name("add"))
        .subcommand(SubCommand::with_name("rem"))
        .get_matches();
    
    match matches.subcommand {
        Some(subcommand) => match subcommand.name.as_str() {
            "add" => match add() {
                Ok(_) => (),
                Err(e) => error(e),
            },
            "rem" => match rem() {
                Ok(_) => (),
                Err(e) => error(e),
            },
            _ => (),
        },
        None => if !Path::new("./.fshuf").exists() {
            match add() {
                Ok(_) => (),
                Err(e) => error(e),
            }
        } else {
            match rem() {
                Ok(_) => (),
                Err(e) => error(e),
            }
        },
    }
}

