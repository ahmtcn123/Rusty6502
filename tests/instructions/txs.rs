mod txs_tests {
    use rusty_6502::{cpu::CPU, mem::MEM};

    #[test]
    fn txs_implied() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0xA2;
        mem[601] = 0x03;
        mem[602] = 0x9A;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x00);
        assert_eq!(cpu.X, 0x03);
        assert_eq!(cpu.Y, 0x00);
        assert_eq!(mem[600], 0xA2);
        assert_eq!(mem[601], 0x03);
        assert_eq!(mem[602], 0x9A);
    }
}
