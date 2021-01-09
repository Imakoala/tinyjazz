// auto-generated: "lalrpop 0.19.1"
// sha256: d6abb1ac2eacdab87f777af07914d9e238c97219662ff3772c20b72e6843a6
use std::str::FromStr;
use crate::frontend::from_netlist::parse_ast::*;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use crate::frontend::from_netlist::parse_ast::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1((&'input str, usize)),
        Variant2(::std::vec::Vec<(&'input str, usize)>),
        Variant3(::std::vec::Vec<&'input str>),
        Variant4(BiOp),
        Variant5(Vec<(&'input str, usize)>),
        Variant6(Vec<&'input str>),
        Variant7(Vec<bool>),
        Variant8(Expr<'input>),
        Variant9((&'input str, Expr<'input>)),
        Variant10(::std::vec::Vec<(&'input str, Expr<'input>)>),
        Variant11(usize),
        Variant12(Netlist<'input>),
        Variant13(::std::option::Option<(&'input str, usize)>),
        Variant14(::std::option::Option<&'input str>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, 0, 31,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0, 0, 0, -23, 0, 0, 31,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 31,
        // State 4
        0, 0, 0, 0, 0, 0, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31,
        // State 5
        0, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31,
        // State 7
        0, 45, 46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31,
        // State 9
        0, 52, 53, 0, 0, 54, 55, 0, 0, 12, 56, 13, 57, 0, 14, 15, 16, 17, 18, 0, 58, 0, 31,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31,
        // State 16
        0, 45, 46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0,
        // State 17
        0, 45, 46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31,
        // State 22
        0, 45, 46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0, 0, 0, -20, 0, 0, 0,
        // State 30
        -52, 0, 0, -52, -52, 0, 0, -52, 0, 0, 0, 0, 0, -52, 0, 0, 0, 0, 0, -52, 0, 0, -52,
        // State 31
        34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, 0, 0, -22, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, -9, 0, 0, -9,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, -10, 0, 0, -10,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        40, 0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        -48, 0, 0, 8, 0, 0, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        41, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4,
        // State 40
        0, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41,
        // State 42
        0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        -49, 0, 0, 0, 0, 0, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        -44, -44, -44, 0, 0, 0, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, -44,
        // State 45
        -45, -45, -45, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, -45,
        // State 46
        -43, -43, -43, 0, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, -43,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42,
        // State 48
        0, 59, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37,
        // State 51
        0, -26, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26,
        // State 52
        0, -27, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -11,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13,
        // State 58
        0, -24, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24,
        // State 59
        0, -25, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -34,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32,
        // State 67
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 23 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        -46,
        // State 7
        0,
        // State 8
        -47,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        -55,
        // State 28
        0,
        // State 29
        0,
        // State 30
        -52,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        -41,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        -42,
        // State 48
        -36,
        // State 49
        -38,
        // State 50
        -37,
        // State 51
        -26,
        // State 52
        -27,
        // State 53
        0,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        -24,
        // State 59
        -25,
        // State 60
        -28,
        // State 61
        -30,
        // State 62
        -29,
        // State 63
        -31,
        // State 64
        -33,
        // State 65
        -34,
        // State 66
        -32,
        // State 67
        -35,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 5,
            5 => 2,
            6 => 10,
            7 => 35,
            8 => match state {
                3 => 34,
                _ => 28,
            },
            9 => 48,
            10 => 49,
            11 => match state {
                8 => 47,
                _ => 41,
            },
            13 => 8,
            14 => match state {
                17 => 22,
                22 => 25,
                7 => 43,
                _ => 21,
            },
            15 => 27,
            16 => match state {
                5 => 38,
                _ => 36,
            },
            18 => match state {
                10 => 18,
                11 => 19,
                13 => 20,
                19 => 23,
                20 => 24,
                24 => 26,
                2 => 31,
                4..=5 => 37,
                6 | 8 => 42,
                9 => 50,
                12 => 60,
                14 => 61,
                15 => 62,
                18 => 63,
                21 => 64,
                23 => 65,
                25 => 66,
                26 => 67,
                _ => 29,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> Vec<::std::string::String> {
        const __TERMINAL: &[&str] = &[
            r###"",""###,
            r###""0""###,
            r###""1""###,
            r###"":""###,
            r###""=""###,
            r###""AND""###,
            r###""CONCAT""###,
            r###""IN""###,
            r###""INPUT""###,
            r###""MUX""###,
            r###""NAND""###,
            r###""NOT""###,
            r###""OR""###,
            r###""OUTPUT""###,
            r###""RAM""###,
            r###""REG""###,
            r###""ROM""###,
            r###""SELECT""###,
            r###""SLICE""###,
            r###""VAR""###,
            r###""XOR""###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[a-zA-Z_][a-zA-Z_0-9]*"#"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct __StateMachine<'input>
    where 
    {
        input: &'input str,
        __phantom: ::std::marker::PhantomData<(&'input ())>,
    }
    impl<'input> __state_machine::ParserDefinition for __StateMachine<'input>
    where 
    {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Netlist<'input>;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, ::std::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 23 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, ::std::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i8) -> Vec<String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut Vec<i8>,
            symbols: &mut Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                action,
                start_location,
                states,
                symbols,
                ::std::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
        'input,
    >(
        __token: &Token<'input>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(2, _) if true => Some(0),
            Token(3, _) if true => Some(1),
            Token(4, _) if true => Some(2),
            Token(5, _) if true => Some(3),
            Token(6, _) if true => Some(4),
            Token(7, _) if true => Some(5),
            Token(8, _) if true => Some(6),
            Token(9, _) if true => Some(7),
            Token(10, _) if true => Some(8),
            Token(11, _) if true => Some(9),
            Token(12, _) if true => Some(10),
            Token(13, _) if true => Some(11),
            Token(14, _) if true => Some(12),
            Token(15, _) if true => Some(13),
            Token(16, _) if true => Some(14),
            Token(17, _) if true => Some(15),
            Token(18, _) if true => Some(16),
            Token(19, _) if true => Some(17),
            Token(20, _) if true => Some(18),
            Token(21, _) if true => Some(19),
            Token(22, _) if true => Some(20),
            Token(0, _) if true => Some(21),
            Token(1, _) if true => Some(22),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 => match __token {
                Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(17, __tok0) | Token(18, __tok0) | Token(19, __tok0) | Token(20, __tok0) | Token(21, __tok0) | Token(22, __tok0) | Token(0, __tok0) | Token(1, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct ProgramParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl ProgramParser {
        pub fn new() -> ProgramParser {
            let __builder = super::__intern_token::new_builder();
            ProgramParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
        >(
            &self,
            input: &'input str,
        ) -> Result<Netlist<'input>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    input,
                    __phantom: ::std::marker::PhantomData::<(&())>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> Option<Result<Netlist<'input>,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            2 => {
                __reduce2(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            3 => {
                __reduce3(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            6 => {
                __reduce6(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            7 => {
                __reduce7(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            8 => {
                __reduce8(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            9 => {
                __reduce9(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            10 => {
                __reduce10(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            11 => {
                __reduce11(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            12 => {
                __reduce12(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            13 => {
                __reduce13(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            14 => {
                __reduce14(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            15 => {
                __reduce15(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            16 => {
                __reduce16(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            17 => {
                __reduce17(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            18 => {
                __reduce18(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            19 => {
                __reduce19(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            20 => {
                __reduce20(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            21 => {
                __reduce21(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            22 => {
                __reduce22(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            23 => {
                __reduce23(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            24 => {
                __reduce24(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            25 => {
                __reduce25(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            26 => {
                __reduce26(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            27 => {
                __reduce27(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            28 => {
                __reduce28(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            29 => {
                __reduce29(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            30 => {
                __reduce30(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            31 => {
                __reduce31(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            32 => {
                __reduce32(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            33 => {
                __reduce33(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            34 => {
                __reduce34(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            35 => {
                __reduce35(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            36 => {
                __reduce36(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            37 => {
                __reduce37(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            38 => {
                __reduce38(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            39 => {
                __reduce39(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            40 => {
                __reduce40(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            41 => {
                __reduce41(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            42 => {
                __reduce42(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            43 => {
                __reduce43(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            44 => {
                __reduce44(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            45 => {
                __reduce45(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            46 => {
                __reduce46(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            47 => {
                __reduce47(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            48 => {
                __reduce48(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            49 => {
                __reduce49(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            50 => {
                __reduce50(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            51 => {
                __reduce51(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            52 => {
                __reduce52(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            53 => {
                __reduce53(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            54 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_Variant12(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (&'input str, Expr<'input>), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (&'input str, usize), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BiOp, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Netlist<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<(&'input str, usize)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<bool>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<&'input str>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<(&'input str, usize)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<&'input str>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(&'input str, Expr<'input>)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(&'input str, usize)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<&'input str>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<SizedVar> ",") = SizedVar, "," => ActionFn(41);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action41::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<SizedVar> ",")* =  => ActionFn(39);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action39::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<SizedVar> ",")* = (<SizedVar> ",")+ => ActionFn(40);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<SizedVar> ",")+ = SizedVar, "," => ActionFn(48);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action48::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<SizedVar> ",")+ = (<SizedVar> ",")+, SizedVar, "," => ActionFn(49);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action49::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Var> ",") = Var, "," => ActionFn(36);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action36::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Var> ",")* =  => ActionFn(34);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action34::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Var> ",")* = (<Var> ",")+ => ActionFn(35);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Var> ",")+ = Var, "," => ActionFn(52);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action52::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Var> ",")+ = (<Var> ",")+, Var, "," => ActionFn(53);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action53::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BiOp = "AND" => ActionFn(19);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BiOp = "OR" => ActionFn(20);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BiOp = "XOR" => ActionFn(21);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BiOp = "NAND" => ActionFn(22);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BiOp = "CONCAT" => ActionFn(23);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<SizedVar> = SizedVar => ActionFn(58);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<SizedVar> =  => ActionFn(59);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action59::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<SizedVar> = (<SizedVar> ",")+, SizedVar => ActionFn(60);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action60::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 7)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<SizedVar> = (<SizedVar> ",")+ => ActionFn(61);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action61::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Var> = Var => ActionFn(62);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action62::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Var> =  => ActionFn(63);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action63::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 8)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Var> = (<Var> ",")+, Var => ActionFn(64);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action64::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Var> = (<Var> ",")+ => ActionFn(65);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action65::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Const = Const, "0" => ActionFn(24);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action24::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Const = Const, "1" => ActionFn(25);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action25::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Const = "0" => ActionFn(26);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Const = "1" => ActionFn(27);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = "NOT", Var => ActionFn(9);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action9::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 10)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = "ROM", Var => ActionFn(10);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action10::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 10)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = "REG", Var => ActionFn(11);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action11::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 10)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = BiOp, Var, Var => ActionFn(12);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action12::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 10)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = "SLICE", Num, Num, Var => ActionFn(13);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant11(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action13::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (4, 10)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = "SELECT", Num, Var => ActionFn(14);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action14::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 10)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = "MUX", Var, Var, Var => ActionFn(15);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action15::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (4, 10)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = "RAM", Var, Var, Var, Var => ActionFn(16);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action16::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (5, 10)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = Const => ActionFn(17);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce36<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = Var => ActionFn(18);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce37<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Instr = Var, "=", Expr => ActionFn(6);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action6::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce38<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Instr* =  => ActionFn(28);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action28::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 12)
    }
    pub(crate) fn __reduce39<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Instr* = Instr+ => ActionFn(29);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce40<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Instr+ = Instr => ActionFn(42);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce41<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Instr+ = Instr+, Instr => ActionFn(43);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action43::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (2, 13)
    }
    pub(crate) fn __reduce42<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Num = r#"[0-9]+"# => ActionFn(2);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce43<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Num = "0" => ActionFn(3);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce44<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Num = "1" => ActionFn(4);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce45<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = "INPUT", Comma<Var>, "OUTPUT", Comma<Var>, "VAR", Comma<SizedVar>, "IN" => ActionFn(56);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant5(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action56::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (7, 15)
    }
    pub(crate) fn __reduce46<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = "INPUT", Comma<Var>, "OUTPUT", Comma<Var>, "VAR", Comma<SizedVar>, "IN", Instr+ => ActionFn(57);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant10(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant5(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action57::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (8, 15)
    }
    pub(crate) fn __reduce47<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // SizedVar = Var => ActionFn(7);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce48<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // SizedVar = Var, ":", Num => ActionFn(8);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant11(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 16)
    }
    pub(crate) fn __reduce49<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // SizedVar? = SizedVar => ActionFn(37);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce50<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // SizedVar? =  => ActionFn(38);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action38::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 17)
    }
    pub(crate) fn __reduce51<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Var = r#"[a-zA-Z_][a-zA-Z_0-9]*"# => ActionFn(1);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce52<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Var? = Var => ActionFn(32);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action32::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce53<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Var? =  => ActionFn(33);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action33::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (0, 19)
    }
}
pub use self::__parse__Program::ProgramParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use std::str::FromStr;
    use crate::frontend::from_netlist::parse_ast::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
        let __strs: &[(&str, bool)] = &[
            ("^([0-9]+)", false),
            ("^([A-Z_a-z][0-9A-Z_a-z]*)", false),
            ("^(,)", false),
            ("^(0)", false),
            ("^(1)", false),
            ("^(:)", false),
            ("^(=)", false),
            ("^(AND)", false),
            ("^(CONCAT)", false),
            ("^(IN)", false),
            ("^(INPUT)", false),
            ("^(MUX)", false),
            ("^(NAND)", false),
            ("^(NOT)", false),
            ("^(OR)", false),
            ("^(OUTPUT)", false),
            ("^(RAM)", false),
            ("^(REG)", false),
            ("^(ROM)", false),
            ("^(SELECT)", false),
            ("^(SLICE)", false),
            ("^(VAR)", false),
            ("^(XOR)", false),
            (r"^(\s*)", true),
        ];
        __lalrpop_util::lexer::MatcherBuilder::new(__strs.iter().copied()).unwrap()
    }
}
pub use self::__lalrpop_util::lexer::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Netlist<'input>, usize),
) -> Netlist<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    __0
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> usize
{
    usize::from_str(s).unwrap()
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> usize
{
    0
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> usize
{
    1
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, i, _): (usize, Vec<&'input str>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, o, _): (usize, Vec<&'input str>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, Vec<(&'input str, usize)>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, instr, _): (usize, ::std::vec::Vec<(&'input str, Expr<'input>)>, usize),
) -> Netlist<'input>
{
    {
        Netlist {
            inputs: i.into_iter().collect(), 
            outputs: o.into_iter().collect(),
            vars: v.into_iter().collect(),
            instr: instr.into_iter().collect()
        }
    }
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, Expr<'input>, usize),
) -> (&'input str, Expr<'input>)
{
    (v, e)
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, &'input str, usize),
) -> (&'input str, usize)
{
    (v, 1)
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, usize, usize),
) -> (&'input str, usize)
{
    (v, n)
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, &'input str, usize),
) -> Expr<'input>
{
    Expr::Not(v)
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, &'input str, usize),
) -> Expr<'input>
{
    Expr::Rom(v)
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, &'input str, usize),
) -> Expr<'input>
{
    Expr::Reg(v)
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, op, _): (usize, BiOp, usize),
    (_, v1, _): (usize, &'input str, usize),
    (_, v2, _): (usize, &'input str, usize),
) -> Expr<'input>
{
    Expr::BiOp(op, v1, v2)
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, n1, _): (usize, usize, usize),
    (_, n2, _): (usize, usize, usize),
    (_, v, _): (usize, &'input str, usize),
) -> Expr<'input>
{
    Expr::Slice(v, n1, n2)
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, n1, _): (usize, usize, usize),
    (_, v, _): (usize, &'input str, usize),
) -> Expr<'input>
{
    Expr::Slice(v, n1, n1+1)
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, v1, _): (usize, &'input str, usize),
    (_, v2, _): (usize, &'input str, usize),
    (_, v3, _): (usize, &'input str, usize),
) -> Expr<'input>
{
    Expr::Mux(v1, v2, v3)
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, v1, _): (usize, &'input str, usize),
    (_, v2, _): (usize, &'input str, usize),
    (_, v3, _): (usize, &'input str, usize),
    (_, v4, _): (usize, &'input str, usize),
) -> Expr<'input>
{
    Expr::Ram(v1, v2, v3, v4)
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<bool>, usize),
) -> Expr<'input>
{
    Expr::Const(__0)
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expr<'input>
{
    Expr::Var(__0)
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BiOp
{
    BiOp::And
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BiOp
{
    BiOp::Or
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BiOp
{
    BiOp::Xor
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BiOp
{
    BiOp::Nand
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BiOp
{
    BiOp::Concat
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    input: &'input str,
    (_, mut v, _): (usize, Vec<bool>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Vec<bool>
{
    {v.push(false); v}
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    input: &'input str,
    (_, mut v, _): (usize, Vec<bool>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Vec<bool>
{
    {v.push(true); v}
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Vec<bool>
{
    vec![false]
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Vec<bool>
{
    vec![true]
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<(&'input str, Expr<'input>)>
{
    vec![]
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(&'input str, Expr<'input>)>, usize),
) -> ::std::vec::Vec<(&'input str, Expr<'input>)>
{
    v
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(&'input str, usize)>, usize),
    (_, e, _): (usize, ::std::option::Option<(&'input str, usize)>, usize),
) -> Vec<(&'input str, usize)>
{
    match e { 
        None => v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
fn __action31<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<&'input str>, usize),
    (_, e, _): (usize, ::std::option::Option<&'input str>, usize),
) -> Vec<&'input str>
{
    match e { 
        None => v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
fn __action32<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ::std::option::Option<&'input str>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action33<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<&'input str>
{
    None
}

#[allow(unused_variables)]
fn __action34<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<&'input str>
{
    vec![]
}

#[allow(unused_variables)]
fn __action35<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<&'input str>, usize),
) -> ::std::vec::Vec<&'input str>
{
    v
}

#[allow(unused_variables)]
fn __action36<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> &'input str
{
    __0
}

#[allow(unused_variables)]
fn __action37<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (&'input str, usize), usize),
) -> ::std::option::Option<(&'input str, usize)>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action38<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<(&'input str, usize)>
{
    None
}

#[allow(unused_variables)]
fn __action39<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<(&'input str, usize)>
{
    vec![]
}

#[allow(unused_variables)]
fn __action40<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(&'input str, usize)>, usize),
) -> ::std::vec::Vec<(&'input str, usize)>
{
    v
}

#[allow(unused_variables)]
fn __action41<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (&'input str, usize), usize),
    (_, _, _): (usize, &'input str, usize),
) -> (&'input str, usize)
{
    __0
}

#[allow(unused_variables)]
fn __action42<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (&'input str, Expr<'input>), usize),
) -> ::std::vec::Vec<(&'input str, Expr<'input>)>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action43<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(&'input str, Expr<'input>)>, usize),
    (_, e, _): (usize, (&'input str, Expr<'input>), usize),
) -> ::std::vec::Vec<(&'input str, Expr<'input>)>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action44<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (&'input str, usize), usize),
) -> ::std::vec::Vec<(&'input str, usize)>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action45<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(&'input str, usize)>, usize),
    (_, e, _): (usize, (&'input str, usize), usize),
) -> ::std::vec::Vec<(&'input str, usize)>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action46<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ::std::vec::Vec<&'input str>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action47<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<&'input str>, usize),
    (_, e, _): (usize, &'input str, usize),
) -> ::std::vec::Vec<&'input str>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action48<
    'input,
>(
    input: &'input str,
    __0: (usize, (&'input str, usize), usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<(&'input str, usize)>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action41(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action44(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action49<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<(&'input str, usize)>, usize),
    __1: (usize, (&'input str, usize), usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<(&'input str, usize)>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action41(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action45(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action50<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::option::Option<(&'input str, usize)>, usize),
) -> Vec<(&'input str, usize)>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action39(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action30(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action51<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<(&'input str, usize)>, usize),
    __1: (usize, ::std::option::Option<(&'input str, usize)>, usize),
) -> Vec<(&'input str, usize)>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action40(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action30(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action52<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<&'input str>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action36(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action46(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action53<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<&'input str>, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<&'input str>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action36(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action47(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action54<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::option::Option<&'input str>, usize),
) -> Vec<&'input str>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action34(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action31(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action55<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<&'input str>, usize),
    __1: (usize, ::std::option::Option<&'input str>, usize),
) -> Vec<&'input str>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action35(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action31(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action56<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Vec<&'input str>, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, Vec<&'input str>, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, Vec<(&'input str, usize)>, usize),
    __6: (usize, &'input str, usize),
) -> Netlist<'input>
{
    let __start0 = __6.2.clone();
    let __end0 = __6.2.clone();
    let __temp0 = __action28(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action57<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Vec<&'input str>, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, Vec<&'input str>, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, Vec<(&'input str, usize)>, usize),
    __6: (usize, &'input str, usize),
    __7: (usize, ::std::vec::Vec<(&'input str, Expr<'input>)>, usize),
) -> Netlist<'input>
{
    let __start0 = __7.0.clone();
    let __end0 = __7.2.clone();
    let __temp0 = __action29(
        input,
        __7,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action58<
    'input,
>(
    input: &'input str,
    __0: (usize, (&'input str, usize), usize),
) -> Vec<(&'input str, usize)>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action37(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action50(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action59<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<(&'input str, usize)>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action38(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action50(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action60<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<(&'input str, usize)>, usize),
    __1: (usize, (&'input str, usize), usize),
) -> Vec<(&'input str, usize)>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action37(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action51(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action61<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<(&'input str, usize)>, usize),
) -> Vec<(&'input str, usize)>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action38(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action51(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action62<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> Vec<&'input str>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action32(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action54(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action63<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<&'input str>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action33(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action54(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action64<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<&'input str>, usize),
    __1: (usize, &'input str, usize),
) -> Vec<&'input str>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action32(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action55(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action65<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<&'input str>, usize),
) -> Vec<&'input str>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action33(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action55(
        input,
        __0,
        __temp0,
    )
}

pub trait __ToTriple<'input, > {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>;
}

impl<'input, > __ToTriple<'input, > for (usize, Token<'input>, usize) {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Token<'input>, usize), &'static str> {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
