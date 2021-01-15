use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use std::rc::Rc;

use crate::frontend::{
    constants::ComputeConstError,
    functions::{ExpandFnError, REC_DEPTH},
    hierarchical_automata::CollapseAutomataError,
    nested_expr::FlattenError,
    parser_wrapper::{ParseErrorType, ParserError},
    scheduler::ScheduleError,
    typing::TypingError,
};

/*
This file is dedicated to the handling of all errors, to pretty print them using codespan_diagnostic.
It is a bit verbose, but provides very clear error messages
*/
pub enum ErrorType {
    Parser(ParserError),
    ComputeConst(ComputeConstError),
    ExpandFn(ExpandFnError),
    Typing(TypingError),
    ColAutomata(CollapseAutomataError),
    FlattenError(FlattenError),
    ScheduleError(ScheduleError),
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
                    .with_message("Syntax Error")
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
                .with_message("Error : duplicate variable")
                .with_code("E0013")
                .with_labels(vec![
                    Label::primary(loc1.0, loc1.1..loc1.2),
                    Label::primary(loc2.0, loc2.1..loc2.2),
                ])
                .with_message(format!("Duplicate variable {}", name)),
            TypingError::UnknownNode(name, loc) => Diagnostic::error()
                .with_message("Error : unknown node")
                .with_code("E0015")
                .with_labels(vec![Label::primary(loc.0, loc.1..loc.2)])
                .with_message(format!("Unknown node {}", name)),
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
                    "Index out of range : index is {} for a bus of length {}",
                    got, len
                )),
            TypingError::LocalVarInUnless(loc, name) => Diagnostic::error()
                .with_message("Error : local var in strong transition")
                .with_code("E0021")
                .with_labels(vec![Label::primary(loc.0, loc.1..loc.2)])
                .with_message(format!("Local var {} in strong transition", name)),
            TypingError::ConflictingNodeShared(loc1, name, loc2) => Diagnostic::error()
                .with_message("Error : Conflicting node name and shared variable name")
                .with_code("E0025")
                .with_labels(vec![
                    Label::primary(loc1.0, loc1.1..loc1.2),
                    Label::primary(loc2.0, loc2.1..loc2.2),
                ])
                .with_message(format!(
                    "A node and a shared variable cannot have the same name {}",
                    name
                )),
            TypingError::NonSharedInLast(loc, name) => Diagnostic::error()
                .with_message("Error : Non-shared var in last")
                .with_code("E0026")
                .with_labels(vec![Label::primary(loc.0, loc.1..loc.2)])
                .with_message(format!("Non shared var {} in last", name)),
        },
        ErrorType::ColAutomata(err) => match err {
            CollapseAutomataError::CyclicModuleCall(s) => Diagnostic::error()
                .with_message("Error : cyclic module calls")
                .with_code("E0019")
                .with_message(format!("Module {} called itself", s)),
            CollapseAutomataError::NoMainModule => Diagnostic::error()
                .with_message("Error : no main module")
                .with_code("E0020")
                .with_message(format!("must have a module called \"main\"")),
            CollapseAutomataError::UnknownModule(loc, name) => Diagnostic::error()
                .with_message("Error : unknown module")
                .with_code("E0014")
                .with_labels(vec![Label::primary(loc.0, loc.1..loc.2)])
                .with_message(format!("Unknown module {}", name)),
            CollapseAutomataError::UnknownVar(loc, name) => Diagnostic::error()
                .with_message("Error : unknown shared variable")
                .with_code("E0012")
                .with_labels(vec![Label::primary(loc.0, loc.1..loc.2)])
                .with_message(format!("Unknown shared variable {}", name)),
            CollapseAutomataError::WrongNumber(typ, loc, expected, got) => Diagnostic::error()
                .with_message(format!("Error : wrong number of {}", typ))
                .with_code("E0016")
                .with_labels(vec![Label::primary(loc.0, loc.1..loc.2)])
                .with_message(format!(
                    "Wrong number of {}:expected {}, got {}",
                    typ, expected, got
                )),
        },
        ErrorType::FlattenError(flatten_error) => {
            let (pos, message, note) = match flatten_error {
                FlattenError::MemoryInReg(pos) => (
                    pos,
                    "Error: Access to memory in register",
                    "Can't access rom or ram memory in a register",
                ),
                FlattenError::ConcatInReg(pos) => (
                    pos,
                    "Error: Concatenation in register",
                    "Cannot statically determine the length of these expressions",
                ),
                FlattenError::SliceInReg(pos) => (
                    pos,
                    "Error: Slice in register",
                    "Cannot statically determine the length of this expression<",
                ),
            };
            Diagnostic::error()
                .with_message(message)
                .with_code("E0022")
                .with_labels(vec![Label::primary(pos.0, pos.1..pos.2)])
                .with_message(note)
        }
        ErrorType::ScheduleError(error) => match error {
            ScheduleError::CycleError => Diagnostic::error()
                .with_message("Error : cyclic immediate shared variable assignment")
                .with_notes(vec![
                    "Could not derive a sound node execution order".to_string()
                ])
                .with_code("E0023"),
            ScheduleError::Other(s) => Diagnostic::error()
                .with_message(format!("Error : {}", s))
                .with_code("E0024"),
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

impl From<(CollapseAutomataError, Rc<SimpleFiles<String, String>>)> for TinyjazzError {
    fn from(err: (CollapseAutomataError, Rc<SimpleFiles<String, String>>)) -> Self {
        let (typ_error, files) = err;
        TinyjazzError {
            error: ErrorType::ColAutomata(typ_error),
            files,
        }
    }
}

impl From<(FlattenError, Rc<SimpleFiles<String, String>>)> for TinyjazzError {
    fn from(err: (FlattenError, Rc<SimpleFiles<String, String>>)) -> Self {
        let (typ_error, files) = err;
        TinyjazzError {
            error: ErrorType::FlattenError(typ_error),
            files,
        }
    }
}

impl From<(ScheduleError, Rc<SimpleFiles<String, String>>)> for TinyjazzError {
    fn from(err: (ScheduleError, Rc<SimpleFiles<String, String>>)) -> Self {
        let (error, files) = err;
        TinyjazzError {
            error: ErrorType::ScheduleError(error),
            files,
        }
    }
}
