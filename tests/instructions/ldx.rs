mod ldy_tests {
    use rusty_6502::{cpu::CPU, mem::MEM};

    #[test]
    fn ldx_immediate() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0xA2;
        mem[601] = 0x01;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x00);
        assert_eq!(cpu.X, 0x01);
        assert_eq!(cpu.Y, 0x00);
        assert_eq!(mem[600], 0xA2);
        assert_eq!(mem[601], 0x01);
    }

    #[test]
    fn ldx_zero_page() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0xA9;
        mem[601] = 0x02;
        mem[602] = 0x85;
        mem[603] = 0x02;
        mem[604] = 0xA6;
        mem[605] = 0x02;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x02);
        assert_eq!(cpu.X, 0x02);
        assert_eq!(cpu.Y, 0x00);
        assert_eq!(mem[600], 0xA9);
        assert_eq!(mem[601], 0x02);
        assert_eq!(mem[602], 0x85);
        assert_eq!(mem[603], 0x02);
        assert_eq!(mem[604], 0xA6);
        assert_eq!(mem[605], 0x02);
    }

    #[test]
    fn ldx_zero_page_y() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0xA9;
        mem[601] = 0x02;
        mem[602] = 0x85;
        mem[603] = 0x03;
        mem[604] = 0xA0;
        mem[605] = 0x01;
        mem[606] = 0xB6;
        mem[607] = 0x02;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x02);
        assert_eq!(cpu.X, 0x02);
        assert_eq!(cpu.Y, 0x01);
        assert_eq!(mem[600], 0xA9);
        assert_eq!(mem[601], 0x02);
        assert_eq!(mem[602], 0x85);
        assert_eq!(mem[603], 0x03);
        assert_eq!(mem[604], 0xA0);
        assert_eq!(mem[605], 0x01);
        assert_eq!(mem[606], 0xB6);
        assert_eq!(mem[607], 0x02);
    }

    #[test]
    fn ldx_absolute() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0xA9;
        mem[601] = 0x02;
        mem[602] = 0x8D;
        mem[603] = 0x02;
        mem[604] = 0x03;
        mem[605] = 0xAE;
        mem[606] = 0x02;
        mem[607] = 0x03;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x02);
        assert_eq!(cpu.X, 0x02);
        assert_eq!(cpu.Y, 0x00);
        assert_eq!(mem[600], 0xA9);
        assert_eq!(mem[601], 0x02);
        assert_eq!(mem[602], 0x8D);
        assert_eq!(mem[603], 0x02);
        assert_eq!(mem[604], 0x03);
        assert_eq!(mem[605], 0xAE);
        assert_eq!(mem[606], 0x02);
        assert_eq!(mem[607], 0x03);
    }

    #[test]
    fn ldx_absolute_y() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0xA9;
        mem[601] = 0x02;
        mem[602] = 0x8D;
        mem[603] = 0x03;
        mem[604] = 0x03;
        mem[605] = 0xA0;
        mem[606] = 0x01;
        mem[607] = 0xBE;
        mem[608] = 0x02;
        mem[609] = 0x03;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x02);
        assert_eq!(cpu.X, 0x02);
        assert_eq!(cpu.Y, 0x01);
        assert_eq!(mem[600], 0xA9);
        assert_eq!(mem[601], 0x02);
        assert_eq!(mem[602], 0x8D);
        assert_eq!(mem[603], 0x03);
        assert_eq!(mem[604], 0x03);
        assert_eq!(mem[605], 0xA0);
        assert_eq!(mem[606], 0x01);
        assert_eq!(mem[607], 0xBE);
        assert_eq!(mem[608], 0x02);
        assert_eq!(mem[609], 0x03);
    }
}
