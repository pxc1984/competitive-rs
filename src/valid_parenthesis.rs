fn is_closing_for(prev: char, next: char) -> bool {
	match (prev, next) {
		('(', ')') => true,
		('[', ']') => true,
		('{', '}') => true,
		_ => false,
	}
}

fn is_opening(c: char) -> bool {
	match c {
		'(' => true,
		'[' => true,
		'{' => true,
		_ => false,
	}
}

pub fn is_valid_parenthesis(s: &str) -> bool {
	let mut stack = Vec::with_capacity(s.len());
	for c in s.chars() {
		match is_opening(c) {
			true => {
				stack.push(c);
			}
			false => {
				if let Some(prev) = stack.pop() {
					if !is_closing_for(prev, c) {
						return false;
					}
				} else if stack.len() == 0 {
					return false;
				}
			}
		};
	}
	stack.len() == 0
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn works() {
		let cases = [
			("()", true),
			("[]", true),
			("{}", true),
			("}{", false),
			("{[(]})", false),
			(")", false),
		];

		for (case_input, expected) in cases {
			assert_eq!(is_valid_parenthesis(case_input), expected, "{case_input} should return {expected}")
		}
	}
}