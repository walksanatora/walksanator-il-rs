#![allow(dead_code)]

pub fn parse(string: &str) -> Vec<String> {
	let mut output = vec![];
	for line in string.split('\n') {
		let mut flag_comment = false;
		for word in shlex::split(line).unwrap() {
			if ! word.starts_with('#') && ! flag_comment {
				output.push(word)
			} else {flag_comment = true}
		}
		output.push("\n".to_string());
	}
	output
}

pub fn combine(words: Vec<String>) -> String {
	let mut accu = Vec::new();
	let mut temp:Vec<String> = Vec::new();

	//combine by newlines
	for word in words {
		if word == "\n" {
			accu.push(temp.clone());
			temp.drain(0..temp.len());
		} else {
			temp.push(word);
		}
	}
	//join each word in lines by space
	let mut lines = Vec::new();
	for line in accu {
		lines.push(
			line.join(" ")
		)
	}
	//remove blank lines
	let mut output = Vec::new();
	for line in lines {
		if !line.is_empty() {
			output.push(line);
		}
	}

	output.join("\n")
}

#[cfg(test)]
mod test {
	#![allow(dead_code)]

	use super::parse;

	const EXAMPLE_SCRIPT: &str = include_str!("../hello.ils");
	const EXAMPLE_PARSED: [&str;61] = ["IF", "OP", "0", "VAR", "std", "\n", "IMPORT", "std", "\n", "ES", "\n", "CALL", "print", "STR", "Hello World!", "\n", "\n", "\n", "DEF", "print_times", "STR", "str", "times", "\n", "LBL", "lp", "\n", "CALLN", "print", "VAR", "str", "\n", "MATH", "1", "times", "1", "\n", "IF", "OP", "0", "OP", "2", "times", "0", "\n", "GOTO", "STR", "lp", "\n", "ES", "\n", "ES", "\n", "\n", "CALL", "print_times", "Hello World!", "10", "\n", "EOF", "\n"];
	#[test]
	fn sanity(){
		assert_eq!(1,1)
	}
	#[test]
	fn parsing() {
		let p = parse(EXAMPLE_SCRIPT);
		let mut string_parsed = vec![];
		for w in EXAMPLE_PARSED.into_iter() {
			string_parsed.push(w.to_string())
		};
		assert_eq!(
			p,string_parsed
		)
	}

}