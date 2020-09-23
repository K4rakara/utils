use crate::rand;

use rand::random;

pub(crate) trait PrefixChar {
	fn generate(&self) -> char;
	fn can_output(&self, v: char) -> bool;
}

pub(crate) struct DecimalChar;

impl DecimalChar { pub fn new() -> Self { DecimalChar } }

impl PrefixChar for DecimalChar {
	fn generate(&self) -> char {
		((random::<f32>() * 9.0).round() as u8 + 48) as char
	}
	fn can_output(&self, v: char) -> bool {
		(v as u8) - 47 <= 10
	}
}

#[cfg(test)]
mod test_decimal_char {
	use super::DecimalChar;
	use super::PrefixChar;
	#[test]
	fn test_can_output() {
		let gen = DecimalChar::new();
		for i in 0..9 {
			let as_char = {
				let working = format!("{}", i);
				working.chars().next().unwrap()
			};
			assert!(gen.can_output(as_char));
		}
	}
	#[test]
	fn test_generate() {
		let gen = DecimalChar::new();
		for _ in 0..64 { assert!(gen.can_output(gen.generate())); }
	}
}

pub(crate) struct BinaryChar;

impl BinaryChar { pub fn new() -> Self { BinaryChar } }

impl PrefixChar for BinaryChar {
	fn generate(&self) -> char { if random() { '1' } else { '0' } }
	fn can_output(&self, v: char) -> bool { v == '0' || v == '1' }
}


#[cfg(test)]
mod test_binary_char {
	use super::BinaryChar;
	use super::PrefixChar;
	#[test]
	fn test_can_output() {
		let gen = BinaryChar::new();
		assert!(gen.can_output('0'));
		assert!(gen.can_output('1'));
	}
	#[test]
	fn test_generate() {
		let gen = BinaryChar::new();
		for _ in 0..64 { assert!(gen.can_output(gen.generate())); }
	}
}

pub(crate) struct Prefix ( Vec<Box<dyn PrefixChar>> );

impl Prefix {
	pub fn new() -> Self { Prefix ( Vec::new() ) }
	pub fn new_from(pattern: &String) -> Self {
		let mut to_return = Prefix::new();
		{
			let mut chars = pattern.chars();
			while let Some(this_char) = chars.next() {
				match this_char {
					'd' => { to_return.push(Box::new(DecimalChar::new())); }
					'b' => { to_return.push(Box::new(BinaryChar::new())); }
					e => { panic!(format!("Invalid suffix pattern character \"{}\". This should have been caught earlier on.", e)); }
				}
			}
		}
		to_return
	}
	pub fn push(&mut self, v: Box<dyn PrefixChar>) { self.0.push(v); }
	pub fn generate(&self) -> String {
		let mut to_return = String::new();
		for this_char in self.0.iter() { to_return.push(this_char.generate()); }
		to_return
	}
	pub fn can_output(&self, v: &String) -> bool {
		if v.len() != self.0.len() { return false; }
		let mut chars = v.chars();
		let mut i = 0;
		while let Some(this_char) = chars.next() {
			if !self.0[i].can_output(this_char) { return false; }
			i += 1;
		}
		true
	}
}

#[cfg(test)]
mod test_prefix {
	use super::Prefix;
	#[test]
	fn test_can_output() {
		{
			let prefix = Prefix::new_from(&String::from("dddd"));
			assert!(prefix.can_output(&String::from("0000")));
			assert!(prefix.can_output(&String::from("0099")));
			assert!(prefix.can_output(&String::from("9900")));
			assert_eq!(prefix.can_output(&String::from("aaaa")), false);
		}
		{
			let prefix = Prefix::new_from(&String::from("bbbb"));
			assert!(prefix.can_output(&String::from("0000")));
			assert!(prefix.can_output(&String::from("1111")));
			assert_eq!(prefix.can_output(&String::from("0002")), false);
		}
	}
	#[test]
	fn test_generate() {
		let prefix = Prefix::new_from(&String::from("db"));
		for _ in 0..512 { assert!(prefix.can_output(&prefix.generate())); }
	}
}

