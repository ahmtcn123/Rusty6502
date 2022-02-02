mod inc_tests {
    use rusty_6502::{cpu::CPU, mem::MEM};

    #[test]
    fn inc_zero_page() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0xA0;
        mem[601] = 0x03;
        mem[602] = 0x84;
        mem[603] = 0x02;
        mem[604] = 0xE6;
        mem[605] = 0x02;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x00);
        assert_eq!(cpu.X, 0x00);
        assert_eq!(cpu.Y, 0x03);
        assert_eq!(mem[600], 0xA0);
        assert_eq!(mem[601], 0x03);
        assert_eq!(mem[602], 0x84);
        assert_eq!(mem[603], 0x02);
        assert_eq!(mem[604], 0xE6);
        assert_eq!(mem[605], 0x02);
        assert_eq!(mem[2], 0x04);
    }

    #[test]
    fn inc_zero_page_x() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0xA2;
        mem[601] = 0x01;
        mem[602] = 0xA0;
        mem[603] = 0x03;
        mem[604] = 0x84;
        mem[605] = 0x02;
        mem[606] = 0xF6;
        mem[607] = 0x01;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x00);
        assert_eq!(cpu.X, 0x01);
        assert_eq!(cpu.Y, 0x03);
        assert_eq!(mem[600], 0xA2);
        assert_eq!(mem[601], 0x01);
        assert_eq!(mem[602], 0xA0);
        assert_eq!(mem[603], 0x03);
        assert_eq!(mem[604], 0x84);
        assert_eq!(mem[605], 0x02);
        assert_eq!(mem[606], 0xF6);
        assert_eq!(mem[607], 0x01);
        assert_eq!(mem[2], 0x04);
    }

    #[test]
    fn inc_absolute() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0xA0;
        mem[601] = 0x03;
        mem[602] = 0x8C;
        mem[603] = 0x02;
        mem[604] = 0x02;
        mem[605] = 0xEE;
        mem[606] = 0x02;
        mem[607] = 0x02;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x00);
        assert_eq!(cpu.X, 0x00);
        assert_eq!(cpu.Y, 0x03);
        assert_eq!(mem[600], 0xA0);
        assert_eq!(mem[601], 0x03);
        assert_eq!(mem[602], 0x8C);
        assert_eq!(mem[603], 0x02);
        assert_eq!(mem[604], 0x02);
        assert_eq!(mem[605], 0xEE);
        assert_eq!(mem[606], 0x02);
        assert_eq!(mem[607], 0x02);
        assert_eq!(mem[0x0202], 0x04);
    }

    #[test]
    fn inc_absolute_x() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0xA2;
        mem[601] = 0x01;
        mem[602] = 0xA0;
        mem[603] = 0x03;
        mem[604] = 0x8C;
        mem[605] = 0x02;
        mem[606] = 0x02;
        mem[607] = 0xFE;
        mem[608] = 0x01;
        mem[609] = 0x02;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x00);
        assert_eq!(cpu.X, 0x01);
        assert_eq!(cpu.Y, 0x03);
        assert_eq!(mem[600], 0xA2);
        assert_eq!(mem[601], 0x01);
        assert_eq!(mem[602], 0xA0);
        assert_eq!(mem[603], 0x03);
        assert_eq!(mem[604], 0x8C);
        assert_eq!(mem[605], 0x02);
        assert_eq!(mem[606], 0x02);
        assert_eq!(mem[607], 0xFE);
        assert_eq!(mem[608], 0x01);
        assert_eq!(mem[609], 0x02);
        assert_eq!(mem[0x0202], 0x04);
    }
}
