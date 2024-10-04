use std::{collections::HashMap, env, fmt::Display, thread::Scope};

use lazy_static::lazy_static;
use regex::Regex;

/// Address code
#[derive(Debug, PartialEq)]
pub struct AddrCode {
    /// Required cycles
    pub cycles: u32,
    /// OP code
    pub opcode: u8,
}

#[derive(Debug, PartialEq)]
pub enum RawAddrCode {
    /// Accumulator
    Accumulator,
    /// Immediate
    Immediate(u8),
    /// Zero Page
    ZeroPage(u8),
    /// Zero Page X
    ZeroPageX(u8),
    /// Zero Page Y
    ZeroPageY(u8),
    /// Absolute
    Absolute(u16),
    /// Absolute X
    AbsoluteX(u16),
    /// Absolute Y
    AbsoluteY(u16),
    /// Indirect
    Indirect(u16),
    /// Indirect X
    IndirectX(u8),
    /// Indirect Y
    IndirectY(u8),
    /// Relative
    Relative(u8),
    /// Implicit
    Implicit,
}

impl RawAddrCode {
    pub fn parse_assembly_string(s: &str) -> Option<RawAddrCode> {
        let s = s.trim();

        lazy_static! {
            // Immediate Addressing Mode: #$01
            static ref RE_IMMEDIATE: Regex = Regex::new(r"^#\s*\$\s*([0-9A-Fa-f]+)$").unwrap();

            // Indexed Indirect Addressing Mode: ($01, X)
            static ref RE_INDIRECT_X: Regex = Regex::new(r"^\(\s*\$\s*([0-9A-Fa-f]+)\s*,\s*X\s*\)$").unwrap();

            // Indirect Indexed Addressing Mode: ($01), Y
            static ref RE_INDIRECT_Y: Regex = Regex::new(r"^\(\s*\$\s*([0-9A-Fa-f]+)\s*\)\s*,\s*Y$").unwrap();

            // Zero Page Indexed with X: $01, X
            static ref RE_ZERO_PAGE_X: Regex = Regex::new(r"^\$\s*([0-9A-Fa-f]+)\s*,\s*X$").unwrap();

            // Zero Page Indexed with Y: $01, Y
            static ref RE_ZERO_PAGE_Y: Regex = Regex::new(r"^\$\s*([0-9A-Fa-f]+)\s*,\s*Y$").unwrap();

            // Zero Page Addressing Mode: $01
            static ref RE_ZERO_PAGE: Regex = Regex::new(r"^\$\s*([0-9A-Fa-f]{1,2})$").unwrap();

            // Absolute Indexed with X: $1234, X
            static ref RE_ABSOLUTE_X: Regex = Regex::new(r"^\$\s*([0-9A-Fa-f]{3,4})\s*,\s*X$").unwrap();

            // Absolute Indexed with Y: $1234, Y
            static ref RE_ABSOLUTE_Y: Regex = Regex::new(r"^\$\s*([0-9A-Fa-f]{3,4})\s*,\s*Y$").unwrap();

            // Absolute Addressing Mode: $1234
            static ref RE_ABSOLUTE: Regex = Regex::new(r"^\$\s*([0-9A-Fa-f]{3,4})$").unwrap();

            // Indirect Addressing Mode: ($1234)
            static ref RE_INDIRECT: Regex = Regex::new(r"^\(\s*\$\s*([0-9A-Fa-f]{3,4})\s*\)$").unwrap();

            // Accumulator Addressing Mode: A
            static ref RE_ACCUMULATOR: Regex = Regex::new(r"^A$").unwrap();

            // Implicit Addressing Mode: (no operand)
            static ref RE_IMPLICIT: Regex = Regex::new(r"^$").unwrap();
        }

        if let Some(caps) = RE_IMMEDIATE.captures(s) {
            let value = u8::from_str_radix(&caps[1], 16).ok()?;
            Some(RawAddrCode::Immediate(value))
        } else if let Some(caps) = RE_INDIRECT_X.captures(s) {
            let value = u8::from_str_radix(&caps[1], 16).ok()?;
            Some(RawAddrCode::IndirectX(value))
        } else if let Some(caps) = RE_INDIRECT_Y.captures(s) {
            let value = u8::from_str_radix(&caps[1], 16).ok()?;
            Some(RawAddrCode::IndirectY(value))
        } else if let Some(caps) = RE_ZERO_PAGE_X.captures(s) {
            let value = u8::from_str_radix(&caps[1], 16).ok()?;
            Some(RawAddrCode::ZeroPageX(value))
        } else if let Some(caps) = RE_ZERO_PAGE_Y.captures(s) {
            let value = u8::from_str_radix(&caps[1], 16).ok()?;
            Some(RawAddrCode::ZeroPageY(value))
        } else if let Some(caps) = RE_ZERO_PAGE.captures(s) {
            let value = u8::from_str_radix(&caps[1], 16).ok()?;
            Some(RawAddrCode::ZeroPage(value))
        } else if let Some(caps) = RE_ABSOLUTE_X.captures(s) {
            let value = u16::from_str_radix(&caps[1], 16).ok()?;
            Some(RawAddrCode::AbsoluteX(value))
        } else if let Some(caps) = RE_ABSOLUTE_Y.captures(s) {
            let value = u16::from_str_radix(&caps[1], 16).ok()?;
            Some(RawAddrCode::AbsoluteY(value))
        } else if let Some(caps) = RE_ABSOLUTE.captures(s) {
            let value = u16::from_str_radix(&caps[1], 16).ok()?;
            Some(RawAddrCode::Absolute(value))
        } else if let Some(caps) = RE_INDIRECT.captures(s) {
            let value = u16::from_str_radix(&caps[1], 16).ok()?;
            Some(RawAddrCode::Indirect(value))
        } else if RE_ACCUMULATOR.is_match(s) {
            Some(RawAddrCode::Accumulator)
        } else if RE_IMPLICIT.is_match(s) {
            Some(RawAddrCode::Implicit)
        } else {
            None
        }
    }
}

/// Addressing modes
#[derive(Debug, PartialEq)]
pub enum AddrMode {
    /// Accumulator
    Accumulator(AddrCode),
    /// Immediate
    Immediate(AddrCode),
    /// Zero Page
    ZeroPage(AddrCode),
    /// Zero Page X
    ZeroPageX(AddrCode),
    /// Zero Page Y
    ZeroPageY(AddrCode),
    /// Absolute
    Absolute(AddrCode),
    /// Absolute X
    AbsoluteX(AddrCode),
    /// Absolute Y
    AbsoluteY(AddrCode),
    /// Indirect
    Indirect(AddrCode),
    /// Indirect X
    IndirectX(AddrCode),
    /// Indirect Y
    IndirectY(AddrCode),
    /// Relative
    Relative(AddrCode),
    /// Implicit
    Implicit(AddrCode),
}

impl Display for AddrMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddrMode::Accumulator(_) => write!(f, "Accumulator"),
            AddrMode::Immediate(_) => write!(f, "Immediate"),
            AddrMode::ZeroPage(_) => write!(f, "ZeroPage"),
            AddrMode::ZeroPageX(_) => write!(f, "ZeroPageX"),
            AddrMode::ZeroPageY(_) => write!(f, "ZeroPageY"),
            AddrMode::Absolute(_) => write!(f, "Absolute"),
            AddrMode::AbsoluteX(_) => write!(f, "AbsoluteX"),
            AddrMode::AbsoluteY(_) => write!(f, "AbsoluteY"),
            AddrMode::Indirect(_) => write!(f, "Indirect"),
            AddrMode::IndirectX(_) => write!(f, "IndirectX"),
            AddrMode::IndirectY(_) => write!(f, "IndirectY"),
            AddrMode::Relative(_) => write!(f, "Relative"),
            AddrMode::Implicit(_) => write!(f, "Implicit"),
        }
    }
}

/// Instructions
#[derive(Debug, PartialEq)]
pub enum Instructions {
    /// ADC
    ADC(AddrMode),
    /// AND
    AND(AddrMode),
    /// ASL
    ASL(AddrMode),
    /// BCC
    BCC(AddrMode),
    /// BCS
    BCS(AddrMode),
    /// BEQ
    BEQ(AddrMode),
    /// BIT
    BIT(AddrMode),
    /// BMI
    BMI(AddrMode),
    /// BNE
    BNE(AddrMode),
    /// BPL
    BPL(AddrMode),
    /// BRK
    BRK(AddrMode),
    /// BVC
    BVC(AddrMode),
    /// BVS
    BVS(AddrMode),
    /// CLC
    CLC(AddrMode),
    /// CLD
    CLD(AddrMode),
    /// CLI
    CLI(AddrMode),
    /// CLV
    CLV(AddrMode),
    /// CMP
    CMP(AddrMode),
    /// CPX
    CPX(AddrMode),
    /// CPY
    CPY(AddrMode),
    /// DEC
    DEC(AddrMode),
    /// DEX
    DEX(AddrMode),
    /// DEY
    DEY(AddrMode),
    /// EOR
    EOR(AddrMode),
    /// INC
    INC(AddrMode),
    /// INX
    INX(AddrMode),
    /// INY
    INY(AddrMode),
    /// JMP
    JMP(AddrMode),
    /// JSR
    JSR(AddrMode),
    /// LDA
    LDA(AddrMode),
    /// LDX
    LDX(AddrMode),
    /// LDY
    LDY(AddrMode),
    /// LSR
    LSR(AddrMode),
    /// NOP
    NOP(AddrMode),
    /// ORA
    ORA(AddrMode),
    /// PHA
    PHA(AddrMode),
    /// PHP
    PHP(AddrMode),
    /// PLA
    PLA(AddrMode),
    /// PLP
    PLP(AddrMode),
    /// ROL
    ROL(AddrMode),
    /// ROR
    ROR(AddrMode),
    /// RTI
    RTI(AddrMode),
    /// RTS
    RTS(AddrMode),
    /// SBC
    SBC(AddrMode),
    /// SEC
    SEC(AddrMode),
    /// SED
    SED(AddrMode),
    /// SEI
    SEI(AddrMode),
    /// STA
    STA(AddrMode),
    /// STX
    STX(AddrMode),
    /// STY
    STY(AddrMode),
    /// TAX
    TAX(AddrMode),
    /// TAY
    TAY(AddrMode),
    /// TSX
    TSX(AddrMode),
    /// TXA
    TXA(AddrMode),
    /// TXS
    TXS(AddrMode),
    /// TYA
    TYA(AddrMode),
}

impl Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instructions::ADC(a_mode) => write!(f, "ADC : {}", a_mode),
            Instructions::AND(a_mode) => write!(f, "AND : {}", a_mode),
            Instructions::ASL(a_mode) => write!(f, "ASL : {}", a_mode),
            Instructions::BCC(a_mode) => write!(f, "BCC : {}", a_mode),
            Instructions::BCS(a_mode) => write!(f, "BCS : {}", a_mode),
            Instructions::BEQ(a_mode) => write!(f, "BEQ : {}", a_mode),
            Instructions::BIT(a_mode) => write!(f, "BIT : {}", a_mode),
            Instructions::BMI(a_mode) => write!(f, "BMI : {}", a_mode),
            Instructions::BNE(a_mode) => write!(f, "BNE : {}", a_mode),
            Instructions::BPL(a_mode) => write!(f, "BPL : {}", a_mode),
            Instructions::BRK(a_mode) => write!(f, "BRK : {}", a_mode),
            Instructions::BVC(a_mode) => write!(f, "BVC : {}", a_mode),
            Instructions::BVS(a_mode) => write!(f, "BVS : {}", a_mode),
            Instructions::CLC(a_mode) => write!(f, "CLC : {}", a_mode),
            Instructions::CLD(a_mode) => write!(f, "CLD : {}", a_mode),
            Instructions::CLI(a_mode) => write!(f, "CLI : {}", a_mode),
            Instructions::CLV(a_mode) => write!(f, "CLV : {}", a_mode),
            Instructions::CMP(a_mode) => write!(f, "CMP : {}", a_mode),
            Instructions::CPX(a_mode) => write!(f, "CPX : {}", a_mode),
            Instructions::CPY(a_mode) => write!(f, "CPY : {}", a_mode),
            Instructions::DEC(a_mode) => write!(f, "DEC : {}", a_mode),
            Instructions::DEX(a_mode) => write!(f, "DEX : {}", a_mode),
            Instructions::DEY(a_mode) => write!(f, "DEY : {}", a_mode),
            Instructions::EOR(a_mode) => write!(f, "EOR : {}", a_mode),
            Instructions::INC(a_mode) => write!(f, "INC : {}", a_mode),
            Instructions::INX(a_mode) => write!(f, "INX : {}", a_mode),
            Instructions::INY(a_mode) => write!(f, "INY : {}", a_mode),
            Instructions::JMP(a_mode) => write!(f, "JMP : {}", a_mode),
            Instructions::JSR(a_mode) => write!(f, "JSR : {}", a_mode),
            Instructions::LDA(a_mode) => write!(f, "LDA : {}", a_mode),
            Instructions::LDX(a_mode) => write!(f, "LDX : {}", a_mode),
            Instructions::LDY(a_mode) => write!(f, "LDY : {}", a_mode),
            Instructions::LSR(a_mode) => write!(f, "LSR : {}", a_mode),
            Instructions::NOP(a_mode) => write!(f, "NOP : {}", a_mode),
            Instructions::ORA(a_mode) => write!(f, "ORA : {}", a_mode),
            Instructions::PHA(a_mode) => write!(f, "PHA : {}", a_mode),
            Instructions::PHP(a_mode) => write!(f, "PHP : {}", a_mode),
            Instructions::PLA(a_mode) => write!(f, "PLA : {}", a_mode),
            Instructions::PLP(a_mode) => write!(f, "PLP : {}", a_mode),
            Instructions::ROL(a_mode) => write!(f, "ROL : {}", a_mode),
            Instructions::ROR(a_mode) => write!(f, "ROR : {}", a_mode),
            Instructions::RTI(a_mode) => write!(f, "RTI : {}", a_mode),
            Instructions::RTS(a_mode) => write!(f, "RTS : {}", a_mode),
            Instructions::SBC(a_mode) => write!(f, "SBC : {}", a_mode),
            Instructions::SEC(a_mode) => write!(f, "SEC : {}", a_mode),
            Instructions::SED(a_mode) => write!(f, "SED : {}", a_mode),
            Instructions::SEI(a_mode) => write!(f, "SEI : {}", a_mode),
            Instructions::STA(a_mode) => write!(f, "STA : {}", a_mode),
            Instructions::STX(a_mode) => write!(f, "STX : {}", a_mode),
            Instructions::STY(a_mode) => write!(f, "STY : {}", a_mode),
            Instructions::TAX(a_mode) => write!(f, "TAX : {}", a_mode),
            Instructions::TAY(a_mode) => write!(f, "TAY : {}", a_mode),
            Instructions::TSX(a_mode) => write!(f, "TSX : {}", a_mode),
            Instructions::TXA(a_mode) => write!(f, "TXA : {}", a_mode),
            Instructions::TXS(a_mode) => write!(f, "TXS : {}", a_mode),
            Instructions::TYA(a_mode) => write!(f, "TYA : {}", a_mode),
        }
    }
}

impl Instructions {
    /// Resolve instruction from opcode
    /// ## Arguments
    /// * `opcode` - opcode to resolve [`u8`]
    /// ## Returns
    /// * [`Instructions`]
    /// ## Example
    /// ```
    /// use rusty_6502::asm;
    /// let opcode = 0x00;
    /// let instruction = asm::Instructions::resolve(opcode);
    /// assert_eq!(instruction, asm::Instructions::BRK(asm::AddrMode::Implicit(asm::AddrCode {
    ///     cycles: 7,
    ///     opcode: 0x00,
    /// })));
    /// ```
    pub fn resolve(opcode: u8) -> Instructions {
        match opcode {
            0x69 => Instructions::ADC(AddrMode::Immediate(AddrCode {
                cycles: 2,
                opcode: 0x69,
            })),
            0x65 => Instructions::ADC(AddrMode::ZeroPage(AddrCode {
                cycles: 3,
                opcode: 0x65,
            })),
            0x75 => Instructions::ADC(AddrMode::ZeroPageX(AddrCode {
                cycles: 4,
                opcode: 0x75,
            })),
            0x6D => Instructions::ADC(AddrMode::Absolute(AddrCode {
                cycles: 4,
                opcode: 0x6D,
            })),
            0x7D => Instructions::ADC(AddrMode::AbsoluteX(AddrCode {
                cycles: 4,
                opcode: 0x7D,
            })),
            0x79 => Instructions::ADC(AddrMode::AbsoluteY(AddrCode {
                cycles: 4,
                opcode: 0x79,
            })),
            0x61 => Instructions::ADC(AddrMode::IndirectX(AddrCode {
                cycles: 6,
                opcode: 0x61,
            })),
            0x71 => Instructions::ADC(AddrMode::IndirectY(AddrCode {
                cycles: 5,
                opcode: 0x71,
            })),
            0x29 => Instructions::AND(AddrMode::Immediate(AddrCode {
                cycles: 2,
                opcode: 0x29,
            })),
            0x25 => Instructions::AND(AddrMode::ZeroPage(AddrCode {
                cycles: 3,
                opcode: 0x25,
            })),
            0x35 => Instructions::AND(AddrMode::ZeroPageX(AddrCode {
                cycles: 4,
                opcode: 0x35,
            })),
            0x2D => Instructions::AND(AddrMode::Absolute(AddrCode {
                cycles: 4,
                opcode: 0x2D,
            })),
            0x3D => Instructions::AND(AddrMode::AbsoluteX(AddrCode {
                cycles: 4,
                opcode: 0x3D,
            })),
            0x39 => Instructions::AND(AddrMode::AbsoluteY(AddrCode {
                cycles: 4,
                opcode: 0x39,
            })),
            0x21 => Instructions::AND(AddrMode::IndirectX(AddrCode {
                cycles: 6,
                opcode: 0x21,
            })),
            0x31 => Instructions::AND(AddrMode::IndirectY(AddrCode {
                cycles: 5,
                opcode: 0x31,
            })),
            0x0A => Instructions::ASL(AddrMode::Accumulator(AddrCode {
                cycles: 2,
                opcode: 0x0A,
            })),
            0x06 => Instructions::ASL(AddrMode::ZeroPage(AddrCode {
                cycles: 5,
                opcode: 0x06,
            })),
            0x16 => Instructions::ASL(AddrMode::ZeroPageX(AddrCode {
                cycles: 6,
                opcode: 0x16,
            })),
            0x0E => Instructions::ASL(AddrMode::Absolute(AddrCode {
                cycles: 6,
                opcode: 0x0E,
            })),
            0x1E => Instructions::ASL(AddrMode::AbsoluteX(AddrCode {
                cycles: 7,
                opcode: 0x1E,
            })),
            0x90 => Instructions::BCC(AddrMode::Relative(AddrCode {
                cycles: 2,
                opcode: 0x90,
            })),
            0xB0 => Instructions::BCS(AddrMode::Relative(AddrCode {
                cycles: 2,
                opcode: 0xB0,
            })),
            0xF0 => Instructions::BEQ(AddrMode::Relative(AddrCode {
                cycles: 2,
                opcode: 0xF0,
            })),
            0x24 => Instructions::BIT(AddrMode::ZeroPage(AddrCode {
                cycles: 3,
                opcode: 0x24,
            })),
            0x2C => Instructions::BIT(AddrMode::Absolute(AddrCode {
                cycles: 4,
                opcode: 0x2C,
            })),
            0x30 => Instructions::BMI(AddrMode::Relative(AddrCode {
                cycles: 2,
                opcode: 0x30,
            })),
            0xD0 => Instructions::BNE(AddrMode::Relative(AddrCode {
                cycles: 2,
                opcode: 0xD0,
            })),
            0x10 => Instructions::BPL(AddrMode::Relative(AddrCode {
                cycles: 2,
                opcode: 0x10,
            })),
            0x00 => Instructions::BRK(AddrMode::Implicit(AddrCode {
                cycles: 7,
                opcode: 0x00,
            })),
            0x50 => Instructions::BVC(AddrMode::Relative(AddrCode {
                cycles: 2,
                opcode: 0x50,
            })),
            0x70 => Instructions::BVS(AddrMode::Relative(AddrCode {
                cycles: 2,
                opcode: 0x70,
            })),
            0x18 => Instructions::CLC(AddrMode::Implicit(AddrCode {
                cycles: 2,
                opcode: 0x18,
            })),
            0xD8 => Instructions::CLD(AddrMode::Implicit(AddrCode {
                cycles: 2,
                opcode: 0xD8,
            })),
            0x58 => Instructions::CLI(AddrMode::Implicit(AddrCode {
                cycles: 2,
                opcode: 0x58,
            })),
            0xB8 => Instructions::CLV(AddrMode::Implicit(AddrCode {
                cycles: 2,
                opcode: 0xB8,
            })),
            0xC9 => Instructions::CMP(AddrMode::Immediate(AddrCode {
                cycles: 2,
                opcode: 0xC9,
            })),
            0xC5 => Instructions::CMP(AddrMode::ZeroPage(AddrCode {
                cycles: 3,
                opcode: 0xC5,
            })),
            0xD5 => Instructions::CMP(AddrMode::ZeroPageX(AddrCode {
                cycles: 4,
                opcode: 0xD5,
            })),
            0xCD => Instructions::CMP(AddrMode::Absolute(AddrCode {
                cycles: 4,
                opcode: 0xCD,
            })),
            0xDD => Instructions::CMP(AddrMode::AbsoluteX(AddrCode {
                cycles: 4,
                opcode: 0xDD,
            })),
            0xD9 => Instructions::CMP(AddrMode::AbsoluteY(AddrCode {
                cycles: 4,
                opcode: 0xD9,
            })),
            0xC1 => Instructions::CMP(AddrMode::IndirectX(AddrCode {
                cycles: 6,
                opcode: 0xC1,
            })),
            0xD1 => Instructions::CMP(AddrMode::IndirectY(AddrCode {
                cycles: 5,
                opcode: 0xD1,
            })),
            0xE0 => Instructions::CPX(AddrMode::Immediate(AddrCode {
                cycles: 2,
                opcode: 0xE0,
            })),
            0xE4 => Instructions::CPX(AddrMode::ZeroPage(AddrCode {
                cycles: 3,
                opcode: 0xE4,
            })),
            0xEC => Instructions::CPX(AddrMode::Absolute(AddrCode {
                cycles: 4,
                opcode: 0xEC,
            })),
            0xC0 => Instructions::CPY(AddrMode::Immediate(AddrCode {
                cycles: 2,
                opcode: 0xC0,
            })),
            0xC4 => Instructions::CPY(AddrMode::ZeroPage(AddrCode {
                cycles: 3,
                opcode: 0xC4,
            })),
            0xCC => Instructions::CPY(AddrMode::Absolute(AddrCode {
                cycles: 4,
                opcode: 0xCC,
            })),
            0xC6 => Instructions::DEC(AddrMode::ZeroPage(AddrCode {
                cycles: 5,
                opcode: 0xC6,
            })),
            0xD6 => Instructions::DEC(AddrMode::ZeroPageX(AddrCode {
                cycles: 6,
                opcode: 0xD6,
            })),
            0xCE => Instructions::DEC(AddrMode::Absolute(AddrCode {
                cycles: 6,
                opcode: 0xCE,
            })),
            0xDE => Instructions::DEC(AddrMode::AbsoluteX(AddrCode {
                cycles: 7,
                opcode: 0xDE,
            })),
            0xCA => Instructions::DEX(AddrMode::Implicit(AddrCode {
                cycles: 2,
                opcode: 0xCA,
            })),
            0x88 => Instructions::DEY(AddrMode::Implicit(AddrCode {
                cycles: 2,
                opcode: 0x88,
            })),
            0x49 => Instructions::EOR(AddrMode::Immediate(AddrCode {
                cycles: 2,
                opcode: 0x49,
            })),
            0x45 => Instructions::EOR(AddrMode::ZeroPage(AddrCode {
                cycles: 3,
                opcode: 0x45,
            })),
            0x55 => Instructions::EOR(AddrMode::ZeroPageX(AddrCode {
                cycles: 4,
                opcode: 0x55,
            })),
            0x4D => Instructions::EOR(AddrMode::Absolute(AddrCode {
                cycles: 4,
                opcode: 0x4D,
            })),
            0x5D => Instructions::EOR(AddrMode::AbsoluteX(AddrCode {
                cycles: 4,
                opcode: 0x5D,
            })),
            0x59 => Instructions::EOR(AddrMode::AbsoluteY(AddrCode {
                cycles: 4,
                opcode: 0x59,
            })),
            0x41 => Instructions::EOR(AddrMode::IndirectX(AddrCode {
                cycles: 6,
                opcode: 0x41,
            })),
            0x51 => Instructions::EOR(AddrMode::IndirectY(AddrCode {
                cycles: 5,
                opcode: 0x51,
            })),
            0xE6 => Instructions::INC(AddrMode::ZeroPage(AddrCode {
                cycles: 5,
                opcode: 0xE6,
            })),
            0xF6 => Instructions::INC(AddrMode::ZeroPageX(AddrCode {
                cycles: 6,
                opcode: 0xF6,
            })),
            0xEE => Instructions::INC(AddrMode::Absolute(AddrCode {
                cycles: 6,
                opcode: 0xEE,
            })),
            0xFE => Instructions::INC(AddrMode::AbsoluteX(AddrCode {
                cycles: 7,
                opcode: 0xFE,
            })),
            0xE8 => Instructions::INX(AddrMode::Implicit(AddrCode {
                cycles: 2,
                opcode: 0xE8,
            })),
            0xC8 => Instructions::INY(AddrMode::Implicit(AddrCode {
                cycles: 2,
                opcode: 0xC8,
            })),
            0x4C => Instructions::JMP(AddrMode::Absolute(AddrCode {
                cycles: 3,
                opcode: 0x4C,
            })),
            0x6C => Instructions::JMP(AddrMode::Indirect(AddrCode {
                cycles: 5,
                opcode: 0x6C,
            })),
            0x20 => Instructions::JSR(AddrMode::Absolute(AddrCode {
                cycles: 6,
                opcode: 0x20,
            })),
            0xA9 => Instructions::LDA(AddrMode::Immediate(AddrCode {
                cycles: 2,
                opcode: 0xA9,
            })),
            0xA5 => Instructions::LDA(AddrMode::ZeroPage(AddrCode {
                cycles: 3,
                opcode: 0xA5,
            })),
            0xB5 => Instructions::LDA(AddrMode::ZeroPageX(AddrCode {
                cycles: 4,
                opcode: 0xB5,
            })),
            0xAD => Instructions::LDA(AddrMode::Absolute(AddrCode {
                cycles: 4,
                opcode: 0xAD,
            })),
            0xBD => Instructions::LDA(AddrMode::AbsoluteX(AddrCode {
                cycles: 4,
                opcode: 0xBD,
            })),
            0xB9 => Instructions::LDA(AddrMode::AbsoluteY(AddrCode {
                cycles: 4,
                opcode: 0xB9,
            })),
            0xA1 => Instructions::LDA(AddrMode::IndirectX(AddrCode {
                cycles: 6,
                opcode: 0xA1,
            })),
            0xB1 => Instructions::LDA(AddrMode::IndirectY(AddrCode {
                cycles: 5,
                opcode: 0xB1,
            })),
            0xA2 => Instructions::LDX(AddrMode::Immediate(AddrCode {
                cycles: 2,
                opcode: 0xA2,
            })),
            0xA6 => Instructions::LDX(AddrMode::ZeroPage(AddrCode {
                cycles: 3,
                opcode: 0xA6,
            })),
            0xB6 => Instructions::LDX(AddrMode::ZeroPageY(AddrCode {
                cycles: 4,
                opcode: 0xB6,
            })),
            0xAE => Instructions::LDX(AddrMode::Absolute(AddrCode {
                cycles: 4,
                opcode: 0xAE,
            })),
            0xBE => Instructions::LDX(AddrMode::AbsoluteY(AddrCode {
                cycles: 4,
                opcode: 0xBE,
            })),
            0xA0 => Instructions::LDY(AddrMode::Immediate(AddrCode {
                cycles: 2,
                opcode: 0xA0,
            })),
            0xA4 => Instructions::LDY(AddrMode::ZeroPage(AddrCode {
                cycles: 3,
                opcode: 0xA4,
            })),
            0xB4 => Instructions::LDY(AddrMode::ZeroPageX(AddrCode {
                cycles: 4,
                opcode: 0xB4,
            })),
            0xAC => Instructions::LDY(AddrMode::Absolute(AddrCode {
                cycles: 4,
                opcode: 0xAC,
            })),
            0xBC => Instructions::LDY(AddrMode::AbsoluteX(AddrCode {
                cycles: 4,
                opcode: 0xBC,
            })),
            0x4A => Instructions::LSR(AddrMode::Accumulator(AddrCode {
                cycles: 2,
                opcode: 0x4A,
            })),
            0x46 => Instructions::LSR(AddrMode::ZeroPage(AddrCode {
                cycles: 5,
                opcode: 0x46,
            })),
            0x56 => Instructions::LSR(AddrMode::ZeroPageX(AddrCode {
                cycles: 6,
                opcode: 0x56,
            })),
            0x4E => Instructions::LSR(AddrMode::Absolute(AddrCode {
                cycles: 6,
                opcode: 0x4E,
            })),
            0x5E => Instructions::LSR(AddrMode::AbsoluteX(AddrCode {
                cycles: 7,
                opcode: 0x5E,
            })),
            0xEA => Instructions::NOP(AddrMode::Implicit(AddrCode {
                cycles: 2,
                opcode: 0xEA,
            })),
            0x09 => Instructions::ORA(AddrMode::Immediate(AddrCode {
                cycles: 2,
                opcode: 0x09,
            })),
            0x05 => Instructions::ORA(AddrMode::ZeroPage(AddrCode {
                cycles: 3,
                opcode: 0x05,
            })),
            0x15 => Instructions::ORA(AddrMode::ZeroPageX(AddrCode {
                cycles: 4,
                opcode: 0x15,
            })),
            0x0D => Instructions::ORA(AddrMode::Absolute(AddrCode {
                cycles: 4,
                opcode: 0x0D,
            })),
            0x1D => Instructions::ORA(AddrMode::AbsoluteX(AddrCode {
                cycles: 4,
                opcode: 0x1D,
            })),
            0x19 => Instructions::ORA(AddrMode::AbsoluteY(AddrCode {
                cycles: 4,
                opcode: 0x19,
            })),
            0x01 => Instructions::ORA(AddrMode::IndirectX(AddrCode {
                cycles: 6,
                opcode: 0x01,
            })),
            0x11 => Instructions::ORA(AddrMode::IndirectY(AddrCode {
                cycles: 5,
                opcode: 0x11,
            })),
            0x48 => Instructions::PHA(AddrMode::Implicit(AddrCode {
                cycles: 3,
                opcode: 0x48,
            })),
            0x08 => Instructions::PHP(AddrMode::Implicit(AddrCode {
                cycles: 3,
                opcode: 0x08,
            })),
            0x68 => Instructions::PLA(AddrMode::Implicit(AddrCode {
                cycles: 4,
                opcode: 0x68,
            })),
            0x28 => Instructions::PLP(AddrMode::Implicit(AddrCode {
                cycles: 4,
                opcode: 0x28,
            })),
            0x2A => Instructions::ROL(AddrMode::Accumulator(AddrCode {
                cycles: 2,
                opcode: 0x2A,
            })),
            0x26 => Instructions::ROL(AddrMode::ZeroPage(AddrCode {
                cycles: 5,
                opcode: 0x26,
            })),
            0x36 => Instructions::ROL(AddrMode::ZeroPageX(AddrCode {
                cycles: 6,
                opcode: 0x36,
            })),
            0x2E => Instructions::ROL(AddrMode::Absolute(AddrCode {
                cycles: 6,
                opcode: 0x2E,
            })),
            0x3E => Instructions::ROL(AddrMode::AbsoluteX(AddrCode {
                cycles: 7,
                opcode: 0x3E,
            })),
            0x6A => Instructions::ROR(AddrMode::Accumulator(AddrCode {
                cycles: 2,
                opcode: 0x6A,
            })),
            0x66 => Instructions::ROR(AddrMode::ZeroPage(AddrCode {
                cycles: 5,
                opcode: 0x66,
            })),
            0x76 => Instructions::ROR(AddrMode::ZeroPageX(AddrCode {
                cycles: 6,
                opcode: 0x76,
            })),
            0x6E => Instructions::ROR(AddrMode::Absolute(AddrCode {
                cycles: 6,
                opcode: 0x6E,
            })),
            0x7E => Instructions::ROR(AddrMode::AbsoluteX(AddrCode {
                cycles: 7,
                opcode: 0x7E,
            })),
            0x40 => Instructions::RTI(AddrMode::Implicit(AddrCode {
                cycles: 6,
                opcode: 0x40,
            })),
            0x60 => Instructions::RTS(AddrMode::Implicit(AddrCode {
                cycles: 6,
                opcode: 0x60,
            })),
            0xE9 => Instructions::SBC(AddrMode::Immediate(AddrCode {
                cycles: 2,
                opcode: 0xE9,
            })),
            0xE5 => Instructions::SBC(AddrMode::ZeroPage(AddrCode {
                cycles: 3,
                opcode: 0xE5,
            })),
            0xF5 => Instructions::SBC(AddrMode::ZeroPageX(AddrCode {
                cycles: 4,
                opcode: 0xF5,
            })),
            0xED => Instructions::SBC(AddrMode::Absolute(AddrCode {
                cycles: 4,
                opcode: 0xED,
            })),
            0xFD => Instructions::SBC(AddrMode::AbsoluteX(AddrCode {
                cycles: 4,
                opcode: 0xFD,
            })),
            0xF9 => Instructions::SBC(AddrMode::AbsoluteY(AddrCode {
                cycles: 4,
                opcode: 0xF9,
            })),
            0xE1 => Instructions::SBC(AddrMode::IndirectX(AddrCode {
                cycles: 6,
                opcode: 0xE1,
            })),
            0xF1 => Instructions::SBC(AddrMode::IndirectY(AddrCode {
                cycles: 5,
                opcode: 0xF1,
            })),
            0x38 => Instructions::SEC(AddrMode::Implicit(AddrCode {
                cycles: 2,
                opcode: 0x38,
            })),
            0xF8 => Instructions::SED(AddrMode::Implicit(AddrCode {
                cycles: 2,
                opcode: 0xF8,
            })),
            0x78 => Instructions::SEI(AddrMode::Implicit(AddrCode {
                cycles: 2,
                opcode: 0x78,
            })),
            0x85 => Instructions::STA(AddrMode::ZeroPage(AddrCode {
                cycles: 3,
                opcode: 0x85,
            })),
            0x95 => Instructions::STA(AddrMode::ZeroPageX(AddrCode {
                cycles: 4,
                opcode: 0x95,
            })),
            0x8D => Instructions::STA(AddrMode::Absolute(AddrCode {
                cycles: 4,
                opcode: 0x8D,
            })),
            0x9D => Instructions::STA(AddrMode::AbsoluteX(AddrCode {
                cycles: 5,
                opcode: 0x9D,
            })),
            0x99 => Instructions::STA(AddrMode::AbsoluteY(AddrCode {
                cycles: 5,
                opcode: 0x99,
            })),
            0x81 => Instructions::STA(AddrMode::IndirectX(AddrCode {
                cycles: 6,
                opcode: 0x81,
            })),
            0x91 => Instructions::STA(AddrMode::IndirectY(AddrCode {
                cycles: 6,
                opcode: 0x91,
            })),
            0x86 => Instructions::STX(AddrMode::ZeroPage(AddrCode {
                cycles: 3,
                opcode: 0x86,
            })),
            0x96 => Instructions::STX(AddrMode::ZeroPageY(AddrCode {
                cycles: 4,
                opcode: 0x96,
            })),
            0x8E => Instructions::STX(AddrMode::Absolute(AddrCode {
                cycles: 4,
                opcode: 0x8E,
            })),
            0x84 => Instructions::STY(AddrMode::ZeroPage(AddrCode {
                cycles: 3,
                opcode: 0x84,
            })),
            0x94 => Instructions::STY(AddrMode::ZeroPageX(AddrCode {
                cycles: 4,
                opcode: 0x94,
            })),
            0x8C => Instructions::STY(AddrMode::Absolute(AddrCode {
                cycles: 4,
                opcode: 0x8C,
            })),
            0xAA => Instructions::TAX(AddrMode::Implicit(AddrCode {
                cycles: 2,
                opcode: 0xAA,
            })),
            0xA8 => Instructions::TAY(AddrMode::Implicit(AddrCode {
                cycles: 2,
                opcode: 0xA8,
            })),
            0xBA => Instructions::TSX(AddrMode::Implicit(AddrCode {
                cycles: 2,
                opcode: 0xBA,
            })),
            0x8A => Instructions::TXA(AddrMode::Implicit(AddrCode {
                cycles: 2,
                opcode: 0x8A,
            })),
            0x9A => Instructions::TXS(AddrMode::Implicit(AddrCode {
                cycles: 2,
                opcode: 0x9A,
            })),
            0x98 => Instructions::TYA(AddrMode::Implicit(AddrCode {
                cycles: 2,
                opcode: 0x98,
            })),
            _ => panic!("Wrong opcode: {}", opcode),
        }
    }

    pub fn resolve_string(opcode: &str) -> Option<Vec<u8>> {
        let parts = opcode.split(" ").collect::<Vec<_>>();
        let addr_mode = RawAddrCode::parse_assembly_string(parts[1..].join(" ").as_str());

        if addr_mode.is_none() {
            return None;
        }
        let addr_mode = addr_mode.unwrap();

        let code = match parts[0] {
            "ADC" => match addr_mode {
                RawAddrCode::Immediate(p) => vec![0x69, p],
                RawAddrCode::ZeroPage(p) => vec![0x65, p],
                RawAddrCode::ZeroPageX(p) => vec![0x75, p],
                RawAddrCode::Absolute(p) => vec![0x6D, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::AbsoluteX(p) => vec![0x7D, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::AbsoluteY(p) => vec![0x79, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::IndirectX(p) => vec![0x61, p],
                RawAddrCode::IndirectY(p) => vec![0x71, p],
                _ => panic!("Invalid addressing mode for ADC"),
            },
            "AND" => match addr_mode {
                RawAddrCode::Immediate(p) => vec![0x29, p],
                RawAddrCode::ZeroPage(p) => vec![0x25, p],
                RawAddrCode::ZeroPageX(p) => vec![0x35, p],
                RawAddrCode::Absolute(p) => vec![0x2D, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::AbsoluteX(p) => vec![0x3D, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::AbsoluteY(p) => vec![0x39, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::IndirectX(p) => vec![0x21, p],
                RawAddrCode::IndirectY(p) => vec![0x31, p],
                _ => panic!("Invalid addressing mode for AND"),
            },
            "ASL" => match addr_mode {
                RawAddrCode::AbsoluteY(p) => vec![0x0A, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::ZeroPage(p) => vec![0x06, p],
                RawAddrCode::ZeroPageX(p) => vec![0x16, p],
                RawAddrCode::Absolute(p) => vec![0x0E, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::AbsoluteX(p) => vec![0x1E, (p & 0xFF) as u8, (p >> 8) as u8],
                _ => panic!("Invalid addressing mode for ASL"),
            },
            "BCC" => match addr_mode {
                RawAddrCode::Relative(p) => vec![0x90, p],
                _ => panic!("Invalid addressing mode for BCC"),
            },
            "BCS" => match addr_mode {
                RawAddrCode::Relative(p) => vec![0xB0, p],
                _ => panic!("Invalid addressing mode for BCS"),
            },
            "BEQ" => match addr_mode {
                RawAddrCode::Relative(p) => vec![0xF0, p],
                _ => panic!("Invalid addressing mode for BEQ"),
            },
            "BIT" => match addr_mode {
                RawAddrCode::ZeroPage(p) => vec![0x24, p],
                RawAddrCode::Absolute(p) => vec![0x2C, (p & 0xFF) as u8, (p >> 8) as u8],
                _ => panic!("Invalid addressing mode for BIT"),
            },
            "BMI" => match addr_mode {
                RawAddrCode::Relative(p) => vec![0x30, p],
                _ => panic!("Invalid addressing mode for BMI"),
            },
            "BNE" => match addr_mode {
                RawAddrCode::Relative(p) => vec![0xD0, p],
                _ => panic!("Invalid addressing mode for BNE"),
            },
            "BPL" => match addr_mode {
                RawAddrCode::Relative(p) => vec![0x10, p],
                _ => panic!("Invalid addressing mode for BPL"),
            },
            "BRK" => match addr_mode {
                RawAddrCode::Implicit => vec![0x00],
                _ => panic!("Invalid addressing mode for BRK"),
            },
            "BVC" => match addr_mode {
                RawAddrCode::Relative(p) => vec![0x50, p],
                _ => panic!("Invalid addressing mode for BVC"),
            },
            "BVS" => match addr_mode {
                RawAddrCode::Relative(p) => vec![0x70, p],
                _ => panic!("Invalid addressing mode for BVS"),
            },
            "CLC" => match addr_mode {
                RawAddrCode::Implicit => vec![0x18],
                _ => panic!("Invalid addressing mode for CLC"),
            },
            "CLD" => match addr_mode {
                RawAddrCode::Implicit => vec![0xD8],
                _ => panic!("Invalid addressing mode for CLD"),
            },
            "CLI" => match addr_mode {
                RawAddrCode::Implicit => vec![0x58],
                _ => panic!("Invalid addressing mode for CLI"),
            },
            "CLV" => match addr_mode {
                RawAddrCode::Implicit => vec![0xB8],
                _ => panic!("Invalid addressing mode for CLV"),
            },
            "CMP" => match addr_mode {
                RawAddrCode::Immediate(p) => vec![0xC9, p],
                RawAddrCode::ZeroPage(p) => vec![0xC5, p],
                RawAddrCode::ZeroPageX(p) => vec![0xD5, p],
                RawAddrCode::Absolute(p) => vec![0xCD, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::AbsoluteX(p) => vec![0xDD, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::AbsoluteY(p) => vec![0xD9, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::IndirectX(p) => vec![0xC1, p],
                RawAddrCode::IndirectY(p) => vec![0xD1, p],
                _ => panic!("Invalid addressing mode for CMP"),
            },
            "CPX" => match addr_mode {
                RawAddrCode::Immediate(p) => vec![0xE0, p],
                RawAddrCode::ZeroPage(p) => vec![0xE4, p],
                RawAddrCode::Absolute(p) => vec![0xEC, (p & 0xFF) as u8, (p >> 8) as u8],
                _ => panic!("Invalid addressing mode for CPX"),
            },
            "CPY" => match addr_mode {
                RawAddrCode::Immediate(p) => vec![0xC0, p],
                RawAddrCode::ZeroPage(p) => vec![0xC4, p],
                RawAddrCode::Absolute(p) => vec![0xCC, (p & 0xFF) as u8, (p >> 8) as u8],
                _ => panic!("Invalid addressing mode for CPY"),
            },
            "DEC" => match addr_mode {
                RawAddrCode::ZeroPage(p) => vec![0xC6, p],
                RawAddrCode::ZeroPageX(p) => vec![0xD6, p],
                RawAddrCode::Absolute(p) => vec![0xCE, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::AbsoluteX(p) => vec![0xDE, (p & 0xFF) as u8, (p >> 8) as u8],
                _ => panic!("Invalid addressing mode for DEC"),
            },
            "DEX" => match addr_mode {
                RawAddrCode::Implicit => vec![0xCA],
                _ => panic!("Invalid addressing mode for DEX"),
            },
            "DEY" => match addr_mode {
                RawAddrCode::Implicit => vec![0x88],
                _ => panic!("Invalid addressing mode for DEY"),
            },
            "EOR" => match addr_mode {
                RawAddrCode::Immediate(p) => vec![0x49, p],
                RawAddrCode::ZeroPage(p) => vec![0x45, p],
                RawAddrCode::ZeroPageX(p) => vec![0x55, p],
                RawAddrCode::Absolute(p) => vec![0x4D, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::AbsoluteX(p) => vec![0x5D, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::AbsoluteY(p) => vec![0x59, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::IndirectX(p) => vec![0x41, p],
                RawAddrCode::IndirectY(p) => vec![0x51, p],
                _ => panic!("Invalid addressing mode for EOR"),
            },
            "INC" => match addr_mode {
                RawAddrCode::ZeroPage(p) => vec![0xE6, p],
                RawAddrCode::ZeroPageX(p) => vec![0xF6, p],
                RawAddrCode::Absolute(p) => vec![0xEE, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::AbsoluteX(p) => vec![0xFE, (p & 0xFF) as u8, (p >> 8) as u8],
                _ => panic!("Invalid addressing mode for INC"),
            },
            "INX" => match addr_mode {
                RawAddrCode::Implicit => vec![0xE8],
                _ => panic!("Invalid addressing mode for INX"),
            },
            "INY" => match addr_mode {
                RawAddrCode::Implicit => vec![0xC8],
                _ => panic!("Invalid addressing mode for INY"),
            },
            "JMP" => match addr_mode {
                RawAddrCode::Absolute(p) => vec![0x4C, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::Indirect(p) => vec![0x6C, (p & 0xFF) as u8, (p >> 8) as u8],
                _ => panic!("Invalid addressing mode for JMP"),
            },
            "JSR" => match addr_mode {
                RawAddrCode::Absolute(p) => vec![0x20, (p & 0xFF) as u8, (p >> 8) as u8],
                _ => panic!("Invalid addressing mode for JSR"),
            },
            "LDA" => match addr_mode {
                RawAddrCode::Immediate(p) => vec![0xA9, p],
                RawAddrCode::ZeroPage(p) => vec![0xA5, p],
                RawAddrCode::ZeroPageX(p) => vec![0xB5, p],
                RawAddrCode::Absolute(p) => vec![0xAD, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::AbsoluteX(p) => vec![0xBD, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::AbsoluteY(p) => vec![0xB9, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::IndirectX(p) => vec![0xA1, p],
                RawAddrCode::IndirectY(p) => vec![0xB1, p],
                _ => panic!("Invalid addressing mode for LDA"),
            },
            "LDX" => match addr_mode {
                RawAddrCode::Immediate(p) => vec![0xA2, p],
                RawAddrCode::ZeroPage(p) => vec![0xA6, p],
                RawAddrCode::ZeroPageY(p) => vec![0xB6, p],
                RawAddrCode::Absolute(p) => vec![0xAE, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::AbsoluteY(p) => vec![0xBE, (p & 0xFF) as u8, (p >> 8) as u8],
                _ => panic!("Invalid addressing mode for LDX"),
            },
            "LDY" => match addr_mode {
                RawAddrCode::Immediate(p) => vec![0xA0, p],
                RawAddrCode::ZeroPage(p) => vec![0xA4, p],
                RawAddrCode::ZeroPageX(p) => vec![0xB4, p],
                RawAddrCode::Absolute(p) => vec![0xAC, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::AbsoluteX(p) => vec![0xBC, (p & 0xFF) as u8, (p >> 8) as u8],
                _ => panic!("Invalid addressing mode for LDY"),
            },
            "LSR" => match addr_mode {
                RawAddrCode::AbsoluteY(p) => vec![0x4A, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::ZeroPage(p) => vec![0x46, p],
                RawAddrCode::ZeroPageX(p) => vec![0x56, p],
                RawAddrCode::Absolute(p) => vec![0x4E, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::AbsoluteX(p) => vec![0x5E, (p & 0xFF) as u8, (p >> 8) as u8],
                _ => panic!("Invalid addressing mode for LSR"),
            },
            "NOP" => match addr_mode {
                RawAddrCode::Implicit => vec![0xEA],
                _ => panic!("Invalid addressing mode for NOP"),
            },
            "ORA" => match addr_mode {
                RawAddrCode::Immediate(p) => vec![0x09, p],
                RawAddrCode::ZeroPage(p) => vec![0x05, p],
                RawAddrCode::ZeroPageX(p) => vec![0x15, p],
                RawAddrCode::Absolute(p) => vec![0x0D, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::AbsoluteX(p) => vec![0x1D, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::AbsoluteY(p) => vec![0x19, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::IndirectX(p) => vec![0x01, p],
                RawAddrCode::IndirectY(p) => vec![0x11, p],
                _ => panic!("Invalid addressing mode for ORA"),
            },
            "PHA" => match addr_mode {
                RawAddrCode::Implicit => vec![0x48],
                _ => panic!("Invalid addressing mode for PHA"),
            },
            "PHP" => match addr_mode {
                RawAddrCode::Implicit => vec![0x08],
                _ => panic!("Invalid addressing mode for PHP"),
            },
            "PLA" => match addr_mode {
                RawAddrCode::Implicit => vec![0x68],
                _ => panic!("Invalid addressing mode for PLA"),
            },
            "PLP" => match addr_mode {
                RawAddrCode::Implicit => vec![0x28],
                _ => panic!("Invalid addressing mode for PLP"),
            },
            "ROL" => match addr_mode {
                RawAddrCode::AbsoluteY(p) => vec![0x2A, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::ZeroPage(p) => vec![0x26, p],
                RawAddrCode::ZeroPageX(p) => vec![0x36, p],
                RawAddrCode::Absolute(p) => vec![0x2E, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::AbsoluteX(p) => vec![0x3E, (p & 0xFF) as u8, (p >> 8) as u8],
                _ => panic!("Invalid addressing mode for ROL"),
            },
            "ROR" => match addr_mode {
                RawAddrCode::AbsoluteY(p) => vec![0x6A, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::ZeroPage(p) => vec![0x66, p],
                RawAddrCode::ZeroPageX(p) => vec![0x76, p],
                RawAddrCode::Absolute(p) => vec![0x6E, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::AbsoluteX(p) => vec![0x7E, (p & 0xFF) as u8, (p >> 8) as u8],
                _ => panic!("Invalid addressing mode for ROR"),
            },
            "RTI" => match addr_mode {
                RawAddrCode::Implicit => vec![0x40],
                _ => panic!("Invalid addressing mode for RTI"),
            },
            "RTS" => match addr_mode {
                RawAddrCode::Implicit => vec![0x60],
                _ => panic!("Invalid addressing mode for RTS"),
            },
            "SBC" => match addr_mode {
                RawAddrCode::Immediate(p) => vec![0xE9, p],
                RawAddrCode::ZeroPage(p) => vec![0xE5, p],
                RawAddrCode::ZeroPageX(p) => vec![0xF5, p],
                RawAddrCode::Absolute(p) => vec![0xED, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::AbsoluteX(p) => vec![0xFD, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::AbsoluteY(p) => vec![0xF9, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::IndirectX(p) => vec![0xE1, p],
                RawAddrCode::IndirectY(p) => vec![0xF1, p],
                _ => panic!("Invalid addressing mode for SBC"),
            },
            "SEC" => match addr_mode {
                RawAddrCode::Implicit => vec![0x38],
                _ => panic!("Invalid addressing mode for SEC"),
            },
            "SED" => match addr_mode {
                RawAddrCode::Implicit => vec![0xF8],
                _ => panic!("Invalid addressing mode for SED"),
            },
            "SEI" => match addr_mode {
                RawAddrCode::Implicit => vec![0x78],
                _ => panic!("Invalid addressing mode for SEI"),
            },
            "STA" => match addr_mode {
                RawAddrCode::ZeroPage(p) => vec![0x85, p],
                RawAddrCode::ZeroPageX(p) => vec![0x95, p],
                RawAddrCode::Absolute(p) => vec![0x8D, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::AbsoluteX(p) => vec![0x9D, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::AbsoluteY(p) => vec![0x99, (p & 0xFF) as u8, (p >> 8) as u8],
                RawAddrCode::IndirectX(p) => vec![0x81, p],
                RawAddrCode::IndirectY(p) => vec![0x91, p],
                _ => panic!("Invalid addressing mode for STA"),
            },
            "STX" => match addr_mode {
                RawAddrCode::ZeroPage(p) => vec![0x86, p],
                RawAddrCode::ZeroPageY(p) => vec![0x96, p],
                RawAddrCode::Absolute(p) => vec![0x8E, (p & 0xFF) as u8, (p >> 8) as u8],
                _ => panic!("Invalid addressing mode for STX"),
            },
            "STY" => match addr_mode {
                RawAddrCode::ZeroPage(p) => vec![0x84, p],
                RawAddrCode::ZeroPageX(p) => vec![0x94, p],
                RawAddrCode::Absolute(p) => vec![0x8C, (p & 0xFF) as u8, (p >> 8) as u8],
                _ => panic!("Invalid addressing mode for STY"),
            },
            "TAX" => match addr_mode {
                RawAddrCode::Implicit => vec![0xAA],
                _ => panic!("Invalid addressing mode for TAX"),
            },
            "TAY" => match addr_mode {
                RawAddrCode::Implicit => vec![0xA8],
                _ => panic!("Invalid addressing mode for TAY"),
            },
            "TSX" => match addr_mode {
                RawAddrCode::Implicit => vec![0xBA],
                _ => panic!("Invalid addressing mode for TSX"),
            },
            "TXA" => match addr_mode {
                RawAddrCode::Implicit => vec![0x8A],
                _ => panic!("Invalid addressing mode for TXA"),
            },
            "TXS" => match addr_mode {
                RawAddrCode::Implicit => vec![0x9A],
                _ => panic!("Invalid addressing mode for TXS"),
            },
            "TYA" => match addr_mode {
                RawAddrCode::Implicit => vec![0x98],
                _ => panic!("Invalid addressing mode for TYA"),
            },
            _ => panic!("Invalid opcode: {}", opcode),
        };

        Some(code)
    }
}

/// A program structure
#[derive(Debug)]
pub struct Program {
    /// Start address
    pub start_addr: usize,
    /// Lines
    pub lines: Vec<u8>,
}

impl Program {
    /// Create a new program with gicen address
    /// ## Parameters
    /// * `start_addr` - Start address [`usize`]
    pub fn new(start_addr: usize) -> Self {
        Program {
            start_addr: start_addr,
            lines: Vec::new(),
        }
    }

    /// Add a line to the program
    /// ## Parameters
    /// * `instruction` - Instruction [`u8`]
    pub fn addl(&mut self, instruction: u8) -> &mut Self {
        self.lines.push(instruction);
        self
    }

    /// Fill the ram with the program
    /// ## Parameters
    /// * `ram` - Ram [`crate::mem::MEM`]
    pub fn fill_ram(&self, ram: &mut crate::mem::MEM) {
        for (i, line) in self.lines.clone().iter_mut().enumerate() {
            ram[self.start_addr + i] = *line;
        }
    }

    /// Parse the program from given string
    /// ## Parameters
    /// * `program` - Program [`str`]
    /// ## Example
    /// ```
    /// use rusty_6502::{asm, mem};
    /// let mut mem = mem::MEM::new();
    /// let program = asm::Program::new(600)
    /// .get_from_str("A2 01")
    /// .fill_ram(&mut mem);
    /// assert_eq!(mem[600], 0xA2);
    /// assert_eq!(mem[601], 0x01);
    /// ```
    pub fn get_from_str(&mut self, code: &str) -> &mut Self {
        let instructions = code.split(" ");
        for instruction in instructions {
            self.lines
                .push(u8::from_str_radix(instruction, 16).unwrap())
        }
        self
    }
}
