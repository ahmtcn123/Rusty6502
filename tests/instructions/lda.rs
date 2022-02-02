mod lda_tests {
    use rusty_6502::{cpu::CPU, mem::MEM};

    #[test]
    fn lda_immediate() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0xA9;
        mem[601] = 0x01;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x01);
        assert_eq!(cpu.X, 0x00);
        assert_eq!(cpu.Y, 0x00);
        assert_eq!(mem[600], 0xA9);
        assert_eq!(mem[601], 0x01);
    }

    #[test]
    fn lda_zero_page() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);

        mem[600] = 0xA2;
        mem[601] = 0x01;
        mem[602] = 0x86;
        mem[603] = 0x00;
        mem[604] = 0xA5;
        mem[605] = 0x00;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x01);
        assert_eq!(cpu.X, 0x01);
        assert_eq!(cpu.Y, 0x00);
        assert_eq!(mem[600], 0xA2);
        assert_eq!(mem[601], 0x01);
        assert_eq!(mem[602], 0x86);
        assert_eq!(mem[603], 0x00);
        assert_eq!(mem[604], 0xA5);
        assert_eq!(mem[605], 0x00);
    }

    #[test]
    fn lda_zero_page_x() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);

        mem[600] = 0xA2;
        mem[601] = 0x01;
        mem[602] = 0x86;
        mem[603] = 0x00;
        mem[604] = 0xA5;
        mem[605] = 0x00;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x01);
        assert_eq!(cpu.X, 0x01);
        assert_eq!(cpu.Y, 0x00);
        assert_eq!(mem[600], 0xA2);
        assert_eq!(mem[601], 0x01);
        assert_eq!(mem[602], 0x86);
        assert_eq!(mem[603], 0x00);
        assert_eq!(mem[604], 0xA5);
        assert_eq!(mem[605], 0x00);
    }

    #[test]
    fn lda_absolute() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);

        mem[600] = 0xA2;
        mem[601] = 0x01;
        mem[602] = 0x8E;
        mem[603] = 0x00;
        mem[604] = 0x00;
        mem[605] = 0xAD;
        mem[606] = 0x00;
        mem[607] = 0x00;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x01);
        assert_eq!(cpu.X, 0x01);
        assert_eq!(cpu.Y, 0x00);
        assert_eq!(mem[600], 0xA2);
        assert_eq!(mem[601], 0x01);
        assert_eq!(mem[602], 0x8E);
        assert_eq!(mem[603], 0x00);
        assert_eq!(mem[604], 0x00);
        assert_eq!(mem[605], 0xAD);
        assert_eq!(mem[606], 0x00);
        assert_eq!(mem[607], 0x00);
    }

    #[test]
    fn lda_absolute_x() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);

        mem[600] = 0xA2;
        mem[601] = 0x01;
        mem[602] = 0x8E;
        mem[603] = 0x02;
        mem[604] = 0x00;
        mem[605] = 0xBD;
        mem[606] = 0x01;
        mem[607] = 0x00;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x01);
        assert_eq!(cpu.X, 0x01);
        assert_eq!(cpu.Y, 0x00);
        assert_eq!(mem[600], 0xA2);
        assert_eq!(mem[601], 0x01);
        assert_eq!(mem[602], 0x8E);
        assert_eq!(mem[603], 0x02);
        assert_eq!(mem[604], 0x00);
        assert_eq!(mem[605], 0xBD);
        assert_eq!(mem[606], 0x01);
        assert_eq!(mem[607], 0x00);
    }

    #[test]
    fn lda_absolute_y() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);

        mem[600] = 0xA0;
        mem[601] = 0x01;
        mem[602] = 0x8C;
        mem[603] = 0x02;
        mem[604] = 0x00;
        mem[605] = 0xB9;
        mem[606] = 0x01;
        mem[607] = 0x00;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x01);
        assert_eq!(cpu.X, 0x00);
        assert_eq!(cpu.Y, 0x01);
        assert_eq!(mem[600], 0xA0);
        assert_eq!(mem[601], 0x01);
        assert_eq!(mem[602], 0x8C);
        assert_eq!(mem[603], 0x02);
        assert_eq!(mem[604], 0x00);
        assert_eq!(mem[605], 0xB9);
        assert_eq!(mem[606], 0x01);
        assert_eq!(mem[607], 0x00);
    }

    #[test]
    fn lda_indirect_x() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);

        mem[600] = 0xA2;
        mem[601] = 0x01;
        mem[602] = 0xA9;
        mem[603] = 0x05;
        mem[604] = 0x85;
        mem[605] = 0x01;
        mem[606] = 0xA9;
        mem[607] = 0x07;
        mem[608] = 0x85;
        mem[609] = 0x02;
        mem[610] = 0xA0;
        mem[611] = 0x0A;
        mem[612] = 0x8C;
        mem[613] = 0x05;
        mem[614] = 0x07;
        mem[615] = 0xA1;
        mem[616] = 0x00;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x0a);
        assert_eq!(cpu.X, 0x01);
        assert_eq!(cpu.Y, 0x0a);
        assert_eq!(mem[600], 0xA2);
        assert_eq!(mem[601], 0x01);
        assert_eq!(mem[602], 0xA9);
        assert_eq!(mem[603], 0x05);
        assert_eq!(mem[604], 0x85);
        assert_eq!(mem[605], 0x01);
        assert_eq!(mem[606], 0xA9);
        assert_eq!(mem[607], 0x07);
        assert_eq!(mem[608], 0x85);
        assert_eq!(mem[609], 0x02);
        assert_eq!(mem[610], 0xA0);
        assert_eq!(mem[611], 0x0A);
        assert_eq!(mem[612], 0x8C);
        assert_eq!(mem[613], 0x05);
        assert_eq!(mem[614], 0x07);
        assert_eq!(mem[615], 0xA1);
        assert_eq!(mem[616], 0x00);
    }

    #[test]
    fn lda_indirect_y() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);

        mem[600] = 0xa0;
        mem[601] = 0x01;
        mem[602] = 0xa9;
        mem[603] = 0x03;
        mem[604] = 0x85;
        mem[605] = 0x01;
        mem[606] = 0xa9;
        mem[607] = 0x07;
        mem[608] = 0x85;
        mem[609] = 0x02;
        mem[610] = 0xa2;
        mem[611] = 0x0a;
        mem[612] = 0x8e;
        mem[613] = 0x04;
        mem[614] = 0x07;
        mem[615] = 0xb1;
        mem[616] = 0x01;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x0a);
        assert_eq!(cpu.X, 0x0a);
        assert_eq!(cpu.Y, 0x01);
        assert_eq!(mem[600], 0xa0);
        assert_eq!(mem[601], 0x01);
        assert_eq!(mem[602], 0xa9);
        assert_eq!(mem[603], 0x03);
        assert_eq!(mem[604], 0x85);
        assert_eq!(mem[605], 0x01);
        assert_eq!(mem[606], 0xa9);
        assert_eq!(mem[607], 0x07);
        assert_eq!(mem[608], 0x85);
        assert_eq!(mem[609], 0x02);
        assert_eq!(mem[610], 0xa2);
        assert_eq!(mem[611], 0x0a);
        assert_eq!(mem[612], 0x8e);
        assert_eq!(mem[613], 0x04);
        assert_eq!(mem[614], 0x07);
        assert_eq!(mem[615], 0xb1);
        assert_eq!(mem[616], 0x01);
    }
}
