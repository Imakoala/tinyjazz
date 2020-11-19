use std::rc::Rc;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

use crate::{
    compute_consts::ComputeConstError,
    expand_fn::{ExpandFnError, STACK_SIZE},
    parser_wrapper::{ParseErrorType, ParserError},
};

pub enum ErrorType {
    Parser(ParserError),
    ComputeConst(ComputeConstError),
    ExpandFn(ExpandFnError),
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
                    name.value, STACK_SIZE
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
