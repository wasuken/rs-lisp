use regex::Regex;
fn is_empty_char(c: char) -> bool {
	return c == ' ' || c == '\n' || c == '\t'
}

fn lexer(s: &str) -> Vec<String> {
	let s_chrs:Vec<char> = s.chars().collect();
	let mut stack: String = "".to_string();
	let len = s_chrs.len();
	let mut cur_pos: usize = 0;
	let mut result: Vec<String> = Vec::new();
	let paren_re = Regex::new(r"\(|\)").unwrap();

	while cur_pos < len {
		if !is_empty_char(s_chrs[cur_pos]) {
			if s_chrs[cur_pos] == '"' {
				// 最初の"を追加
				stack = format!("{}{}", stack, s_chrs[cur_pos]);
				cur_pos += 1;

				while cur_pos < len && s_chrs[cur_pos] != '"' {
					stack = format!("{}{}", stack, s_chrs[cur_pos]);
					cur_pos += 1;
				}

				// 最後の"を追加
				stack = format!("{}{}", stack, s_chrs[cur_pos]);
				cur_pos += 1;
				result.push(stack.to_string());
				stack = "".to_string();
			}
			while cur_pos < len && !is_empty_char(s_chrs[cur_pos]) {
				if paren_re.is_match(&s_chrs[cur_pos].to_string()) {
					if !stack.is_empty() {
						result.push(stack.to_string());
						stack = "".to_string();
					}
					stack = format!("{}", &s_chrs[cur_pos].to_string());
					break
				}
				stack = format!("{}{}", stack, s_chrs[cur_pos]);
				cur_pos += 1;
			}
			if !stack.is_empty() {
				result.push(stack.to_string());
				stack = "".to_string();
			}
		}
		cur_pos += 1;
	}

	return result
}

enum TokenType {
	LPAREN,
	RPAREN,
	VARIABLE,
	NUMBER,
	STRING,
	UNKNOWN,
}

struct Token {
	token_type: TokenType,
	value: String,
	children: Vec<Token>,
}

fn parser(nodes: &Vec<String>) -> Vec<Token> {
	let mut result = Vec::new();
	for x in nodes {
		let token_type = match x {
			x if Regex::new(r"^\(").unwrap().is_match(x) => TokenType::LPAREN,
			x if Regex::new(r"^\)").unwrap().is_match(x) => TokenType::RPAREN,
			x if Regex::new(r"^[0-9|\.]+").unwrap().is_match(x) => TokenType::NUMBER,
			x if Regex::new(r"^[a-z|A-Z|\-|_]|]+").unwrap().is_match(x) => TokenType::VARIABLE,
			x if Regex::new("^\"").unwrap().is_match(x) => TokenType::STRING,
			_ => TokenType::UNKNOWN
		};
		result.push(Token{
			token_type: token_type,
			value: x.to_string(),
			children: Vec::new()
		});
	}
	return result
}

fn eval(s: &str){
	// 字句解析
	// let nodes = lexer(s);
	// 構文解析
	// parser(&nodes);
	// 意味解析
	// semantic_analysis();
}

fn main() {
    loop {
		print!(">> ");
		let mut s = String::new();
		std::io::stdin().read_line(&mut s).ok();
		println!();
		eval(&s);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn lexer_test(){
		// 単純な四則演算テスト
		let result = lexer("(+ 1 2)");
		let expect = vec!["(", "+", "1", "2", ")"];
		assert_eq!(expect, result);
		// 二重括弧の四則演算テスト
		let result2 = lexer("(+ (* 2 2) (/ 4 2))");
		let expect2 = vec!["(", "+", "(", "*", "2", "2", ")", "(", "/", "4", "2", ")", ")"];
		assert_eq!(expect2, result2);
		// lispっぽいやつ
		let result3 = lexer("(expt 2 2)");
		let expect3 = vec!["(", "expt", "2", "2", ")"];
		assert_eq!(expect3, result3);
		// lispっぽいやつ(文字列)
		let result4 = lexer("(print \"hoge fuga \")");
		let expect4 = vec!["(", "print", "\"hoge fuga \"", ")"];
		assert_eq!(expect4, result4);
	}
}
