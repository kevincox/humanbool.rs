use std::env;

#[derive(Debug,PartialEq)]
pub enum Error { Empty, Unknown }

/** Parse a human config string as a boolean.
	
	```rust
	use humanbool::parse;
	assert!(parse("y") == Ok(true));
	```
	
	Currently the following are supported:
	- 1/0
	- yes/no/y/n
	- true/false/t/f
	- on/off
 */
pub fn parse(s: &str) -> Result<bool, Error> {
	match s {
		"1" | "y" | "yes" | "on" | "t" | "true" => Ok(true),
		"0" | "n" | "no" | "off" | "f" | "false" => Ok(false),
		"" => Err(Error::Empty),
		_ => Err(Error::Unknown),
	}
}

/** Parse a setting from the environment.
	
	```rust
	use humanbool::*;
	assert_eq!(env("ENABLE_KITTENS", "f"), Ok(false));
	std::env::set_var("ENABLE_KITTENS", "1");
	assert!(env("ENABLE_KITTENS", "f") == Ok(true));
	
	assert!(env("ENABLE_TURBO", "") == Err(Error::Empty));
	```
 */
pub fn env(k: &str, default: &str) -> Result<bool, Error> {
	match env::var(k) {
		Ok(s) => parse(&s),
		Err(env::VarError::NotPresent) => parse(default),
		Err(env::VarError::NotUnicode(_)) => {
			panic!("The environemnt variable {:?} isn't valid UTF8", k)
		},
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn yn() {
		assert!(parse("y") == Ok(true));
		assert!(parse("n") == Ok(false));
	}
	
	#[test]
	fn tf() {
		assert!(parse("t") == Ok(true));
		assert!(parse("f") == Ok(false));
	}
	
	#[test]
	fn _10() {
		assert!(parse("1") == Ok(true));
		assert!(parse("0") == Ok(false));
	}
	
	#[test]
	fn yesno() {
		assert!(parse("yes") == Ok(true));
		assert!(parse("no") == Ok(false));
	}
	
	#[test]
	fn onoff() {
		assert!(parse("on") == Ok(true));
		assert!(parse("off") == Ok(false));
	}
	
	#[test]
	fn unknown() {
		assert!(parse("foo") == Err(Error::Unknown));
		assert!(parse("bar") == Err(Error::Unknown));
		assert!(parse("ye") == Err(Error::Unknown));
		assert!(parse("noway") == Err(Error::Unknown));
	}
	
	#[test]
	fn empty() {
		assert!(parse("") == Err(Error::Empty));
	}
}
