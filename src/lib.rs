#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(test, deny(warnings))]
#![doc(html_root_url = "https://docs.rs/rusty_6502/0.3.2-alpha")]

//! # rusty_6502
//! A 6502 emulator written in Rust.
//! ## Example
//! ```
//! use std::{fs::File, io::Write};
//! use rusty_6502::{cpu, mem};
//! //Create memory space
//! let mut mem = mem::MEM::new();
//! 
//! //Create CPU
//! let mut cpu = cpu::CPU::new(&|_| {
//!     //Optional debugger see [cpu::debugger](./src/cpu.rs#L50)
//! });
//! 
//! //Reset the CPU, with start point at 0x600
//! cpu.reset(600, &mut mem);
//! 
//! /*
//!     LDA #$01; Load accumulator with 01
//! */
//! mem[600] = 0xA9;
//! mem[601] = 0x01;
//! 
//! let (cycles, ending_pc) = cpu.execute_continuous(&mut mem);
//! //Print A, X, and Y registers
//! println!("\nA: {:02x} X: {:02x} Y: {:02x}", cpu.A, cpu.X, cpu.Y);
//! 
//! //Print status flags
//! println!(
//!         "| NV-BDIZC |\n| {}{}{}{}{}{}{}{} |",
//!         cpu.status_flags.N,
//!         cpu.status_flags.V,
//!         cpu.status_flags.U,
//!         cpu.status_flags.B,
//!         cpu.status_flags.D,
//!         cpu.status_flags.I,
//!         cpu.status_flags.Z,
//!         cpu.status_flags.C
//!     );
//! 
//! //Print program counter and stack pointer
//! println!("PC: {} SP: {:02x}", cpu.PC, cpu.SP);
//! //Print processor status
//! println!("PS: {:02x}", cpu.status_flags.get_ps());
//! 
//! //Dump the memory for inspection, hex viewer is helpfull
//! File::create("mem.dump").unwrap().write_all(&mem.data).unwrap();
//! ```
//! 
//! ## License
//! [GPL-2.0 License](./LICENSE)

///CPU
pub mod cpu;
///Memory
pub mod mem;
#[macro_use]
///Instructions and utils
pub mod asm;
///Debugger
pub mod debugger;
