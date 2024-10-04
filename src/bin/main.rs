use rusty_6502::{
    asm::Program,
    cpu::{self, Step},
    debugger, mem,
    parser::parse_assembly_string,
};
use std::{
    fs::{self, File},
    io::{Read, Write},
};

fn main() {
    parse_assembly_string(
        r#"
    .org 600
    LDX #$00            
    LDA #$00            
loop_start:
    INX                 
    STA $01             
    CPX #$09           
    BNE loop_start      
    BRK                 

    "#,
    )
    .unwrap();

    panic!();
    let mut mem = mem::MEM::new();
    let mut cpu = cpu::CPU::new(&|e| match e {
        debugger::MessageType::LineExecuted(ins, cocyl) => {
            println!(
                "| INS: {:02x} ({:?}), Consumed cycles: {}",
                ins,
                rusty_6502::asm::Instructions::resolve(ins),
                cocyl
            );
        }
        debugger::MessageType::Waiting => todo!(),
        debugger::MessageType::Overflow => todo!(),
        debugger::MessageType::BreakpointHit => todo!(),
    });
    cpu.step = Step::Supervised;

    cpu.reset(600, &mut mem);

    /*  Program::new(600)
        .parse_assembly_string(
            r#"
    LDX #$00
    LDA #$09
    STA $01
    BRK
        "#,
        )
        .fill_ram(&mut mem); */

    let (cycles, end) = cpu.execute_continuous(&mut mem);
    println!("\nA: {:02x} X: {:02x} Y: {:02x}", cpu.A, cpu.X, cpu.Y);
    println!(
        "| NV-BDIZC |\n| {}{}{}{}{}{}{}{} |",
        cpu.status_flags.N,
        cpu.status_flags.V,
        cpu.status_flags.U,
        cpu.status_flags.B,
        cpu.status_flags.D,
        cpu.status_flags.I,
        cpu.status_flags.Z,
        cpu.status_flags.C
    );
    //println!("NMI: {:02x}   BRK: {:02x} IRQ: {:02x}", cpu.status_flags.N, cpu.status_flags.B, cpu.IRQ);
    println!("PC: {} SP: {:02x}", cpu.PC, cpu.SP);
    println!("PS: {:02x}", cpu.status_flags.get_ps());
    println!("Program started at PC: {}", 600);
    println!("Program ended at: {}", end);
    println!("Program took cycles: {}", cycles);
    println!(
        "Program: {}: {}",
        600,
        mem.hex_dump(600, 600 + (cycles) as usize)
    );
    println!("Memory dumped to mem.dump");
    File::create("mem.dump")
        .unwrap()
        .write_all(&mem.data)
        .unwrap();
}

//let mut mem_viewer = mem_viewer::MemViewer::create_display(mem.data.len());
//mem_viewer.update(mem);
