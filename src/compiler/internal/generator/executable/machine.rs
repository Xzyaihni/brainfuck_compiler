#[cfg(target_pointer_width = "32")]
pub type TargetSize = u32;

#[cfg(target_pointer_width = "64")]
pub type TargetSize = u64;


#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum Register
{
    Eax = 0,
    Ecx,
    Edx,
    Ebx,
    Esp,
    Ebp,
    Esi,
    Edi
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum RegisterType
{
    Byte,
    Word
}

impl Register
{
    pub fn mod_rm(&self, modif: u8, reg: u8) -> u8
    {
        modif<<6|reg<<3|self.rm()
    }

    pub fn rm(&self) -> u8
    {
        *self as u8
    }
}

pub trait Bytable
{
    fn as_bytes(&self) -> Box<[u8]>;
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Variable
{
    SByte(i8),
    SHalf(i16),
    SWord(i32),
    SDWord(i64),
    Byte(u8),
    Half(u16),
    Word(u32),
    DWord(u64)
}

impl Bytable for Variable
{
    fn as_bytes(&self) -> Box<[u8]>
    {
        match self
        {
            Variable::SByte(num) => Box::new(num.to_ne_bytes()),
            Variable::SHalf(num) => Box::new(num.to_ne_bytes()),
            Variable::SWord(num) => Box::new(num.to_ne_bytes()),
            Variable::SDWord(num) => Box::new(num.to_ne_bytes()),
            Variable::Byte(num) => Box::new([*num]),
            Variable::Half(num) => Box::new(num.to_ne_bytes()),
            Variable::Word(num) => Box::new(num.to_ne_bytes()),
            Variable::DWord(num) => Box::new(num.to_ne_bytes())
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Arg
{
    Register(RegisterType, Register),
    Value(Variable),
    VariableIndex(usize),
    AtRegister(Register),
    AtInstruction(usize)
}

pub struct Addresser
{
    instruction_indices: Vec<u32>,
    code_offset: TargetSize,
    variable_indices: Vec<u32>,
    data_offset: TargetSize
}

impl Addresser
{
    pub fn new(
        instructions: &[Instruction],
        code_offset: TargetSize,
        variables: &[Variable],
        data_offset: TargetSize
        ) -> Self
    {
        let instruction_indices = Self::byte_indices(instructions);
        let variable_indices = Self::byte_indices(variables);

        Addresser{
            instruction_indices,
            code_offset,
            variable_indices,
            data_offset
            }
    }

    fn byte_indices(bytable: &[impl Bytable]) -> Vec<u32>
    {
        let mut acc = 0;
        bytable.iter().map(|var|
            {
                let ret = acc;
                acc += var.as_bytes().len();
                ret as u32
            }).collect()
    }

    fn get(&self, index: usize) -> u32
    {
        let item = self.variable_indices.get(index).expect("out of bounds access");
        self.data_offset as u32 + item
    }

    fn get_instruction(&self, index: usize) -> u32
    {
        let item = self.instruction_indices.get(index).expect("no instruction at index");
        self.code_offset as u32 + item
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Instruction
{
    Nop,
    Dec(Arg),
    Inc(Arg),
    Int(Arg),
    Mov(Arg, Arg),
    Xor(Arg, Arg),
    Cmp(Arg, Arg),
    Jmp(Arg, Arg),
    Je(Variable),
    Jne(Variable)
}

impl Bytable for Instruction
{
    fn as_bytes(&self) -> Box<[u8]>
    {
        self.as_bytes(None)
    }
}

impl Instruction
{
    pub fn as_bytes(&self, addresser: Option<&Addresser>) -> Box<[u8]>
    {
        match self
        {
            Instruction::Nop =>
            {
                Box::new([0x90])
            },
            Instruction::Dec(Arg::Register(RegisterType::Byte, dst)) =>
            {
                Box::new([0xfe, dst.mod_rm(0b11,1)])
            },
            Instruction::Inc(Arg::Register(RegisterType::Byte, dst)) =>
            {
                Box::new([0xfe, dst.mod_rm(0b11,0)])
            },
            Instruction::Dec(Arg::Register(RegisterType::Word, dst)) =>
            {
                Box::new([0xff, dst.mod_rm(0b11,1)])
            },
            Instruction::Inc(Arg::Register(RegisterType::Word, dst)) =>
            {
                Box::new([0xff, dst.mod_rm(0b11,0)])
            },
            Instruction::Int(Arg::Value(Variable::Byte(dst))) =>
            {
                Box::new([0xcd, *dst])
            },
            Instruction::Mov(Arg::Register(RegisterType::Word, dst),
                Arg::Value(Variable::Word(src))) =>
            {
                [0xc7, dst.mod_rm(0b11,0)]
                    .into_iter().chain(src.to_ne_bytes())
                    .collect::<Vec<u8>>().into_boxed_slice()
            },
            Instruction::Mov(Arg::Register(RegisterType::Word, dst),
                Arg::Register(RegisterType::Word, src)) =>
            {
                Box::new([0x89, dst.mod_rm(0b11,src.rm())])
            },
            Instruction::Mov(Arg::Register(RegisterType::Word, dst), Arg::VariableIndex(src)) =>
            {
                let address_bytes = if let Some(addresser) = addresser
                {
                    addresser.get(*src).to_ne_bytes()
                } else
                {
                    [0; 4]
                };

                [0xc7, dst.mod_rm(0b11,0)]
                    .into_iter().chain(address_bytes)
                    .collect::<Vec<u8>>().into_boxed_slice()
            },
            Instruction::Mov(Arg::AtRegister(dst), Arg::Register(RegisterType::Word, src)) =>
            {
                if cfg!(target_pointer_width = "64")
                {
                    Box::new([0x67, 0x89, dst.mod_rm(0b00,src.rm())])
                } else if cfg!(target_pointer_width = "32")
                {
                    Box::new([0x89, dst.mod_rm(0b00,src.rm())])
                } else
                {
                    unimplemented!()
                }
            },
            Instruction::Mov(Arg::Register(RegisterType::Word, dst), Arg::AtRegister(src)) =>
            {
                if cfg!(target_pointer_width = "64")
                {
                    Box::new([0x67, 0x8b, src.mod_rm(0b00,dst.rm())])
                } else if cfg!(target_pointer_width = "32")
                {
                    Box::new([0x8b, src.mod_rm(0b00,dst.rm())])
                } else
                {
                    unimplemented!()
                }
            },
            Instruction::Xor(Arg::Register(RegisterType::Word, dst),
                Arg::Register(RegisterType::Word, src)) =>
            {
                Box::new([0x31, dst.mod_rm(0b11,src.rm())])
            },
            Instruction::Cmp(Arg::Register(RegisterType::Byte, dst),
                Arg::Value(Variable::Byte(src))) =>
            {
                Box::new([0x80, dst.mod_rm(0b11,7), *src])
            },
            Instruction::Cmp(Arg::Register(RegisterType::Byte, dst),
                Arg::Register(RegisterType::Byte, src)) =>
            {
                Box::new([0x38, dst.mod_rm(0b11,src.rm())])
            },
            Instruction::Jmp(Arg::AtInstruction(dst), Arg::AtInstruction(src)) =>
            {
                let address_bytes = if let Some(addresser) = addresser
                {
                    let dest_address = addresser.get_instruction(*dst) as i32;
                    let src_address = addresser.get_instruction(*src+1) as i32;

                    (dest_address-src_address).to_ne_bytes()
                } else
                {
                    [0; 4]
                };

                [0xe9].into_iter()
                    .chain(address_bytes)
                    .collect::<Vec<u8>>().into_boxed_slice()
            },
            Instruction::Je(Variable::SByte(dst)) =>
            {
                Box::new([0x74, dst.to_ne_bytes()[0]])
            },
            Instruction::Jne(Variable::SByte(dst)) =>
            {
                Box::new([0x75, dst.to_ne_bytes()[0]])
            },
            _ =>
            {
                println!("unimplemented {self:?}");
                unimplemented!()
            }
        }
    }
}