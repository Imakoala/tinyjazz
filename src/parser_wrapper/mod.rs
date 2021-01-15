pub mod parser;
use std::path::PathBuf;
use std::{fs::read_to_string, rc::Rc};

use crate::ast::parse_ast::*;
use ahash::AHashMap;
use lalrpop_util::lexer::Token;

use self::parser::ProgramParser;
use codespan_reporting::files::SimpleFiles;
#[derive(Debug)]
pub enum ParseErrorType {
    Syntax(SyntaxError),
    UnexpectedEOF(usize),
    Other(String),
}
#[derive(Debug)]
pub struct SyntaxError {
    pub l: usize,
    pub r: usize,
    pub token: String,
    pub expected: String,
}

fn replace_regex(input: &mut Vec<String>) {
    for s in input {
        let new_str = match &*s {
            s if *s == r###"r#"[a-zA-Z_][a-zA-Z_0-9]*"#"###.to_string() => {
                Some("Variable name".to_string())
            }
            s if *s == r###"r#"[0-9]+"#"###.to_string() => Some("Number".to_string()),
            s if *s == r###"r#"[a-zA-Z_][a-zA-Z_0-9]*<"#"###.to_string() => Some(
                "Function name + \"<\" (without a space between the name and \"<\") ".to_string(),
            ),
            s if *s == r###"r#"[a-zA-Z_][a-zA-Z_0-9]*\\("#"###.to_string() => Some(
                "Function name + \"(\" (without a space between the name and \"(\") ".to_string(),
            ),
            s if *s == r###"r#"import[ ]+\"[^/\\n\"]*(/[^/\\n\"]*)*\""#"###.to_string()
                || *s == r###"r#"import[ ]+[^/\\n \"]*(/[^/\\n \"]*)*"#"### =>
            {
                Some("Import".to_string())
            }
            _ => None,
        };
        if let Some(replace) = new_str {
            *s = replace;
        }
    }
}

impl From<lalrpop_util::ParseError<usize, Token<'_>, &'static str>> for ParseErrorType {
    fn from(error: lalrpop_util::ParseError<usize, Token, &'static str>) -> Self {
        match error {
            lalrpop_util::ParseError::InvalidToken { location } => {
                ParseErrorType::Syntax(SyntaxError {
                    l: location,
                    r: location,
                    token: "unknown token".to_string(),
                    expected: String::new(),
                })
            }
            lalrpop_util::ParseError::UnrecognizedEOF {
                /// The end of the final token
                location,

                /// The set of expected tokens: these names are taken from the
                /// grammar and hence may not necessarily be suitable for
                /// presenting to the user.
                    expected: _,
            } => ParseErrorType::UnexpectedEOF(location),
            lalrpop_util::ParseError::UnrecognizedToken {
                /// The unexpected token of type `T` with a span given by the two `L` values.
                token,

                /// The set of expected tokens: these names are taken from the
                /// grammar and hence may not necessarily be suitable for
                /// presenting to the user.
                mut expected,
            } => {
                let (start, token, end) = token;
                ParseErrorType::Syntax(SyntaxError {
                    l: start,
                    token: token.1.to_string(),
                    r: end,
                    expected: {
                        replace_regex(&mut expected);
                        expected.join(" \n ")
                    },
                })
            }
            lalrpop_util::ParseError::ExtraToken { token } => {
                let (start, token, end) = token;
                ParseErrorType::Syntax(SyntaxError {
                    l: start,
                    token: token.1.to_string(),
                    r: end,
                    expected: "nothing".to_string(),
                })
            }
            lalrpop_util::ParseError::User { error } => ParseErrorType::Other(error.to_string()),
        }
    }
}
#[derive(Debug)]
pub struct FileError {
    pub file: PathBuf,
    pub error: String,
}

impl From<(PathBuf, std::io::Error)> for FileError {
    fn from(error: (PathBuf, std::io::Error)) -> Self {
        FileError {
            file: error.0,
            error: format!("{}", error.1),
        }
    }
}
#[derive(Debug)]
pub enum ParserError {
    File(FileError),
    Parse(usize, ParseErrorType),
}

impl From<FileError> for ParserError {
    fn from(err: FileError) -> Self {
        ParserError::File(err)
    }
}

impl From<(usize, ParseErrorType)> for ParserError {
    fn from(err: (usize, ParseErrorType)) -> Self {
        ParserError::Parse(err.0, err.1)
    }
}
//horrible implementation with way too many clones, but it should still be fast enough for any use case.
//this function parse the main file and every import.
pub fn parse(
    main_path: PathBuf,
) -> Result<
    (Program, Rc<SimpleFiles<String, String>>),
    (ParserError, Rc<SimpleFiles<String, String>>),
> {
    let mut files = SimpleFiles::new();
    let mut prog_map = AHashMap::<PathBuf, Program>::new();
    let mut queue = Vec::<PathBuf>::new();
    queue.push(main_path.clone());
    while !queue.is_empty() {
        let path = queue.pop().unwrap();
        if prog_map.contains_key(&path) {
            continue;
        }
        let file = read_to_string(path.clone()).map_err(|e| {
            (
                FileError {
                    file: path.clone(),
                    error: format!("{}", e),
                }
                .into(),
                Rc::new(files.clone()),
            )
        })?;
        let file_id = files.add(path.to_string_lossy().to_string(), file.clone());
        let mut prog = ProgramParser::new()
            .parse(file_id, &file)
            .map_err(|e| ((file_id, e.into()).into(), Rc::new(files.clone())))?;
        let imports = std::mem::take(&mut prog.imports);
        prog_map.insert(path.clone(), prog);
        let mut root_path = path.clone();
        root_path.pop();
        for path_end in imports {
            let mut new_path = root_path.clone();
            new_path.push(path_end);
            if !prog_map.contains_key(&new_path) {
                queue.push(new_path)
            }
        }
    }
    let mut main_program = prog_map.remove(&main_path).unwrap();
    for (_, prog) in prog_map {
        for (name, func) in prog.functions {
            main_program.functions.insert(name, func);
        }
        for (name, module) in prog.modules {
            main_program.modules.insert(name, module);
        }
        for (name, cons) in prog.global_consts {
            main_program.global_consts.insert(name, cons);
        }
    }
    Ok((main_program, Rc::new(files)))
}

#[cfg(test)]
mod tests {
    use super::parse;
    use super::parser::ProgramParser;
    use std::fs::read_to_string;
    #[test]
    fn test_parser() {
        let file = read_to_string("src/tests/parser/pass/test.tj").unwrap();
        let prog = ProgramParser::new().parse(0, &file);
        //println!("{:#?}", prog);
        assert!(prog.is_ok());
    }
    #[test]
    fn test_import_fail() {
        let file = parse("src/tests/parser/pass/test.tj".into());
        //println!("{:#?}", file);
        assert!(file.is_err());
    }
    #[test]
    fn test_import_pass() {
        let file = parse("src/tests/parser/pass/import.tj".into());
        //println!("{:#?}", file);
        assert!(file.is_ok());
    }
    #[test]
    fn test_fail() {
        let file = read_to_string("src/tests/parser/fail/test.tj").unwrap();
        let prog = ProgramParser::new().parse(0, &file);
        //println!("{:#?}", prog);
        assert!(prog.is_err());
    }
}
