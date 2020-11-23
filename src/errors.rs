use crate::{
    compute_consts::ComputeConstError,
    expand_fn::{ExpandFnError, REC_DEPTH},
    parser_wrapper::{ParseErrorType, ParserError},
    typing::TypingError,
};
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use std::rc::Rc;

/*
This file is dedicated to the handling of all errors, to pretty_print them using codespan_diagnostic, among others.
It is a bit verbose, but provides very clear error messages
*/
pub enum ErrorType {
    Parser(ParserError),
    ComputeConst(ComputeConstError),
    ExpandFn(ExpandFnError),
    Typing(TypingError),
}

fn get_diagnostic(
    error_type: &ErrorType,
    files: Rc<SimpleFiles<String, String>>,
) -> Diagnostic<usize> {
    match error_type {
        ErrorType::Parser(err) => match err {
            ParserError::File(file_error) => Diagnostic::error()
                .with_message("File Error")
                .with_code("E0000")
                .with_message(format!(
                    "File {}",
                    file_error.file.to_string_lossy().to_string()
                ))
                .with_notes(vec![format!("{}", file_error.error,)]),
            ParserError::Parse(file_id, parser_error) => match parser_error {
                ParseErrorType::Syntax(syntaxe_error) => Diagnostic::error()
                    .with_message("Syntaxe Error")
                    .with_code("E0001")
                    .with_labels(vec![Label::primary(
                        *file_id,
                        syntaxe_error.l..syntaxe_error.r,
                    )
                    .with_message(format!("Unexpected token {}", syntaxe_error.token))])
                    .with_notes(vec![format!(
                        "Expected one of the following tokens : \n{}",
                        syntaxe_error.expected,
                    )]),
                ParseErrorType::UnexpectedEOF(l) => Diagnostic::error()
                    .with_message("Error : Unexpected EOF (unclosed parenthesis?)")
                    .with_code("E0006")
                    .with_labels(vec![Label::primary(*file_id, *l..*l)]),
                ParseErrorType::Other(s) => Diagnostic::error()
                    .with_message(format!("Error : {}", s))
                    .with_code("E0004"),
            },
        },
        ErrorType::ComputeConst(const_error) => match const_error {
            ComputeConstError::UnknowVariable((file_id, l, r), var) => Diagnostic::error()
                .with_message("Error : unknow variable")
                .with_code("E0002")
                .with_labels(vec![Label::primary(*file_id, *l..*r)
                    .with_message(format!("Unknown variable {}", var))]),
            ComputeConstError::DivisionByZero((file_id, l, r)) => Diagnostic::error()
                .with_message("Error : division by zero")
                .with_code("E0002")
                .with_labels(vec![Label::primary(*file_id, *l..*r)
                    .with_message(format!("This evaluates to zero"))]),
            ComputeConstError::CyclicDefinition => Diagnostic::error()
                .with_message("Error : cyclic constant definition")
                .with_code("E0003"),
            ComputeConstError::Other(s) => Diagnostic::error()
                .with_message(format!("Error : {}", s))
                .with_code("E0004"),
        },
        ErrorType::ExpandFn(fn_error) => match fn_error {
            ExpandFnError::StackOverflow(name) => Diagnostic::error()
                .with_message("Error : stack overflow")
                .with_code("E0007")
                .with_labels(vec![Label::primary(name.loc.0, name.loc.1..name.loc.2)])
                .with_message(format!(
                    "Function {} was expanded recursively more than {} times without finishing.",
                    name.value, REC_DEPTH
                )),
            ExpandFnError::WrongNumber(typ, (file_id, l, r), _name, expected, got) => {
                Diagnostic::error()
                    .with_message(format!("Error : wrong number of {}", typ))
                    .with_code("E0008")
                    .with_labels(vec![Label::primary(*file_id, *l..*r)])
                    .with_message(format!("Expected {} {}, got {}", expected, typ, got))
            }
            ExpandFnError::ReplaceConstError(const_error) => {
                get_diagnostic(&ErrorType::ComputeConst(const_error.clone()), files)
            }
            ExpandFnError::UnknowFunction((file_id, l, r), name) => Diagnostic::error()
                .with_message("Error : unknown function")
                .with_code("E0009")
                .with_labels(vec![Label::primary(*file_id, *l..*r)])
                .with_message(format!("Unknown function {}", name)),
        },
        ErrorType::Typing(err) => match err {
            TypingError::NegativeSizeBus((file_id, l, r), i) => Diagnostic::error()
                .with_message("Error : negative length bus")
                .with_code("E0010")
                .with_labels(vec![Label::primary(*file_id, *l..*r)])
                .with_message(format!(
                    "Bus must have a positive length, got {} instead",
                    i
                )),
            TypingError::MismatchedBusSize(token1, token2) => {
                let message1;
                if let Some(name) = &token1.name {
                    message1 = format!("The variable {} has length {}", name, token1.length)
                } else {
                    message1 = format!("This expression has length {}", token1.length)
                }
                let message2;
                if let Some(name) = &token2.name {
                    message2 = format!("The variable {} has length {}", name, token2.length)
                } else {
                    message2 = format!("This expression has length {}", token2.length)
                }
                Diagnostic::error()
                    .with_message("Error : msimatched bus lengths")
                    .with_code("E0011")
                    .with_labels(vec![
                        Label::primary(token1.loc.0, token1.loc.1..token1.loc.2)
                            .with_message(message1),
                        Label::primary(token2.loc.0, token2.loc.1..token2.loc.2)
                            .with_message(message2),
                    ])
                    .with_message("These variables must have the same length ")
            }
            TypingError::UnknownVar(name, loc) => Diagnostic::error()
                .with_message("Error : unknown variable")
                .with_code("E0012")
                .with_labels(vec![Label::primary(loc.0, loc.1..loc.2)])
                .with_message(format!("Unknown variable {}", name)),
            TypingError::DuplicateVar(name, loc1, loc2) => Diagnostic::error()
                .with_message("Error : duplicate shared variable")
                .with_code("E0013")
                .with_labels(vec![
                    Label::primary(loc1.0, loc1.1..loc1.2),
                    Label::primary(loc2.0, loc2.1..loc2.2),
                ])
                .with_message(format!("Duplicate shared variable {}", name)),
            TypingError::UnknownModule(name, loc) => Diagnostic::error()
                .with_message("Error : unknown module")
                .with_code("E0014")
                .with_labels(vec![Label::primary(loc.0, loc.1..loc.2)])
                .with_message(format!("Unknown module {}", name)),
            TypingError::UnknownNode(name, loc) => Diagnostic::error()
                .with_message("Error : unknown node")
                .with_code("E0015")
                .with_labels(vec![Label::primary(loc.0, loc.1..loc.2)])
                .with_message(format!("Unknown node {}", name)),
            TypingError::WrongNumber(typ, loc, expected, got) => Diagnostic::error()
                .with_message(format!("Error : wrong number of {}", typ))
                .with_code("E0016")
                .with_labels(vec![Label::primary(loc.0, loc.1..loc.2)])
                .with_message(format!(
                    "Wrong number of {}:expected {}, got {}",
                    typ, expected, got
                )),
            TypingError::ExpectedSizeOne(loc, n) => Diagnostic::error()
                .with_message("Error : expected us of length 1")
                .with_code("E0017")
                .with_labels(vec![Label::primary(loc.0, loc.1..loc.2)])
                .with_message(format!(
                    "Expected variable of length 1 (single bit), instead, got bus of size {}",
                    n
                )),
            TypingError::IndexOutOfRange(loc, got, len) => Diagnostic::error()
                .with_message("Error : index out of range")
                .with_code("E0018")
                .with_labels(vec![Label::primary(loc.0, loc.1..loc.2)])
                .with_message(format!(
                    "Index out of range : index was {} for a bus of length {}",
                    got, len
                )),
        },
    }
}

pub struct TinyjazzError {
    error: ErrorType,
    files: Rc<SimpleFiles<String, String>>,
}
impl TinyjazzError {
    pub fn print(&self) -> std::fmt::Result {
        let diagnostic = get_diagnostic(&self.error, self.files.clone());
        let config = codespan_reporting::term::Config::default();
        let mut writer = StandardStream::stderr(ColorChoice::Always);
        codespan_reporting::term::emit(&mut writer, &config, &*self.files, &diagnostic).unwrap();
        Ok(())
    }
}

impl From<(ParserError, Rc<SimpleFiles<String, String>>)> for TinyjazzError {
    fn from(err: (ParserError, Rc<SimpleFiles<String, String>>)) -> Self {
        let (parse_error, files) = err;
        TinyjazzError {
            error: ErrorType::Parser(parse_error),
            files,
        }
    }
}
impl From<(ComputeConstError, Rc<SimpleFiles<String, String>>)> for TinyjazzError {
    fn from(err: (ComputeConstError, Rc<SimpleFiles<String, String>>)) -> Self {
        let (const_error, files) = err;
        TinyjazzError {
            error: ErrorType::ComputeConst(const_error),
            files,
        }
    }
}

impl From<(ExpandFnError, Rc<SimpleFiles<String, String>>)> for TinyjazzError {
    fn from(err: (ExpandFnError, Rc<SimpleFiles<String, String>>)) -> Self {
        let (fn_error, files) = err;
        TinyjazzError {
            error: ErrorType::ExpandFn(fn_error),
            files,
        }
    }
}

impl From<(TypingError, Rc<SimpleFiles<String, String>>)> for TinyjazzError {
    fn from(err: (TypingError, Rc<SimpleFiles<String, String>>)) -> Self {
        let (typ_error, files) = err;
        TinyjazzError {
            error: ErrorType::Typing(typ_error),
            files,
        }
    }
}
