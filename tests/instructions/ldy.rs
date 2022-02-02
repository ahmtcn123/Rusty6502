mod ldy_tests {
    use rusty_6502::{cpu::CPU, mem::MEM};

    #[test]
    fn ldy_immediate() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0xA0;
        mem[601] = 0x01;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x00);
        assert_eq!(cpu.X, 0x00);
        assert_eq!(cpu.Y, 0x01);
        assert_eq!(mem[600], 0xA0);
        assert_eq!(mem[601], 0x01);
    }

    #[test]
    fn ldy_zero_page() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0xA9;
        mem[601] = 0x02;
        mem[602] = 0x85;
        mem[603] = 0x03;
        mem[604] = 0xA4;
        mem[605] = 0x03;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x02);
        assert_eq!(cpu.X, 0x00);
        assert_eq!(cpu.Y, 0x02);
        assert_eq!(mem[600], 0xA9);
        assert_eq!(mem[601], 0x02);
        assert_eq!(mem[602], 0x85);
        assert_eq!(mem[603], 0x03);
        assert_eq!(mem[604], 0xA4);
        assert_eq!(mem[605], 0x03);
    }

    #[test]
    fn ldy_zero_page_x() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0xA9;
        mem[601] = 0x02;
        mem[602] = 0x85;
        mem[603] = 0x03;
        mem[604] = 0xA2;
        mem[605] = 0x01;
        mem[606] = 0xB4;
        mem[607] = 0x02;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x02);
        assert_eq!(cpu.X, 0x01);
        assert_eq!(cpu.Y, 0x02);
        assert_eq!(mem[600], 0xA9);
        assert_eq!(mem[601], 0x02);
        assert_eq!(mem[602], 0x85);
        assert_eq!(mem[603], 0x03);
        assert_eq!(mem[604], 0xA2);
        assert_eq!(mem[605], 0x01);
        assert_eq!(mem[606], 0xB4);
        assert_eq!(mem[607], 0x02);
    }

    #[test]
    fn ldy_absolute() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0xA9;
        mem[601] = 0x02;
        mem[602] = 0x8D;
        mem[603] = 0x01;
        mem[604] = 0x03;
        mem[605] = 0xAC;
        mem[606] = 0x01;
        mem[607] = 0x03;
        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x02);
        assert_eq!(cpu.X, 0x00);
        assert_eq!(cpu.Y, 0x02);
        assert_eq!(mem[600], 0xA9);
        assert_eq!(mem[601], 0x02);
        assert_eq!(mem[602], 0x8D);
        assert_eq!(mem[603], 0x01);
        assert_eq!(mem[604], 0x03);
        assert_eq!(mem[605], 0xAC);
        assert_eq!(mem[606], 0x01);
        assert_eq!(mem[607], 0x03);
    }

    #[test]
    fn ldy_absolute_x() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0xA9;
        mem[601] = 0x03;
        mem[602] = 0xA2;
        mem[603] = 0x01;
        mem[604] = 0x9D;
        mem[605] = 0x01;
        mem[606] = 0x00;
        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x03);
        assert_eq!(cpu.X, 0x01);
        assert_eq!(cpu.Y, 0x00);
        assert_eq!(mem[600], 0xA9);
        assert_eq!(mem[601], 0x03);
        assert_eq!(mem[602], 0xA2);
        assert_eq!(mem[603], 0x01);
        assert_eq!(mem[604], 0x9D);
        assert_eq!(mem[605], 0x01);
        assert_eq!(mem[606], 0x00);
    }

    #[test]
    fn ldy_absolute_y() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0xA9;
        mem[601] = 0x02;
        mem[602] = 0x8D;
        mem[603] = 0x01;
        mem[604] = 0x03;
        mem[605] = 0xA2;
        mem[606] = 0x01;
        mem[607] = 0xBC;
        mem[608] = 0x00;
        mem[609] = 0x03;
        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x02);
        assert_eq!(cpu.X, 0x01);
        assert_eq!(cpu.Y, 0x02);
        assert_eq!(mem[600], 0xA9);
        assert_eq!(mem[601], 0x02);
        assert_eq!(mem[602], 0x8D);
        assert_eq!(mem[603], 0x01);
        assert_eq!(mem[604], 0x03);
        assert_eq!(mem[605], 0xA2);
        assert_eq!(mem[606], 0x01);
        assert_eq!(mem[607], 0xBC);
        assert_eq!(mem[608], 0x00);
        assert_eq!(mem[609], 0x03);
    }
}
