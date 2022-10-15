use std::path::Path;
use std::io::{self, Seek, SeekFrom, Write};

use std::fs::{self, File};
use std::os::unix::fs::PermissionsExt;

use std::mem;

use std::iter::Iterator;

use std::env::consts::{OS, ARCH};


use machine::{Bytable, Addresser, TargetSize};

pub mod machine;


struct StringTable
{
    index: usize,
    strings: Vec<u8>
}

impl StringTable
{
    pub fn new() -> Self
    {
        StringTable{
            index: 0,
            strings: vec![b'\0']
            }
    }

    pub fn add(&mut self, text: &str)
    {
        self.strings.extend(text.bytes());
        self.strings.push(b'\0');
    }

    pub fn as_bytes(&self) -> Vec<u8>
    {
        self.strings.clone()
    }
}

impl Iterator for StringTable
{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.index < self.strings.len()
        {
            loop
            {
                let c: u8 = *self.strings.get(self.index)
                    .expect("strings must be null nerminated");

                if c==b'\0'
                {
                    self.index += 1;
                    return Some(self.index as u32);
                }

                self.index += 1;
            }
        } else
        {
            None
        }
    }
}

pub struct Writer
{
    file: File,
    entry: TargetSize,

    shstr_index: Option<usize>,
    shstr_table: StringTable,

    sections: Vec<SectionInfo>,
    programs: Vec<ProgramInfo>,

    code_index: Option<usize>,
    program_bytes: Vec<u8>,

    data_index: Option<usize>,
    program_data: Vec<u8>
}

macro_rules! bytes_add
{
    ( $($x:expr),* ) =>
    {
        {
            let mut out_bytes = Vec::new();

            $(
                out_bytes.extend($x.to_ne_bytes().iter());
            )*

            out_bytes.try_into().expect("the number of output bytes doesnt match")
        }
    };
}

const SECTION_SIZE: usize = mem::size_of::<SectionInfo>();
struct SectionInfo
{
    name_index: u32,
    section_type: u32,
    flags: TargetSize,
    address: TargetSize,
    offset: TargetSize,
    size: TargetSize,
    link: u32,
    info: u32,
    align: TargetSize,
    fixed_size: TargetSize
}

impl SectionInfo
{
    pub fn as_bytes(&self) -> [u8; SECTION_SIZE]
    {
        bytes_add!(
            self.name_index,
            self.section_type,
            self.flags,
            self.address,
            self.offset,
            self.size,
            self.link,
            self.info,
            self.align,
            self.fixed_size
            )
    }
}

const PROGRAM_SIZE: usize = mem::size_of::<ProgramInfo>();
struct ProgramInfo
{
    header_type: u32,
    flags: u32,
    offset: TargetSize,
    v_addr: TargetSize,
    p_addr: TargetSize,
    file_size: TargetSize,
    mem_size: TargetSize,
    align: TargetSize
}

impl ProgramInfo
{
    pub fn as_bytes(&self) -> [u8; PROGRAM_SIZE]
    {
        if cfg!(target_pointer_width = "64")
        {
            bytes_add!(
                self.header_type,
                self.flags,
                self.offset,
                self.v_addr,
                self.p_addr,
                self.file_size,
                self.mem_size,
                self.align
                )
        } else if cfg!(target_pointer_width = "32")
        {
            bytes_add!(
                self.header_type,
                self.offset,
                self.v_addr,
                self.p_addr,
                self.file_size,
                self.mem_size,
                self.flags,
                self.align
                )
        } else
        {
            unimplemented!();
        }
    }
}

struct ElfIdent
{
    magic_numbers: [u8; 4],
    bits: u8,
    endianness: u8,
    version: u8,
    abi: u8,
    padding: [u8; 8]
}

impl ElfIdent
{
    pub fn to_ne_bytes(&self) -> [u8; mem::size_of::<ElfIdent>()]
    {
        let mut out_bytes = Vec::new();

        out_bytes.extend(self.magic_numbers.iter());
        out_bytes.push(self.bits);
        out_bytes.push(self.endianness);
        out_bytes.push(self.version);
        out_bytes.push(self.abi);
        out_bytes.extend(self.padding.iter());

        out_bytes.try_into().expect("e_ident sizes dont match")
    }
}


const ELF_SIZE: usize = mem::size_of::<ElfInfo>();
struct ElfInfo
{
    ident: ElfIdent,
    file_type: u16,
    arch: u16,
    file_version: u32,
    entry: TargetSize,
    program_offset: TargetSize,
    sections_offset: TargetSize,
    flags: u32,
    elf_header_size: u16,
    program_size: u16,
    programs_amount: u16,
    section_size: u16,
    sections_amount: u16,
    name_section_index: u16
}

impl ElfInfo
{
    pub fn as_bytes(&self) -> [u8; ELF_SIZE]
    {
        bytes_add!(
            self.ident,
            self.file_type,
            self.arch,
            self.file_version,
            self.entry,
            self.program_offset,
            self.sections_offset,
            self.flags,
            self.elf_header_size,
            self.program_size,
            self.programs_amount,
            self.section_size,
            self.sections_amount,
            self.name_section_index
            )
    }
}

impl Writer
{
    pub fn build<P: AsRef<Path>>(filename: P) -> Result<Self, io::Error>
    {
        if cfg!(target_family = "windows")
        {
            panic!("unsupported platform: {OS}");
        }

        let file = File::create(&filename)?;
        fs::set_permissions(&filename, fs::Permissions::from_mode(0o755))?;

        Ok(Writer{
            file,
            entry: 0x400000, //default for linux i think

            shstr_index: None,
            shstr_table: StringTable::new(),

            sections: Vec::new(),
            programs: Vec::new(),

            code_index: None,
            program_bytes: Vec::new(),

            data_index: None,
            program_data: Vec::new()
            })
    }

    pub fn create(&mut self,
        instructions: &[machine::Instruction],
        data: &[machine::Variable]
        ) -> io::Result<()>
    {
        let bits =
        {
            if cfg!(target_pointer_width = "32")
            {
                1
            } else if cfg!(target_pointer_width = "64")
            {
                2
            } else
            {
                unimplemented!();
            }
        };

        let endianness =
        {
            if cfg!(target_endian = "little")
            {
                1
            } else
            {
                2
            }
        };

        let arch = match ARCH
        {
            "x86" => 3,
            "x86_64" => 0x3e,
            _ => unimplemented!()
        };

        self.allocate_data(data);

        let sections_offset = self.create_sections(Self::program_length(instructions));

        self.create_programs();

        let elf_ident = ElfIdent{
            magic_numbers: [0x7f, b'E', b'L', b'F'],
            bits,
            endianness,
            version: 1, //current
            abi: 0, //0 for none/systemv for some reason (3 for linux??)
            padding: [0; 8]
            };

        let elf_info = ElfInfo{
            ident: elf_ident,
            file_type: 2, //2 for executable
            arch,
            file_version: 1, //must be 1 (current)
            entry: self.entry + self.sections[self.code_index.unwrap()].offset,
            program_offset: ELF_SIZE as TargetSize,
            sections_offset,
            flags: 0,
            elf_header_size: ELF_SIZE as u16,
            program_size: PROGRAM_SIZE as u16,
            programs_amount: self.programs.len() as u16,
            section_size: SECTION_SIZE as u16,
            sections_amount: self.sections.len() as u16,
            name_section_index: self.shstr_index.unwrap() as u16
            };

        //read it properly (with offsets to .data)
        let addresser = Addresser::new(
            instructions,
            self.entry + self.sections[self.code_index.unwrap()].offset,
            data,
            self.entry + self.sections[self.data_index.unwrap()].offset);

        self.read_bytes(instructions, &addresser);

        self.file.write_all(&elf_info.as_bytes())?;

        for program in self.programs.iter()
        {
            self.file.write_all(&program.as_bytes())?;
        }

        self.write_body()?;

        for section in self.sections.iter()
        {
            self.file.write_all(&section.as_bytes())?;
        }

        Ok(())
    }

    fn create_sections(&mut self, program_length: TargetSize) -> TargetSize
    {
        self.sections.push(SectionInfo{
            name_index: 0,
            section_type: 0,
            flags: 0,
            address: 0,
            offset: 0,
            size: 0,
            link: 0,
            info: 0,
            align: 0,
            fixed_size: 0
            });

        let (index, offset) = self.consecutive_section(
            ".text",
            1, //SHT_PROGBITS
            6, //write and read
            (ELF_SIZE + PROGRAM_SIZE) as TargetSize,
            program_length,
            0x1000 //page aligned
            );
        self.code_index = Some(index);

        let (index, offset) = self.consecutive_section(
            ".data",
            1, //SHT_PROGBITS
            3, //write and execute
            offset,
            self.program_data.len() as TargetSize,
            0x1000 //page aligned
            );
        self.data_index = Some(index);

        let shstr_name = ".shstrtab";
        let shstr_len = self.shstr_table.strings.len() + shstr_name.len();
        let (index, offset) = self.consecutive_section(
            shstr_name,
            3, //SHT_STRTAB
            0, //none
            offset,
            shstr_len as TargetSize,
            1
            );
        self.shstr_index = Some(index);

        offset+1
    }

    fn consecutive_section(
        &mut self,
        name: &str,
        section_type: u32,
        flags: TargetSize,
        offset: TargetSize,
        size: TargetSize,
        align: TargetSize
        ) -> (usize, TargetSize)
    {
        let offset =  Self::next_aligned(offset, 0x1000);

        self.shstr_table.add(name);
        self.sections.push(SectionInfo{
            name_index: self.shstr_table.next().unwrap(),
            section_type,
            flags,
            address: self.entry + offset,
            offset,
            size,
            link: 0,
            info: 0,
            align,
            fixed_size: 0
            });

        (self.sections.len()-1, offset + size)
    }

    fn create_programs(&mut self)
    {
        self.programs.push(self.build_program_section(
            5, //execute and read
            self.code_index.unwrap() //.text section
            ));

        self.programs.push(self.build_program_section(
            6, //read and write
            self.data_index.unwrap() //.data section
            ));

        self.programs.insert(0, self.build_program(
            4, //read only
            0, //references the elf header
            (ELF_SIZE + PROGRAM_SIZE * (self.programs.len() + 1)) as TargetSize,
            ));
    }

    fn build_program_section(&self, flags: u32, index: usize) -> ProgramInfo
    {
        self.build_program(
            flags,
            self.sections[index].offset,
            self.sections[index].size
            )
    }

    fn build_program(
        &self,
        flags: u32,
        offset: TargetSize,
        size: TargetSize
        ) -> ProgramInfo
    {
        ProgramInfo{
            header_type: 1,
            flags,
            offset,
            v_addr: self.entry + offset,
            p_addr: self.entry + offset,
            file_size: size,
            mem_size: size,
            align: 0x1000 //page size
            }
    }

    fn next_aligned(position: TargetSize, align: TargetSize) -> TargetSize
    {
        let modulus = position%align;

        if modulus!=0
        {
            position + align-modulus
        } else
        {
            position
        }
    }

    fn program_length(instructions: &[machine::Instruction]) -> TargetSize
    {
        instructions.iter().fold(0, |acc, instr|
        {
            acc + Bytable::as_bytes(instr).len() as TargetSize
        })
    }

    fn read_bytes(&mut self, instructions: &[machine::Instruction], addresser: &Addresser)
    {
        self.program_bytes.clear();
        instructions.iter().for_each(|instr|
        {
            self.program_bytes.extend(instr.as_bytes(Some(addresser)).iter());
        });
    }

    fn allocate_data(&mut self, data: &[machine::Variable])
    {
        data.iter().for_each(|var|
        {
            self.program_data.extend(var.as_bytes().iter());
        });
    }

    fn write_body(&mut self) -> io::Result<()>
    {
        self.pad_to_section(self.code_index.unwrap())?;
        self.file.write_all(&self.program_bytes)?;

        self.pad_to_section(self.data_index.unwrap())?;
        self.file.write_all(&self.program_data)?;

        self.pad_to_section(self.shstr_index.unwrap())?;
        self.file.write_all(&self.shstr_table.as_bytes())
    }

    fn pad_to_section(&mut self, index: usize) -> io::Result<()>
    {
        let section_offset = self.sections[index].offset;

        self.pad_to(section_offset)
    }

    fn pad_to(&mut self, offset: TargetSize) -> io::Result<()>
    {
        let pad_amount = offset - self.file.seek(SeekFrom::Current(0))? as TargetSize;

        self.file.write_all(&vec![0; pad_amount as usize])
    }
}