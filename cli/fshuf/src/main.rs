pub(crate) extern crate rand;
pub(crate) extern crate regex;

pub(crate) mod help;
pub(crate) mod add;
pub(crate) mod rem;
pub(crate) mod prefix;
pub(crate) mod _mod;

use std::env::{ args };
use std::process::{ exit };

use regex::{ Regex };

use help::help;
use add::add;
use rem::rem;
use _mod::_mod;

fn main() {
	let args: Vec<String> = args().collect();
	if args.len() >= 2 {
		if args.len() == 3 {
            if args[1] != "help" {
                let regexp = Regex::new(r#"[db]+"#).unwrap();
			    if !regexp.is_match(&args[2]) {
				    println!("\u{001b}[38;5;1mError:\u{001b}[0m The passed prefix \"{prefix}\" does not match the RegEx \"[db]+\".",
					    prefix = &args[2]);
				    help();
				    exit(3);
                }
            }
			match args[1].as_str() {
				"help" => { println!("\u{001b}[38;5;1mError:\u{001b}[0mHelp does not accept arguments!"); help(); exit(4); }
				"add" => { add(Some(args[2].clone())); exit(0); }
				"rem" => { rem(Some(args[2].clone())); exit(0); }
				"mod" => { _mod(Some(args[2].clone())); exit(0); }
				invalid => {
                    println!("\u{001b}[35;8;1mError:\u{001b}[0m \"{invalid}\" is not a valid COMMAND.",
                        invalid = invalid);
                    help();
                    exit(2);
                }
			}
		} else if args.len() == 2 {
			match args[1].as_str() {
				"help" => { help(); exit(0); }
				"add" => { add(None); exit(0); }
				"rem" => { rem(None); exit(0); }
				"mod" => { _mod(None); exit(0); }
				invalid => {
					println!("\u{001b}[35;8;1mError:\u{001b}[0m \"{invalid}\" is not a valid COMMAND.",
                        invalid = invalid);
                    help();
                    exit(2);
				}
			}
		}
	} else {
		println!("\u{001b}[38;5;1mError:\u{001b}[0m Not enough arguments.");
		help();
		exit(1);
	}
}

