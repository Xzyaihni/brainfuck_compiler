use std::path::Path;

use internal::parser;
use internal::generator::Compiler;


pub fn compile<P: AsRef<Path>>(program: &[u8], filename: P) -> Result<(), String>
{
    let lexemes = parser::parse(program);
    let compiler = Compiler::compile(&lexemes)?;
    compiler.write(filename).map_err(|err| err.to_string())
}

mod internal
{
    pub struct LexemeInfo
    {
        index: usize
    }

    pub enum Lexeme
    {
        MoveRight,
        MoveLeft,
        Increment,
        Decrement,
        LoopBegin,
        LoopEnd,
        Output,
        Input
    }

    pub mod parser
    {
        use super::{Lexeme, LexemeInfo};

        pub fn parse(program: &[u8]) -> Vec<(LexemeInfo, Lexeme)>
        {
            program.iter().enumerate().filter_map(|(index, byte)|
            {
                match byte
                {
                    b'>' => Some(Lexeme::MoveRight),
                    b'<' => Some(Lexeme::MoveLeft),
                    b'+' => Some(Lexeme::Increment),
                    b'-' => Some(Lexeme::Decrement),
                    b'[' => Some(Lexeme::LoopBegin),
                    b']' => Some(Lexeme::LoopEnd),
                    b'.' => Some(Lexeme::Output),
                    b',' => Some(Lexeme::Input),
                    _ => None
                }.map(|lexeme| (LexemeInfo{index}, lexeme))
            }).collect()
        }
    }

    pub mod generator;
}