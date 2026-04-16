mod utils;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(grammar, "/parser/grammar.rs");

pub type Parser = grammar::ProgramParser;

#[cfg(test)]
mod tests {
	use super::Parser;
	use crate::lexer::Lexer;

	fn parse_source(source: &str) -> Result<crate::ast::Program, lalrpop_util::ParseError<usize, crate::token::Token, String>> {
		let source_path = "parser_test.dr".to_string();
		let lexer = Lexer::new(&source_path, source.as_bytes());
		Parser::new().parse(lexer)
	}

	#[test]
	fn parses_empty_program() {
		let res = parse_source("");
		assert!(res.is_ok());
	}

	#[test]
	fn parses_variable_declarations_and_constants() {
		let src = "let value: Int\nlet const AGE: Int = 21\n";
		parse_source(src).expect("program should parse");
	}

	#[test]
	fn parses_enum_definition() {
		let src = "let Color: enum\nbegin\n    Red = 0\n    Green = 1\n    Blue\nend\n";
		parse_source(src).expect("program should parse");
	}

	#[test]
	fn parses_protocol_with_decl_and_default_impl() {
		let src = concat!(
			"let Printable: prot\n",
			"begin\n",
			"    let print: func () -> Int\n",
			"    let defaultPrint: func () -> Int\n",
			"    begin\n",
			"        return 1\n",
			"    end\n",
			"end\n"
		);

		parse_source(src).expect("program should parse");
	}

	#[test]
	fn parses_struct_with_protocol_impositions_and_methods() {
		let src = concat!(
			"let Point: struct is Printable, Hashable\n",
			"begin\n",
			"    let x: Int\n",
			"    let y: Int\n",
			"\n",
			"    let magnitude: func () -> Int\n",
			"    begin\n",
			"        return self.x\n",
			"    end\n",
			"end\n"
		);

		parse_source(src).expect("program should parse");
	}

	#[test]
	fn parses_function_with_complex_expression_tree() {
		let src = concat!(
			"let calc: func (a: Int, b: Int) -> Int\n",
			"begin\n",
			"    let x: Int <- a + b * 4\n",
			"    let y: Int <- (x << 1) & 7\n",
			"    return y\n",
			"end\n"
		);

		parse_source(src).expect("program should parse");
	}

	#[test]
	fn parses_control_flow_statements() {
		let src = concat!(
			"let main: func () -> Int\n",
			"begin\n",
			"    let x: Int <- 0\n",
			"    until x = 3 do\n",
			"        inc x\n",
			"    end\n",
			"    while x <> 6 do\n",
			"        x <- x + 1\n",
			"    end\n",
			"    for i in 1..x do\n",
			"        dec x\n",
			"    end\n",
			"    if x = 0 then\n",
			"        x <- 1\n",
			"    else if x = 1 then\n",
			"        x <- 2\n",
			"    else\n",
			"        x <- 3\n",
			"    end\n",
			"    return x\n",
			"end\n"
		);

		parse_source(src).expect("program should parse");
	}

	#[test]
	fn parses_calls_indexing_and_member_access() {
		let src = concat!(
			"let main: func () -> Int\n",
			"begin\n",
			"    let arr: [2 Int]\n",
			"    arr[0] <- Module::build().items[1]\n",
			"    Module::log(arr[0], self.value)\n",
			"    return 0\n",
			"end\n"
		);

		parse_source(src).expect("program should parse");
	}

	#[test]
	fn parse_fails_on_missing_function_end() {
		let src = "let main: func () -> Int\nbegin\n    return 0\n";
		let res = parse_source(src);
		assert!(res.is_err());
	}

	#[test]
	fn parse_fails_on_unexpected_token_in_statement() {
		let src = "let main: func () -> Int\nbegin\n    return )\nend\n";
		let res = parse_source(src);
		assert!(res.is_err());
	}

	#[test]
	fn parse_fails_on_malformed_if_then_block() {
		let src = concat!(
			"let main: func () -> Int\n",
			"begin\n",
			"    if true then\n",
			"        return 1\n",
			"    else\n",
			"        return 0\n",
			"end\n"
		);

		let res = parse_source(src);
		assert!(res.is_err());
	}
}
