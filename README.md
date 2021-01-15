# Tinyjazz

Tinjazz is a dataflow language with a netlist backend, based on minijazz <https://github.com/inria-parkas/minijazz>.

## Build instructions

To build, you need a recent version of rustc and cargo.
Then, run:

```sh
cargo build --release
```

The executable is in target/release.

## Usage

To display a help page with all the options:

```sh
./tinyjazz --help
```

## Code structure

The [build.rs](build.rs) file is used to generate the parser and lexer from .lalrpop files during compilation.
All the code is in [/src](src/), organised in different files.

* [The main file](src/main.rs) handles the command line interface, and calls all the other functions.
* [USAGE.docopt](src/USAGE.docopt) is a high-level description of the command line interface, which the docopt crate uses to generates a command line parser.
* [The ast folder](src/ast/) contains all the different internal representation which are used in the compiler.
* [The frontend folder](src/frontend) contains all the code to convert the original file to [the last intermediate representation](src/ast/graph.rs). Each file is named after the object it handles, for example [constants.rs](src/frontend/constants.rs) replaces the constants with their value. The two folders correspond to the netlist parser and to the main parser.
* [The backends folder](src/backends) contains code to convert the last intermediate representation into actual code. The only target is netlists.
* [The optimization folder](src/optimization) contains the code used to optimize the program. It only uses the last intermediate representation for that.
* [The interpreters folder](src/interpreters) contains the two interpreters I made for two different representation. This will probably be refactored into a simple file containing the [newer interpreter](src/interpreters/low_level_interpreter.rs), and the [older one](src/interpreters/high_level_interpreter.rs) will be dropped at some point.
* [The util folder](src/util) contains miscallenous utility features, such as [error handling](src/util/errors.rs), the [.dot file generation](src/util/viz.rs), and the [rhai scripting](src/util/scripting.rs).
* [The test folder](src/test) should contain unit test for the compiler. Currently, it doesn't.
