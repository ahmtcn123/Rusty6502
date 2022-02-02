mod tya_tests {
    use rusty_6502::{cpu::CPU, mem::MEM};

    #[test]
    fn tya_implied() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0xA0;
        mem[601] = 0x01;
        mem[602] = 0x98;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x01);
        assert_eq!(cpu.X, 0x00);
        assert_eq!(cpu.Y, 0x01);
        assert_eq!(mem[600], 0xA0);
        assert_eq!(mem[601], 0x01);
        assert_eq!(mem[602], 0x98);
    }
}
