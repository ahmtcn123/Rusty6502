mod stx_tests {
    use rusty_6502::{cpu::CPU, mem::MEM};

    #[test]
    fn sta_zero_page() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0xA2;
        mem[601] = 0x01;
        mem[602] = 0x86;
        mem[603] = 0x00;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x00);
        assert_eq!(cpu.X, 0x01);
        assert_eq!(cpu.Y, 0x00);
        assert_eq!(mem[600], 0xA2);
        assert_eq!(mem[601], 0x01);
        assert_eq!(mem[602], 0x86);
        assert_eq!(mem[603], 0x00);
        assert_eq!(mem[0], 0x01);
    }

    #[test]
    fn stx_zero_page_y() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0xA2;
        mem[601] = 0x01;
        mem[602] = 0xA0;
        mem[603] = 0x01;
        mem[604] = 0x96;
        mem[605] = 0x00;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x00);
        assert_eq!(cpu.X, 0x01);
        assert_eq!(cpu.Y, 0x01);
        assert_eq!(mem[600], 0xA2);
        assert_eq!(mem[601], 0x01);
        assert_eq!(mem[602], 0xA0);
        assert_eq!(mem[603], 0x01);
        assert_eq!(mem[604], 0x96);
        assert_eq!(mem[605], 0x00);
        assert_eq!(mem[1], 0x01);
    }

    #[test]
    fn stx_absolute() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0xA2;
        mem[601] = 0x01;
        mem[602] = 0x8E;
        mem[603] = 0x01;
        mem[604] = 0x00;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x00);
        assert_eq!(cpu.X, 0x01);
        assert_eq!(cpu.Y, 0x00);
        assert_eq!(mem[600], 0xA2);
        assert_eq!(mem[601], 0x01);
        assert_eq!(mem[602], 0x8E);
        assert_eq!(mem[603], 0x01);
        assert_eq!(mem[604], 0x00);
        assert_eq!(mem[1], 0x01);
    }
}
