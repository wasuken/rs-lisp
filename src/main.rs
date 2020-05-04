use regex::Regex;
use std::fmt::Debug;
use std::collections::HashMap;

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

#[derive(Clone)]
enum LispExp {
	Bool(bool),
	Symbol(String),
	String(String),
	Number(f64),
	List(Vec<LispExp>),
	Func(fn(&[LispExp]) -> Option<LispExp>),
}

impl Debug for LispExp{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			LispExp::Bool(x) => {
				f.debug_struct("LispExp")
					.field("bool", x)
					.finish()
			},
			LispExp::List(x) => {
				f.debug_struct("LispExp")
					.field("list", x)
					.finish()
			},
			LispExp::Number(x) => {
				f.debug_struct("LispExp")
					.field("number", x)
					.finish()
			},
			LispExp::String(x) => {
				f.debug_struct("LispExp")
					.field("string", x)
					.finish()
			},
			LispExp::Symbol(x) => {
				f.debug_struct("LispExp")
					.field("symbol", x)
					.finish()
			},
			_ => {
				f.debug_struct("unknown")
					.finish()
			},
		}
    }
}

impl PartialEq for LispExp {
	fn eq(&self, other: &Self) -> bool {
        match self {
			LispExp::Bool(x) => x == match other {
				LispExp::Bool(y) => y,
				_ => &false
			},
			LispExp::List(x) => Some(x) == match other {
				LispExp::List(y) => Some(y),
				_ => None
			},
			LispExp::Number(x) => Some(x) == match other {
				LispExp::Number(y) => Some(y),
				_ => None
			},
			LispExp::String(x) => Some(x) == match other {
				LispExp::String(y) => Some(y),
				_ => None
			},
			LispExp::Symbol(x) => Some(x) == match other {
				LispExp::Symbol(y) => Some(y),
				_ => None
			},
			_ => false,
		}
    }
}

fn parser(nodes: &mut Vec<String>) -> LispExp {
	if nodes.len() <= 0 {
		return LispExp::Symbol("nil".to_string());
	}

	let token: String = nodes.remove(0);

	match token {
		_ if token == "(" => {
			let mut l = Vec::new();
			while nodes[0] != ")" {
				l.push(parser(nodes));
			}
			nodes.remove(0);
			return LispExp::List(l);
		},
		_ => {
			return match token {
				_ if Regex::new(r"^[0-9|\.]+").unwrap().is_match(&token) =>
					LispExp::Number(token.parse::<f64>().unwrap()),
				_ if Regex::new("^\"").unwrap().is_match(&token) =>
					LispExp::String(token.to_string()),
				_ => LispExp::Symbol(token.to_string()),
			};
		}
	}
}

struct LispEnv {
	variables: HashMap<String, LispExp>
}

fn default_env(env: &mut LispEnv) {
	// 四則演算

	// 配列系

	// 数学系
}

fn semantic_analysis(node: LispExp, env: &mut LispEnv) -> LispExp {
	match node {
		LispExp::Bool(_) | LispExp::Number(_) | LispExp::String(_) => node,
		LispExp::Symbol(x) => match x {
			_ if Regex::new(r"^[0-9|\.]+").unwrap().is_match(&x) =>
				LispExp::Number(x.parse::<f64>().unwrap()),
			_ if Regex::new("^\"").unwrap().is_match(&x) =>
				LispExp::String(x),
			// error
			_ => LispExp::Symbol(x),
		},
	}
}

fn eval(s: &str){
	// 環境変数
	let mut env = &mut LispEnv{variables: Vec::new()};
	default_env(env);
	// 字句解析
	let mut nodes = &mut lexer(s);
	// 構文解析
	let parse = parser(nodes);
	// 意味解析
	// semantic_analysis(parse, env);
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
	#[test]
	fn parser_test(){
		let mut env = &mut LispEnv{variables: Vec::new()};
		default_env(env);
		let result = parser(&mut lexer("(+ 1 2)"));
		let expect = LispExp::List(vec![LispExp::Symbol("+".to_string()),
										LispExp::Number(1 as f64),
										LispExp::Number(2 as f64)]);
		assert_eq!(expect, result);

		let result2 = parser(&mut lexer("(+ (* 2 3) (/ 10 2))"));
		let expect2 = LispExp::List(vec![LispExp::Symbol("+".to_string()),
										 LispExp::List(vec![LispExp::Symbol("*".to_string()),
															LispExp::Number(2 as f64),
															LispExp::Number(3 as f64)]),
										 LispExp::List(vec![LispExp::Symbol("/".to_string()),
															LispExp::Number(10 as f64),
															LispExp::Number(2 as f64)])]);
		assert_eq!(expect2, result2);
	}
}
