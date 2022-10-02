use std::path::Path;
use std::io;


mod executable;

use executable::Writer;
use executable::machine::{Arg, Register, RegisterType, Instruction, Variable};

use super::{Lexeme, LexemeInfo};


pub enum Code
{
    Allocate(u32),
    Exit(u32),
    SetPointer(usize),
    LoopBegin,
    LoopEnd,
    MoveLeft,
    MoveRight,
    Decrement,
    Increment,
    PutChar
}

pub struct Compiler
{
    instructions: Vec<Instruction>,
    data: Vec<Variable>
}

impl Compiler
{
    pub fn compile(lexemes: &[(LexemeInfo, Lexeme)]) -> Result<Self, String>
    {
        let simple = Self::to_simple(lexemes)?;

        let instructions = Self::to_machine(&simple);
        let data = Self::allocate_data(&simple);

        Ok(Compiler{instructions, data})
    }

    fn to_simple(lexemes: &[(LexemeInfo, Lexeme)]) -> Result<Vec<Code>, String>
    {
        let mut instructions = Vec::new();

        instructions.push(Code::Allocate(30000));
        instructions.push(Code::SetPointer(0));

        let mut brackets = Vec::new();

        for (info, lexeme) in lexemes
        {
            match lexeme
            {
                Lexeme::LoopBegin =>
                {
                    instructions.push(Code::LoopBegin);
                    brackets.push(info.index);
                },
                Lexeme::LoopEnd =>
                {
                    if brackets.is_empty()
                    {
                        return Err(format!("no matching [ at byte {}", info.index));
                    }

                    brackets.pop();
                    instructions.push(Code::LoopEnd);
                },
                Lexeme::MoveLeft => instructions.push(Code::MoveLeft),
                Lexeme::MoveRight => instructions.push(Code::MoveRight),
                Lexeme::Decrement => instructions.push(Code::Decrement),
                Lexeme::Increment => instructions.push(Code::Increment),
                Lexeme::Output => instructions.push(Code::PutChar),
                _ => unimplemented!()
            }
        }

        if !brackets.is_empty()
        {
            return Err(format!("no matching ] at byte {}", brackets.pop().unwrap()));
        }

        instructions.push(Code::Exit(0));

        Ok(instructions)
    }

    fn to_machine(instructions: &[Code]) -> Vec<Instruction>
    {
        let mut jump_labels = Vec::new();

        let mut m_instructions = Vec::new();

        for instruction in instructions
        {
            let add_vec = match *instruction
            {
                Code::Exit(exit_code) =>
                {
                    vec![Instruction::Mov(Arg::Register(RegisterType::Word, Register::Ebx),
                            Arg::Value(Variable::Word(exit_code))),
                        Instruction::Xor(Arg::Register(RegisterType::Word, Register::Eax),
                            Arg::Register(RegisterType::Word, Register::Eax)),
                        //exit
                        Instruction::Inc(Arg::Register(RegisterType::Word, Register::Eax)),
                        Instruction::Int(Arg::Value(Variable::Byte(0x80)))]
                },
                Code::SetPointer(value) =>
                {
                    vec![Instruction::Mov(Arg::Register(RegisterType::Word, Register::Esi),
                            Arg::VariableIndex(value))]
                },
                Code::LoopBegin =>
                {
                    let jump_instr = Instruction::Jmp(Arg::AtInstruction(0),
                            Arg::AtInstruction(0));
                    let out_vec = vec![
                        Instruction::Mov(Arg::Register(RegisterType::Word, Register::Ecx),
                            Arg::AtRegister(Register::Esi)),
                        Instruction::Cmp(Arg::Register(RegisterType::Byte, Register::Ecx),
                            Arg::Value(Variable::Byte(0))),
                        Instruction::Jne(Variable::SByte(jump_instr.as_bytes(None).len() as i8)),
                        jump_instr]; //the jump instruction has to be last

                    jump_labels.push(m_instructions.len()+out_vec.len()-1);

                    out_vec
                },
                Code::LoopEnd =>
                {
                    let jump_instr = Instruction::Jmp(Arg::AtInstruction(0),
                        Arg::AtInstruction(0));
                    let mut out_vec = vec![
                        Instruction::Mov(Arg::Register(RegisterType::Word, Register::Ecx),
                            Arg::AtRegister(Register::Esi)),
                        Instruction::Cmp(Arg::Register(RegisterType::Byte, Register::Ecx),
                            Arg::Value(Variable::Byte(0))),
                        Instruction::Je(Variable::SByte(jump_instr.as_bytes(None).len() as i8))];

                    let c_index = m_instructions.len()+out_vec.len();

                    let matching = jump_labels.pop().expect("all labels should have a match");

                    let jump_instr = Instruction::Jmp(Arg::AtInstruction(matching+1),
                        Arg::AtInstruction(c_index));

                    out_vec.push(jump_instr);

                    m_instructions[matching] =
                        Instruction::Jmp(Arg::AtInstruction(c_index+1),
                            Arg::AtInstruction(matching));

                    out_vec
                },
                Code::MoveLeft =>
                {
                    vec![Instruction::Dec(Arg::Register(RegisterType::Word, Register::Esi))]
                },
                Code::MoveRight =>
                {
                    vec![Instruction::Inc(Arg::Register(RegisterType::Word, Register::Esi))]
                },
                Code::Decrement =>
                {
                    vec![Instruction::Mov(Arg::Register(RegisterType::Word, Register::Ecx),
                            Arg::AtRegister(Register::Esi)),
                        Instruction::Dec(Arg::Register(RegisterType::Byte, Register::Ecx)),
                        Instruction::Mov(Arg::AtRegister(Register::Esi),
                            Arg::Register(RegisterType::Word, Register::Ecx))]
                },
                Code::Increment =>
                {
                    vec![Instruction::Mov(Arg::Register(RegisterType::Word, Register::Ecx),
                            Arg::AtRegister(Register::Esi)),
                        Instruction::Inc(Arg::Register(RegisterType::Byte, Register::Ecx)),
                        Instruction::Mov(Arg::AtRegister(Register::Esi),
                            Arg::Register(RegisterType::Word, Register::Ecx))]
                },
                Code::PutChar =>
                {
                    vec![Instruction::Xor(Arg::Register(RegisterType::Word, Register::Ebx),
                            Arg::Register(RegisterType::Word, Register::Ebx)),
                        //std output
                        Instruction::Inc(Arg::Register(RegisterType::Word, Register::Ebx)),
                        //move the current pointer to ecx
                        Instruction::Mov(Arg::Register(RegisterType::Word, Register::Ecx),
                            Arg::Register(RegisterType::Word, Register::Esi)),
                        //1 character
                        Instruction::Mov(Arg::Register(RegisterType::Word, Register::Edx),
                            Arg::Value(Variable::Word(1))),
                        //write
                        Instruction::Mov(Arg::Register(RegisterType::Word, Register::Eax),
                            Arg::Value(Variable::Word(4))),
                        Instruction::Int(Arg::Value(Variable::Byte(0x80)))]
                },
                _ => Vec::new()
            };

            m_instructions.extend(add_vec);
        }

        m_instructions
    }

    fn allocate_data(instructions: &[Code]) -> Vec<Variable>
    {
        let mut variables: Vec<Variable> = Vec::new();

        for instruction in instructions
        {
            match *instruction
            {
                Code::Allocate(num) => {
                    variables.append(&mut vec![Variable::Byte(0); num as usize]);
                    },
                _ => ()
            }
        }

        variables
    }

    pub fn write<P: AsRef<Path>>(&self, filename: P) -> io::Result<()>
    {
        let mut writer = Writer::build(filename)?;
        writer.create(&self.instructions, &self.data)
    }
}