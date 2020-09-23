static HELP: &'static str = include_str!("./.help.clml");

pub(crate) fn help() {
	println!("{}", HELP.replace("\\u{000d}", "\u{000d}"));
}

