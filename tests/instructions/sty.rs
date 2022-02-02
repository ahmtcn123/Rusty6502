mod sty_tests {
    use rusty_6502::{cpu::CPU, mem::MEM};

    #[test]
    fn sta_zero_page() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0xA0;
        mem[601] = 0x01;
        mem[602] = 0x84;
        mem[603] = 0x00;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x00);
        assert_eq!(cpu.X, 0x00);
        assert_eq!(cpu.Y, 0x01);
        assert_eq!(mem[600], 0xA0);
        assert_eq!(mem[601], 0x01);
        assert_eq!(mem[602], 0x84);
        assert_eq!(mem[603], 0x00);
        assert_eq!(mem[0], 0x01);
    }

    #[test]
    fn sty_zero_page_x() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0xA2;
        mem[601] = 0x01;
        mem[602] = 0xA0;
        mem[603] = 0x01;
        mem[604] = 0x94;
        mem[605] = 0x00;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x00);
        assert_eq!(cpu.X, 0x01);
        assert_eq!(cpu.Y, 0x01);
        assert_eq!(mem[600], 0xA2);
        assert_eq!(mem[601], 0x01);
        assert_eq!(mem[602], 0xA0);
        assert_eq!(mem[603], 0x01);
        assert_eq!(mem[604], 0x94);
        assert_eq!(mem[605], 0x00);
        assert_eq!(mem[1], 0x01);
    }

    #[test]
    fn sty_absolute() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0xA0;
        mem[601] = 0x01;
        mem[602] = 0x8C;
        mem[603] = 0x01;
        mem[604] = 0x00;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x00);
        assert_eq!(cpu.X, 0x00);
        assert_eq!(cpu.Y, 0x01);
        assert_eq!(mem[600], 0xA0);
        assert_eq!(mem[601], 0x01);
        assert_eq!(mem[602], 0x8C);
        assert_eq!(mem[603], 0x01);
        assert_eq!(mem[604], 0x00);
        assert_eq!(mem[1], 0x01);
    }
}
