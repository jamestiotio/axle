use alloc::vec::Vec;
use alloc::{borrow::ToOwned, rc::Rc, string::String, vec};
use core::{
    fmt::{Debug, Display},
    mem,
};

#[cfg(feature = "run_in_axle")]
use axle_rt::println;
#[cfg(not(feature = "run_in_axle"))]
use std::{print, println};

use crate::{
    assembly_lexer::AssemblyLexer,
    assembly_parser::{AssemblyParser, BinarySection, EquExpressions, Labels, PotentialLabelTargets},
    new_try::{FileLayout, SymbolEntryType},
};

enum RexPrefixOption {
    Use64BitOperandSize,
    _UseRegisterFieldExtension,
    _UseIndexFieldExtension,
    _UseBaseFieldExtension,
}

struct RexPrefix;
impl RexPrefix {
    fn from_options(options: Vec<RexPrefixOption>) -> u8 {
        let mut out = 0b0100 << 4;
        for option in options.iter() {
            match option {
                RexPrefixOption::Use64BitOperandSize => out |= 1 << 3,
                RexPrefixOption::_UseRegisterFieldExtension => out |= 1 << 2,
                RexPrefixOption::_UseIndexFieldExtension => out |= 1 << 1,
                RexPrefixOption::_UseBaseFieldExtension => out |= 1 << 0,
            }
        }
        out
    }

    fn for_64bit_operand() -> u8 {
        Self::from_options(vec![RexPrefixOption::Use64BitOperandSize])
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Register {
    Rax,
    Rcx,
    Rdx,
    Rbx,
    Rsp,
    Rbp,
    Rsi,
    Rdi,
}

enum ModRmAddressingMode {
    RegisterDirect,
}

struct ModRmByte;
impl ModRmByte {
    fn register_index(register: Register) -> usize {
        match register {
            Register::Rax => 0b000,
            Register::Rcx => 0b001,
            Register::Rdx => 0b010,
            Register::Rbx => 0b011,
            Register::Rsp => 0b100,
            Register::Rbp => 0b101,
            Register::Rsi => 0b110,
            Register::Rdi => 0b111,
        }
    }
    fn from(addressing_mode: ModRmAddressingMode, register: Register, register2: Option<Register>) -> u8 {
        let mut out = 0;

        match addressing_mode {
            ModRmAddressingMode::RegisterDirect => out |= 0b11 << 6,
        }

        out |= Self::register_index(register);

        if let Some(register2) = register2 {
            out |= Self::register_index(register2) << 3;
        }

        out as _
    }
}

#[derive(Debug, Clone)]
pub enum DataSource {
    Literal(usize),
    NamedDataSymbol(String),
    RegisterContents(Register),
}

static mut NEXT_ATOM_ID: usize = 0;

pub fn next_atom_id() -> PotentialLabelTargetId {
    unsafe {
        let ret = NEXT_ATOM_ID;
        NEXT_ATOM_ID += 1;
        PotentialLabelTargetId(ret)
    }
}

pub struct MoveValueToRegister {
    id: PotentialLabelTargetId,
    dest_register: Register,
    source: DataSource,
}

impl MoveValueToRegister {
    pub fn new(dest_register: Register, source: DataSource) -> Self {
        Self {
            id: next_atom_id(),
            dest_register,
            source,
        }
    }
}

impl Instruction for MoveValueToRegister {
    fn render(&self, layout: &FileLayout) -> Vec<u8> {
        // REX prefix
        let mut out = vec![RexPrefix::for_64bit_operand()];

        if let DataSource::RegisterContents(register_name) = self.source {
            // MOV <reg>, <reg> opcode
            out.push(0x89);
            out.push(ModRmByte::from(ModRmAddressingMode::RegisterDirect, self.dest_register, Some(register_name)));
        } else {
            let value: u32 = match &self.source {
                DataSource::Literal(value) => *value as _,
                DataSource::NamedDataSymbol(symbol_name) => {
                    (match layout.symbol_type(symbol_name) {
                        SymbolEntryType::SymbolWithBackingData => layout.address_of_label_name(symbol_name),
                        SymbolEntryType::SymbolWithInlinedValue => layout.value_of_symbol(symbol_name),
                    }) as _
                }
                _ => panic!("Unexpected"),
            };
            // MOV <reg>, <mem> opcode
            out.push(0xc7);
            // Destination register
            out.push(ModRmByte::from(ModRmAddressingMode::RegisterDirect, self.dest_register, None));
            // Source value
            let mut value_bytes = value.to_le_bytes().to_owned().to_vec();
            out.append(&mut value_bytes);
        }

        out
    }
}

impl PotentialLabelTarget for MoveValueToRegister {
    fn container_section(&self) -> BinarySection {
        BinarySection::Text
    }

    fn len(&self) -> usize {
        match self.source {
            DataSource::RegisterContents(_) => 3,
            DataSource::Literal(_) | DataSource::NamedDataSymbol(_) => 7,
        }
    }

    fn render(&self, layout: &FileLayout) -> Vec<u8> {
        Instruction::render(self, layout)
    }

    fn id(&self) -> PotentialLabelTargetId {
        self.id
    }
}

impl Display for MoveValueToRegister {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("mov {:?}, {:?}", self.dest_register, self.source))
    }
}

pub struct Interrupt {
    id: PotentialLabelTargetId,
    vector: u8,
}

impl Interrupt {
    pub fn new(vector: u8) -> Self {
        Self { id: next_atom_id(), vector }
    }
}

impl Instruction for Interrupt {
    fn render(&self, _layout: &FileLayout) -> Vec<u8> {
        vec![
            // INT opcode
            0xcd,
            // INT vector
            self.vector,
        ]
    }
}

impl PotentialLabelTarget for Interrupt {
    fn container_section(&self) -> BinarySection {
        BinarySection::Text
    }

    fn len(&self) -> usize {
        2
    }

    fn render(&self, layout: &FileLayout) -> Vec<u8> {
        Instruction::render(self, layout)
    }

    fn id(&self) -> PotentialLabelTargetId {
        self.id
    }
}

impl Display for Interrupt {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("int 0x{:02x}", self.vector))
    }
}

#[derive(Debug, Clone)]
pub enum JumpTarget {
    Label(String),
}

pub struct Jump {
    id: PotentialLabelTargetId,
    target: JumpTarget,
}

impl Jump {
    pub fn new(target: JumpTarget) -> Self {
        Self { id: next_atom_id(), target }
    }
}

impl Instruction for Jump {
    fn render(&self, layout: &FileLayout) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        let JumpTarget::Label(label_name) = &self.target;
        // TODO(PT): Look for opportunities to use a JMP variant with a smaller encoded size
        // (i.e. if the named symbols is close by, use the JMP reloff8 variant)
        // This would require us to vary self.len()
        // We might have an intermediate stage that selects a more specific variant, if for example
        // we can see the jump target is only 4 atoms away
        println!("Assembling {self}");
        // JMP rel32off
        let distance_to_target = layout.distance_between_atom_id_and_label_name(PotentialLabelTarget::id(self), label_name) - (self.len() as isize);
        let distance_to_target: i32 = distance_to_target.try_into().unwrap();
        out.push(0xe9);
        let mut distance_as_bytes = distance_to_target.to_le_bytes().to_vec();
        assert!(distance_as_bytes.len() <= mem::size_of::<u32>(), "Must fit in a u32");
        distance_as_bytes.resize(mem::size_of::<u32>(), 0);
        out.append(&mut distance_as_bytes);
        out
    }
}

impl PotentialLabelTarget for Jump {
    fn container_section(&self) -> BinarySection {
        BinarySection::Text
    }

    fn id(&self) -> PotentialLabelTargetId {
        self.id
    }

    fn len(&self) -> usize {
        5
    }

    fn render(&self, layout: &FileLayout) -> Vec<u8> {
        Instruction::render(self, layout)
    }
}

impl Display for Jump {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("jmp {:?}", self.target))
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Eq, PartialOrd, Ord)]
pub struct PotentialLabelTargetId(pub usize);

impl Display for PotentialLabelTargetId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("AtomId({})", self.0))
    }
}

// An instruction, piece of constant data, or expression
pub trait PotentialLabelTarget: Display {
    fn container_section(&self) -> BinarySection;
    fn len(&self) -> usize;
    fn id(&self) -> PotentialLabelTargetId;
    fn render(&self, layout: &FileLayout) -> Vec<u8>;
}

pub trait Instruction: Display + PotentialLabelTarget {
    fn render(&self, layout: &FileLayout) -> Vec<u8>;
}

pub fn parse(_layout: &Rc<FileLayout>, source: &str) -> (Labels, EquExpressions, PotentialLabelTargets) {
    // Generate code and data from source
    let lexer = AssemblyLexer::new(source);
    let mut parser = AssemblyParser::new(lexer);
    let (labels, equ_expressions, data_units) = parser.parse();
    println!("[### Assembly + ELF rendering ###]");
    println!("Labels:\n{labels}");
    println!("Equ expressions:\n{equ_expressions}");
    println!("Data units:\n{data_units}");
    (labels, equ_expressions, data_units)
}
