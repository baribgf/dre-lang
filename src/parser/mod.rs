use lalrpop_util::lalrpop_mod;

lalrpop_mod!(grammar, "/parser/grammar.rs");

pub type Parser = grammar::ProgramParser;
