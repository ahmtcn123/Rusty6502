# Rusty6502
[![Crates.io Version](https://img.shields.io/crates/v/rusty_6502?logo=rust)](https://crates.io/crates/rusty_6502)
[![Documentation](https://docs.rs/rusty_6502/badge.svg)](https://docs.rs/rusty_6502)
A 6502 emulator written in Rust.

Created for understanding the 6502 architecture and assembly. You can see the progress [here](./todo.md), when all the instructions are implemented more examples will show up. My goal is implement all the instructions and create a hybrid platform that can work with too many different platforms. 6502 assembly is easy and fun to learn, planed hybrid platform can help other developers to learn how cpu works.

### Virtual console

This repo has a virtual console that can be used to test the emulator. You can find it [here](./src/bin/romodore.md).


## Example

```rust
//Create memory space
let mut mem = mem::MEM::new();

//Create CPU
let mut cpu = cpu::CPU::new(&|_| {
    //Optional debugger see [cpu::debugger](./src/cpu.rs#L50)
});

//Reset the CPU, with start point at 0x600
cpu.reset(600, &mut mem);

/*
    LDA #$01; Load accumulator with 01
*/
mem[600] = 0xA9;
mem[601] = 0x01;

let (cycles, ending_pc) = cpu.execute_continuous(&mut mem);
//Print A, X, and Y registers
println!("\nA: {:02x} X: {:02x} Y: {:02x}", cpu.A, cpu.X, cpu.Y);

//Print status flags
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

//Print program counter and stack pointer
println!("PC: {} SP: {:02x}", cpu.PC, cpu.SP);
//Print processor status
println!("PS: {:02x}", cpu.status_flags.get_ps());

//Dump the memory for inspection, hex viewer is helpfull
File::create("mem.dump").unwrap().write_all(&mem.data).unwrap();
```

## License
[GPL-2.0 License](./LICENSE)