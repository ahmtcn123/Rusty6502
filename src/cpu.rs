#![allow(non_snake_case)]
use std::{fs::File, io::Write};

use crate::{
    asm::Instructions,
    debugger::{Debugger, MessageType},
    mem::MEM,
};

/// Status flags for the 6502
#[derive(Debug)]
pub struct StatusFlags {
    /// Negative flag
    pub N: u8,
    /// Overflow flag
    pub V: u8,
    #[allow(dead_code)]
    /// Unused flag
    pub U: u8,
    /// Interrupt flag
    pub I: u8,
    /// Break flag
    pub B: u8,
    /// Decimal mode flag
    pub D: u8,
    /// Zero flag
    pub Z: u8,
    /// Carry flag
    pub C: u8,
}

impl StatusFlags {
    /// Get processor status flags as a single byte
    pub fn get_ps(&self) -> u8 {
        self.N << 7
            | self.V << 6
            | self.I << 5
            | self.B << 4
            | self.D << 3
            | self.Z << 2
            | self.C << 1
    }
}

#[derive(PartialEq, Debug)]
/// Supervision mode for the CPU
pub enum Step {
    /// Disable Debugger
    Unsupervised,
    /// Enable Debugger
    Supervised,
}

#[allow(missing_debug_implementations)]
/// The 6502 CPU
pub struct CPU<E> {
    /// Program counter
    pub PC: u16,
    /// Status register
    pub SR: u8,
    /// Stack pointer
    pub SP: u8,

    /// Accumulator
    pub A: u8,
    /// X register
    pub X: u8,
    /// Y register
    pub Y: u8,

    /// Status flags
    pub status_flags: StatusFlags,
    /// Debugging option
    pub step: Step,
    /// Debugger bridge
    pub messenger: Debugger<E>,
}

impl<E> CPU<E>
where
    E: FnOnce(MessageType) + Copy + Sized,
{
    /// Create a new CPU
    /// ## Arguments
    /// * `debugger` - [`FnOnce(MessageType)`]
    /// ## Example
    /// ```
    /// use rusty_6502::cpu;
    ///
    /// //Create CPU
    /// let mut cpu = cpu::CPU::new(&|_| {
    ///    //Optional debugger see [cpu::debugger](./src/cpu.rs#L50)
    /// });
    /// // Enable callbacks
    /// cpu.step = cpu::Step::Supervised;
    /// ```
    pub fn new(messenger: E) -> Self {
        CPU {
            PC: 600,
            SR: 0xFF,
            SP: 0xFF,
            status_flags: StatusFlags {
                N: 0,
                V: 0,
                U: 1,
                I: 0,
                B: 0,
                D: 0,
                Z: 0,
                C: 0,
            },
            A: 0,
            X: 0,
            Y: 0,
            step: Step::Unsupervised,
            messenger: Debugger::new(messenger),
        }
    }

    fn emit_debugger(&mut self, message_type: MessageType) {
        if self.step == Step::Supervised {
            (self.messenger.messenger)(message_type);
        }
    }

    /// Reset the CPU
    /// ## Arguments
    /// * `start_pc` - The start point of the program [`u16`]
    /// * `mem` - The memory space [`MEM`]
    pub fn reset(&mut self, pc: u16, mem: &mut MEM) {
        self.PC = pc;
        self.A = 0;
        self.X = 0;
        self.Y = 0;
        self.SR = 0xff;
        self.SP = 0xff;

        self.status_flags.N = 0;
        self.status_flags.V = 0;
        self.status_flags.I = 0;
        self.status_flags.B = 0;
        self.status_flags.D = 0;
        self.status_flags.Z = 0;
        self.status_flags.C = 0;
        mem.initalize();
    }

    /// Fetch byte from program
    /// ## Arguments
    /// * `cycles` - Cycle to reduce [`u32`]
    /// * `mem` - The memory space [`MEM`]
    /// ## Returns
    /// The fetched byte [`u8`]
    pub fn fetch_byte(&mut self, cycles: &mut u32, mem: &mut MEM) -> u8 {
        let data: u8 = mem[self.PC.into()];
        self.PC += 1;
        *cycles -= 1;
        data
    }

    /// Write byte to memory
    /// ## Arguments
    /// * `cycles` - Cycle to reduce [`u32`]
    /// * `mem` - The memory space [`MEM`]
    /// * `address` - The address to write to [`u16`]
    /// * `value` - The data to write [`u8`]
    pub fn write_byte(&mut self, cycles: &mut u32, mem: &mut MEM, address: u16, value: u8) {
        mem[address.into()] = value;
        *cycles -= 1;
    }

    /// Write word to memory
    /// ## Arguments
    /// * `cycles` - Cycle to reduce [`u32`]
    /// * `mem` - The memory space [`MEM`]
    /// * `address` - The address to write to [`u16`]
    /// * `value` - The data to write [`u16`]
    pub fn write_word(&mut self, cycles: &mut u32, mem: &mut MEM, address: u8, value: u16) {
        mem[address.into()] = (value as u8) & 0xff;
        mem[(address + 1).into()] = (value >> 8) as u8;
        *cycles -= 2;
    }

    /// Read byte from memory
    /// ## Arguments
    /// * `cycles` - Cycle to reduce [`u32`]
    /// * `mem` - The memory space [`MEM`]
    /// * `address` - The address to read from [`u16`]
    /// ## Returns
    /// The read byte [`u8`]
    pub fn read_byte(&mut self, cycles: &mut u32, mem: &mut MEM, address: u16) -> u8 {
        let data: u8 = mem[address.into()];
        *cycles -= 1;
        data
    }

    /// Read word from memory
    /// ## Arguments
    /// * `cycles` - Cycle to reduce [`u32`]
    /// * `mem` - The memory space [`MEM`]
    /// * `address` - The address to read from [`u16`]
    /// ## Returns
    /// The read word [`u16`]
    pub fn read_word(&mut self, cycles: &mut u32, mem: &mut MEM, address: u16) -> u16 {
        let high = self.read_byte(cycles, mem, address);
        let low = self.read_byte(cycles, mem, (address) + 1);
        (high as u16) << 8 | (low as u16)
    }

    /// Fetch word from program
    /// ## Arguments
    /// * `cycles` - Cycle to reduce [`u32`]
    /// * `mem` - The memory space [`MEM`]
    /// ## Returns
    /// The fetched word [`u16`]
    pub fn fetch_word(&mut self, cycles: &mut u32, mem: &mut MEM) -> u16 {
        let mut data: u16 = self.fetch_byte(cycles, mem) as u16;
        data |= (self.fetch_byte(cycles, mem) as u16) << 8;
        data
    }

    /// Push word to stack
    /// ## Arguments
    /// * `cycles` - Cycle to reduce [`u32`]
    /// * `mem` - The memory space [`MEM`]
    /// * `value` - The data to push [`u16`]
    pub fn push_word_to_stack(&mut self, cycles: &mut u32, mem: &mut MEM, value: u16) {
        self.write_byte(
            cycles,
            mem,
            (0x100_u16 | self.SP as u16) as u16,
            (value >> 8) as u8,
        );
        self.SP -= 1;
        self.write_byte(
            cycles,
            mem,
            (0x100_u16 | self.SP as u16) as u16,
            (value & 0xff) as u8,
        );
        self.SP -= 1;
    }

    /// Push byte to stack
    /// ## Arguments
    /// * `cycles` - Cycle to reduce [`u32`]
    /// * `mem` - The memory space [`MEM`]
    /// * `value` - The data to push [`u8`]
    pub fn push_byte_to_stack(&mut self, cycles: &mut u32, mem: &mut MEM, value: u8) {
        self.write_byte(cycles, mem, (0x100 as u16 | self.SP as u16) as u16, value);
        self.SP -= 1;
    }

    /// Execute sized until cycles consumed
    /// ## Returns
    /// [`u16`] Ending location of the program counter
    pub fn execute_sized(&mut self, cycles: &mut u32, mem: &mut MEM) -> u16 {
        while cycles > &mut 0 {
            let (_, _, halted) = self.execute_instruction(&mut 9, mem);
            if halted {
                break;
            }
        }
        self.PC
    }

    /// Execute continuously until the CPU is halted.
    /// ## Returns
    /// (cycles: usize, ending_pc: u16)
    /// The number of cycles that were executed.
    /// Ending location of the program counter.
    pub fn execute_continuous(&mut self, mem: &mut MEM) -> (usize, u16) {
        let mut consumed_cycles: usize = 0;
        let mut last_pc = self.PC;
        loop {
            let (instruction, consumed, complete) = self.execute_instruction(&mut 9, mem);
            consumed_cycles += consumed as usize;
            self.emit_debugger(MessageType::LineExecuted(instruction, consumed));
            if complete {
                break;
            }
            last_pc = self.PC;
        }
        (consumed_cycles, last_pc)
    }

    fn execute_instruction(&mut self, cycles: &mut u32, mem: &mut MEM) -> (u8, u32, bool) {
        let old_cycles = cycles.clone();
        let instruction = self.fetch_byte(cycles, mem);
        let resolved_instruction = Instructions::resolve(instruction);
        let mut complete = false;
        match resolved_instruction {
            Instructions::BRK(address_mode) => match address_mode {
                crate::asm::AddrMode::Implied(_) => {
                    self.status_flags.I = 1;
                    self.push_word_to_stack(cycles, mem, self.PC + 2);
                    self.push_byte_to_stack(cycles, mem, self.SR);
                    //self.push_byte_to_stack(cycles, mem, self.status_flags.get_ps());
                    self.PC = self.read_word(cycles, mem, 0xFFFE);
                    self.status_flags.B = 1;
                    *cycles -= 1; //??
                    complete = true;
                }
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::BVC(address_mode) => match address_mode {
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::BVS(address_mode) => match address_mode {
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::CLC(address_mode) => match address_mode {
                crate::asm::AddrMode::Implied(_) => {
                    self.status_flags.C = 0;
                    *cycles -= 1;
                }
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::CLD(address_mode) => match address_mode {
                crate::asm::AddrMode::Implied(_) => {
                    self.status_flags.D = 0;
                    *cycles -= 1;
                }
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::CLI(address_mode) => match address_mode {
                crate::asm::AddrMode::Implied(_) => {
                    self.status_flags.I = 0;
                    *cycles -= 1;
                }
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::CLV(address_mode) => match address_mode {
                crate::asm::AddrMode::Implied(_) => {
                    self.status_flags.V = 0;
                    *cycles -= 1;
                }
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::CMP(address_mode) => match address_mode {
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::CPX(address_mode) => match address_mode {
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::CPY(address_mode) => match address_mode {
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::DEC(address_mode) => match address_mode {
                crate::asm::AddrMode::ZeroPage(_) => {
                    let address = self.fetch_byte(cycles, mem);
                    let value = self.read_byte(cycles, mem, address as u16);
                    self.write_byte(
                        cycles,
                        mem,
                        address.into(),
                        if value == 0 { 255 } else { value - 1 },
                    );
                    self.status_flags.Z = if value == 0 { 1 } else { 0 };
                    self.status_flags.N = if (value & 0b10000000) > 0 { 1 } else { 0 };
                    *cycles -= 1;
                }
                crate::asm::AddrMode::ZeroPageX(_) => {
                    let address = self.fetch_byte(cycles, mem) + self.X;
                    let value = self.read_byte(cycles, mem, address as u16);
                    self.write_byte(
                        cycles,
                        mem,
                        address.into(),
                        if value == 0 { 255 } else { value - 1 },
                    );
                    self.status_flags.Z = if value == 0 { 1 } else { 0 };
                    self.status_flags.N = if (value & 0b10000000) > 0 { 1 } else { 0 };
                    *cycles -= 2;
                }
                crate::asm::AddrMode::Absolute(_) => {
                    let address = self.fetch_word(cycles, mem);
                    let value = self.read_byte(cycles, mem, address as u16);
                    self.write_byte(
                        cycles,
                        mem,
                        address.into(),
                        if value == 0 { 255 } else { value - 1 },
                    );
                    self.status_flags.Z = if value == 0 { 1 } else { 0 };
                    self.status_flags.N = if (value & 0b10000000) > 0 { 1 } else { 0 };
                    *cycles -= 2;
                }
                crate::asm::AddrMode::AbsoluteX(_) => {
                    let mut address = self.fetch_word(cycles, mem);
                    address += self.X as u16;
                    let value = self.read_byte(cycles, mem, address as u16);
                    self.write_byte(
                        cycles,
                        mem,
                        address.into(),
                        if value == 0 { 255 } else { value - 1 },
                    );
                    self.status_flags.Z = if value == 0 { 1 } else { 0 };
                    self.status_flags.N = if (value & 0b10000000) > 0 { 1 } else { 0 };
                    *cycles -= 2;
                }
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::DEX(address_mode) => match address_mode {
                crate::asm::AddrMode::Implied(_) => {
                    self.X = if self.X == 0 { 255 } else { self.X - 1 };
                    self.status_flags.Z = if self.X == 0 { 1 } else { 0 };
                    self.status_flags.N = if (self.X & 0b10000000) > 0 { 1 } else { 0 };
                    *cycles -= 1;
                }
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::DEY(address_mode) => match address_mode {
                crate::asm::AddrMode::Implied(_) => {
                    self.Y = if self.Y == 0 { 255 } else { self.Y - 1 };
                    self.status_flags.Z = if self.Y == 0 { 1 } else { 0 };
                    self.status_flags.N = if (self.Y & 0b10000000) > 0 { 1 } else { 0 };
                    *cycles -= 1;
                }
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::EOR(address_mode) => match address_mode {
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::INC(address_mode) => match address_mode {
                crate::asm::AddrMode::ZeroPage(_) => {
                    let address = self.fetch_byte(cycles, mem);
                    let value = self.read_byte(cycles, mem, address as u16);
                    self.write_byte(
                        cycles,
                        mem,
                        address.into(),
                        if value == 255 { 0 } else { value + 1 },
                    );
                    self.status_flags.Z = if value == 0 { 1 } else { 0 };
                    self.status_flags.N = if (value & 0b10000000) > 0 { 1 } else { 0 };
                    *cycles -= 1;
                }
                crate::asm::AddrMode::ZeroPageX(_) => {
                    let address = self.fetch_byte(cycles, mem) + self.X;
                    let value = self.read_byte(cycles, mem, address as u16);
                    self.write_byte(
                        cycles,
                        mem,
                        address.into(),
                        if value == 255 { 0 } else { value + 1 },
                    );
                    self.status_flags.Z = if value == 0 { 1 } else { 0 };
                    self.status_flags.N = if (value & 0b10000000) > 0 { 1 } else { 0 };
                    *cycles -= 2;
                }
                crate::asm::AddrMode::Absolute(_) => {
                    let address = self.fetch_word(cycles, mem);
                    let value = self.read_byte(cycles, mem, address as u16);
                    self.write_byte(
                        cycles,
                        mem,
                        address.into(),
                        if value == 255 { 0 } else { value + 1 },
                    );
                    self.status_flags.Z = if value == 0 { 1 } else { 0 };
                    self.status_flags.N = if (value & 0b10000000) > 0 { 1 } else { 0 };
                    *cycles -= 2;
                }
                crate::asm::AddrMode::AbsoluteX(_) => {
                    let mut address = self.fetch_word(cycles, mem);
                    address += self.X as u16;
                    let value = self.read_byte(cycles, mem, address as u16);
                    self.write_byte(
                        cycles,
                        mem,
                        address.into(),
                        if value == 255 { 0 } else { value + 1 },
                    );
                    self.status_flags.Z = if value == 0 { 1 } else { 0 };
                    self.status_flags.N = if (value & 0b10000000) > 0 { 1 } else { 0 };
                    *cycles -= 2;
                }
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::INX(address_mode) => match address_mode {
                crate::asm::AddrMode::Implied(_) => {
                    self.X = if self.X == 255 { 0 } else { self.X + 1 };
                    self.status_flags.Z = if self.X == 0 { 1 } else { 0 };
                    self.status_flags.N = if (self.X & 0b10000000) > 0 { 1 } else { 0 };
                    *cycles -= 1;
                }
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::INY(address_mode) => match address_mode {
                crate::asm::AddrMode::Implied(_) => {
                    self.Y = if self.Y == 255 { 0 } else { self.Y + 1 };
                    self.status_flags.Z = if self.Y == 0 { 1 } else { 0 };
                    self.status_flags.N = if (self.Y & 0b10000000) > 0 { 1 } else { 0 };
                    *cycles -= 1;
                }
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::JMP(address_mode) => match address_mode {
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::JSR(address_mode) => match address_mode {
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::LDX(address_mode) => {
                match address_mode {
                    crate::asm::AddrMode::Immediate(_) => {
                        let data = self.fetch_byte(cycles, mem);
                        self.X = data;
                    }
                    crate::asm::AddrMode::ZeroPage(_) => {
                        //Acquire the address
                        let memory_location = self.fetch_byte(cycles, mem);
                        //Read given address
                        self.X = self.read_byte(cycles, mem, memory_location.into());
                    }
                    crate::asm::AddrMode::ZeroPageY(_) => {
                        //Acquire the address
                        let mut memory_location = self.fetch_byte(cycles, mem);
                        memory_location += self.Y;
                        //Read given address
                        self.X = self.read_byte(cycles, mem, memory_location.into());
                        *cycles -= 1;
                    }
                    crate::asm::AddrMode::Absolute(_) => {
                        let address = self.fetch_word(cycles, mem);
                        self.X = self.read_byte(cycles, mem, address);
                    }
                    crate::asm::AddrMode::AbsoluteY(_) => {
                        let mut address = self.fetch_word(cycles, mem);
                        address += self.Y as u16;
                        self.X = self.read_byte(cycles, mem, address);
                    }
                    _ => panic!("Wrong addressing mode"),
                }
                //Set LDX status
                self.status_flags.Z = if self.X == 0 { 1 } else { 0 };
                self.status_flags.N = if (self.X & 0b10000000) > 0 { 1 } else { 0 };
            }
            Instructions::LDA(address_mode) => {
                match address_mode {
                    crate::asm::AddrMode::Immediate(_) => {
                        let data = self.fetch_byte(cycles, mem);
                        self.A = data;
                    }
                    crate::asm::AddrMode::ZeroPage(_) => {
                        //Acquire the address
                        let memory_location = self.fetch_byte(cycles, mem);
                        //Read given address
                        self.A = self.read_byte(cycles, mem, memory_location.into());
                    }
                    crate::asm::AddrMode::ZeroPageX(_) => {
                        //Acquire first parameter
                        let first_param = self.fetch_byte(cycles, mem);
                        //Read given address
                        self.A = self.read_byte(cycles, mem, (first_param + self.X).into());
                        *cycles -= 1;
                    }
                    crate::asm::AddrMode::Absolute(_) => {
                        let address = self.fetch_word(cycles, mem);
                        self.A = self.read_byte(cycles, mem, address);
                    }
                    crate::asm::AddrMode::AbsoluteX(_) => {
                        let mut address = self.fetch_word(cycles, mem);
                        address += self.X as u16;
                        self.A = self.read_byte(cycles, mem, address);
                    }
                    crate::asm::AddrMode::AbsoluteY(_) => {
                        let mut address = self.fetch_word(cycles, mem);
                        address += self.Y as u16;
                        self.A = self.read_byte(cycles, mem, address);
                    }
                    crate::asm::AddrMode::IndirectX(_) => {
                        let mut address = self.fetch_word(cycles, mem);
                        address += self.X as u16;
                        let low = self.read_byte(cycles, mem, address);
                        let high = self.read_byte(cycles, mem, address + 1);
                        let new_address = (high as u16) << 8 | (low as u16);
                        self.A = self.read_byte(cycles, mem, new_address);
                    }
                    crate::asm::AddrMode::IndirectY(_) => {
                        let address = self.fetch_word(cycles, mem);
                        let low = self.read_byte(cycles, mem, address);
                        let high = self.read_byte(cycles, mem, address + 1);
                        let new_address = (high as u16) << 8 | (low as u16);
                        self.A = self.read_byte(cycles, mem, new_address + (self.Y as u16));
                    }
                    _ => panic!("Wrong addressing mode"),
                }
                //Set lda status
                self.status_flags.Z = if self.A == 0 { 1 } else { 0 };
                self.status_flags.N = if (self.A & 0b10000000) > 0 { 1 } else { 0 };
            }
            Instructions::LDY(address_mode) => {
                match address_mode {
                    crate::asm::AddrMode::Immediate(_) => {
                        let data = self.fetch_byte(cycles, mem);
                        self.Y = data;
                    }
                    crate::asm::AddrMode::ZeroPage(_) => {
                        //Acquire the address
                        let memory_location = self.fetch_byte(cycles, mem);
                        //Read given address
                        self.Y = self.read_byte(cycles, mem, memory_location.into());
                    }
                    crate::asm::AddrMode::ZeroPageX(_) => {
                        //Acquire the address
                        let mut memory_location = self.fetch_byte(cycles, mem);
                        memory_location += self.X;
                        //Read given address
                        self.Y = self.read_byte(cycles, mem, memory_location.into());
                    }
                    crate::asm::AddrMode::Absolute(_) => {
                        let address = self.fetch_word(cycles, mem);
                        self.Y = self.read_byte(cycles, mem, address);
                    }
                    crate::asm::AddrMode::AbsoluteX(_) => {
                        let mut address = self.fetch_word(cycles, mem);
                        address += self.X as u16;
                        self.Y = self.read_byte(cycles, mem, address);
                    }
                    _ => panic!("Wrong addressing mode"),
                }
                //Set ldy status
                self.status_flags.Z = if self.Y == 0 { 1 } else { 0 };
                self.status_flags.N = if (self.Y & 0b10000000) > 0 { 1 } else { 0 };
            }
            Instructions::LSR(address_mode) => match address_mode {
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::NOP(address_mode) => match address_mode {
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::ORA(address_mode) => match address_mode {
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::PHA(address_mode) => match address_mode {
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::PHP(address_mode) => match address_mode {
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::PLA(address_mode) => match address_mode {
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::PLP(address_mode) => match address_mode {
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::ROL(address_mode) => match address_mode {
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::ROR(address_mode) => match address_mode {
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::RTI(address_mode) => match address_mode {
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::RTS(address_mode) => match address_mode {
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::SBC(address_mode) => match address_mode {
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::SEC(address_mode) => match address_mode {
                crate::asm::AddrMode::Implied(_) => {
                    self.status_flags.C = 1;
                    *cycles -= 1;
                }
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::SED(address_mode) => match address_mode {
                crate::asm::AddrMode::Implied(_) => {
                    self.status_flags.D = 1;
                    *cycles -= 1;
                }
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::SEI(address_mode) => match address_mode {
                crate::asm::AddrMode::Implied(_) => {
                    self.status_flags.I = 1;
                    *cycles -= 1;
                }
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::STA(address_mode) => match address_mode {
                crate::asm::AddrMode::ZeroPage(_) => {
                    let address = self.fetch_byte(cycles, mem);
                    self.write_byte(cycles, mem, address as u16, self.A);
                }
                crate::asm::AddrMode::ZeroPageX(_) => {
                    let mut address = self.fetch_byte(cycles, mem);
                    address += self.X;
                    self.write_byte(cycles, mem, address as u16, self.A);
                }
                crate::asm::AddrMode::Absolute(_) => {
                    let address = self.fetch_word(cycles, mem);
                    self.write_byte(cycles, mem, address, self.A)
                }
                crate::asm::AddrMode::AbsoluteX(_) => {
                    let mut address = self.fetch_word(cycles, mem);
                    address += self.X as u16;
                    self.write_byte(cycles, mem, address, self.A);
                }
                crate::asm::AddrMode::AbsoluteY(_) => {
                    let mut address = self.fetch_word(cycles, mem);
                    address += self.Y as u16;
                    self.write_byte(cycles, mem, address, self.A);
                }
                crate::asm::AddrMode::IndirectX(_) => {
                    let mut address = self.fetch_word(cycles, mem);
                    address += self.X as u16;
                    let low = self.read_byte(cycles, mem, address);
                    let high = self.read_byte(cycles, mem, address + 1);
                    let new_address = (high as u16) << 8 | (low as u16);
                    self.write_byte(cycles, mem, new_address, self.A);
                }
                crate::asm::AddrMode::IndirectY(_) => {
                    let address = self.fetch_word(cycles, mem);
                    let low = self.read_byte(cycles, mem, address);
                    let high = self.read_byte(cycles, mem, address + 1);
                    let new_address = (high as u16) << 8 | (low as u16);
                    self.write_byte(cycles, mem, new_address + (self.Y as u16), self.A);
                }
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::STX(address_mode) => match address_mode {
                crate::asm::AddrMode::ZeroPage(_) => {
                    let address = self.fetch_byte(cycles, mem);
                    self.write_byte(cycles, mem, address as u16, self.X);
                }
                crate::asm::AddrMode::ZeroPageY(_) => {
                    let mut address = self.fetch_byte(cycles, mem);
                    address += self.Y;
                    self.write_byte(cycles, mem, address as u16, self.X);
                    *cycles -= 1;
                }
                crate::asm::AddrMode::Absolute(_) => {
                    let address = self.fetch_word(cycles, mem);
                    self.write_byte(cycles, mem, address as u16, self.X);
                }
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::STY(address_mode) => match address_mode {
                crate::asm::AddrMode::ZeroPage(_) => {
                    let address = self.fetch_byte(cycles, mem);
                    self.write_byte(cycles, mem, address as u16, self.Y);
                }
                crate::asm::AddrMode::ZeroPageX(_) => {
                    let mut address = self.fetch_byte(cycles, mem);
                    address += self.X;
                    self.write_byte(cycles, mem, address as u16, self.Y);
                }
                crate::asm::AddrMode::Absolute(_) => {
                    let address = self.fetch_word(cycles, mem);
                    self.write_byte(cycles, mem, address as u16, self.Y);
                }
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::TAX(address_mode) => match address_mode {
                crate::asm::AddrMode::Implied(_) => {
                    self.X = self.A;
                    self.status_flags.Z = if self.X == 0 { 1 } else { 0 };
                    self.status_flags.N = if (self.X & 0b10000000) > 0 { 1 } else { 0 };
                    *cycles -= 1;
                }
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::TAY(address_mode) => match address_mode {
                crate::asm::AddrMode::Implied(_) => {
                    self.Y = self.A;
                    self.status_flags.Z = if self.Y == 0 { 1 } else { 0 };
                    self.status_flags.N = if (self.Y & 0b10000000) > 0 { 1 } else { 0 };
                    *cycles -= 1;
                }
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::TSX(address_mode) => match address_mode {
                crate::asm::AddrMode::Implied(_) => {
                    self.X = self.SP;
                    self.status_flags.Z = if self.X == 0 { 1 } else { 0 };
                    self.status_flags.N = if (self.X & 0b10000000) > 0 { 1 } else { 0 };
                    *cycles -= 1;
                }
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::TXA(address_mode) => match address_mode {
                crate::asm::AddrMode::Implied(_) => {
                    self.A = self.X;
                    self.status_flags.Z = if self.A == 0 { 1 } else { 0 };
                    self.status_flags.N = if (self.A & 0b10000000) > 0 { 1 } else { 0 };
                    *cycles -= 1;
                }
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::TXS(address_mode) => match address_mode {
                crate::asm::AddrMode::Implied(_) => {
                    self.SP = self.X;
                    *cycles -= 1;
                }
                _ => panic!("Wrong addressing mode"),
            },
            Instructions::TYA(address_mode) => match address_mode {
                crate::asm::AddrMode::Implied(_) => {
                    self.A = self.Y;
                    self.status_flags.Z = if self.A == 0 { 1 } else { 0 };
                    self.status_flags.N = if (self.A & 0b10000000) > 0 { 1 } else { 0 };
                    *cycles -= 1;
                }
                _ => panic!("Wrong addressing mode"),
            },
            _ => {
                File::create("mem.dump")
                    .unwrap()
                    .write_all(&mem.data)
                    .unwrap();
                unimplemented!(
                    "Unimplemented instruction at {:02x}; {:02X} : {}",
                    cycles,
                    instruction,
                    resolved_instruction
                )
            }
        }
        (instruction, old_cycles - *cycles, complete)
    }
}
