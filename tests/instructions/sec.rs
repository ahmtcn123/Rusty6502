mod sec_tests {
    use rusty_6502::{cpu::CPU, mem::MEM};

    #[test]
    fn sec_immediate() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0x38;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x00);
        assert_eq!(cpu.X, 0x00);
        assert_eq!(cpu.Y, 0x00);
        assert_eq!(cpu.status_flags.C, 1);
    }
}