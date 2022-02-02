mod sta_tests {
    use rusty_6502::{cpu::CPU, mem::MEM};

    #[test]
    fn sta_zero_page() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0xA9;
        mem[601] = 0x03;
        mem[602] = 0x85;
        mem[603] = 0x01;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x03);
        assert_eq!(cpu.X, 0x00);
        assert_eq!(cpu.Y, 0x00);
        assert_eq!(mem[600], 0xA9);
        assert_eq!(mem[601], 0x03);
        assert_eq!(mem[602], 0x85);
        assert_eq!(mem[603], 0x01);
        assert_eq!(mem[1], 0x03);
    }

    #[test]
    fn sta_zero_page_x() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0xA2;
        mem[601] = 0x01;
        mem[602] = 0xA9;
        mem[603] = 0x03;
        mem[604] = 0x95;
        mem[605] = 0x01;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x03);
        assert_eq!(cpu.X, 0x01);
        assert_eq!(cpu.Y, 0x00);
        assert_eq!(mem[600], 0xA2);
        assert_eq!(mem[601], 0x01);
        assert_eq!(mem[602], 0xA9);
        assert_eq!(mem[603], 0x03);
        assert_eq!(mem[604], 0x95);
        assert_eq!(mem[605], 0x01);
        assert_eq!(mem[2], 0x03);
    }

    #[test]
    fn sta_absolute() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0xA9;
        mem[601] = 0x03;
        mem[602] = 0x8D;
        mem[603] = 0x01;
        mem[604] = 0x00;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x03);
        assert_eq!(cpu.X, 0x00);
        assert_eq!(cpu.Y, 0x00);
        assert_eq!(mem[600], 0xA9);
        assert_eq!(mem[601], 0x03);
        assert_eq!(mem[602], 0x8D);
        assert_eq!(mem[603], 0x01);
        assert_eq!(mem[604], 0x00);
        assert_eq!(mem[1], 0x03);
    }

    #[test]
    fn sta_absolute_x() {
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
        assert_eq!(mem[2], 0x03);
    }

    #[test]
    fn sta_absolute_y() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        mem[600] = 0xA9;
        mem[601] = 0x03;
        mem[602] = 0xA0;
        mem[603] = 0x01;
        mem[604] = 0x99;
        mem[605] = 0x01;
        mem[606] = 0x00;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x03);
        assert_eq!(cpu.X, 0x00);
        assert_eq!(cpu.Y, 0x01);
        assert_eq!(mem[600], 0xA9);
        assert_eq!(mem[601], 0x03);
        assert_eq!(mem[602], 0xA0);
        assert_eq!(mem[603], 0x01);
        assert_eq!(mem[604], 0x99);
        assert_eq!(mem[605], 0x01);
        assert_eq!(mem[606], 0x00);
        assert_eq!(mem[2], 0x03);
    }

    #[test]
    fn sta_indirect_x() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        //a2 01 a9 01 85 01 a9 02 85 02 a0 0a 8c 02 01 81
        mem[600] = 0xA2;
        mem[601] = 0x01;
        mem[602] = 0xA9;
        mem[603] = 0x01;
        mem[604] = 0x85;
        mem[605] = 0x01;
        mem[606] = 0xA9;
        mem[607] = 0x02;
        mem[608] = 0x85;
        mem[609] = 0x02;
        mem[610] = 0xA0;
        mem[611] = 0x0A;
        mem[612] = 0x8C;
        mem[613] = 0x02;
        mem[614] = 0x01;
        mem[615] = 0x81;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x02);
        assert_eq!(cpu.X, 0x01);
        assert_eq!(cpu.Y, 0x0A);
        assert_eq!(mem[600], 0xA2);
        assert_eq!(mem[601], 0x01);
        assert_eq!(mem[602], 0xA9);
        assert_eq!(mem[603], 0x01);
        assert_eq!(mem[604], 0x85);
        assert_eq!(mem[605], 0x01);
        assert_eq!(mem[606], 0xA9);
        assert_eq!(mem[607], 0x02);
        assert_eq!(mem[608], 0x85);
        assert_eq!(mem[609], 0x02);
        assert_eq!(mem[610], 0xA0);
        assert_eq!(mem[611], 0x0A);
        assert_eq!(mem[612], 0x8C);
        assert_eq!(mem[613], 0x02);
        assert_eq!(mem[614], 0x01);
        assert_eq!(mem[615], 0x81);
        assert_eq!(mem[1], 0x01);
        assert_eq!(mem[2], 0x02);
        assert_eq!(mem[513], 0x02); //0x201
    }

    #[test]
    fn sta_indirect_y() {
        let mut mem = MEM::new();
        let mut cpu = CPU::new(|_| {});
        cpu.reset(600, &mut mem);
        //a2 01 a9 01 85 01 a9 02 85 02 a0 0a 8c 02 01 81
        mem[600] = 0xA2;
        mem[601] = 0x01;
        mem[602] = 0xA9;
        mem[603] = 0x01;
        mem[604] = 0x85;
        mem[605] = 0x00;
        mem[606] = 0xA9;
        mem[607] = 0x02;
        mem[608] = 0x85;
        mem[609] = 0x01;
        mem[610] = 0xA0;
        mem[611] = 0x0A;
        mem[612] = 0x8C;
        mem[613] = 0x02;
        mem[614] = 0x01;
        mem[615] = 0x91;
        mem[616] = 0x00;

        cpu.execute_continuous(&mut mem);
        assert_eq!(cpu.A, 0x02);
        assert_eq!(cpu.X, 0x01);
        assert_eq!(cpu.Y, 0x0A);
        assert_eq!(mem[600], 0xA2);
        assert_eq!(mem[601], 0x01);
        assert_eq!(mem[602], 0xA9);
        assert_eq!(mem[603], 0x01);
        assert_eq!(mem[604], 0x85);
        assert_eq!(mem[605], 0x00);
        assert_eq!(mem[606], 0xA9);
        assert_eq!(mem[607], 0x02);
        assert_eq!(mem[608], 0x85);
        assert_eq!(mem[609], 0x01);
        assert_eq!(mem[610], 0xA0);
        assert_eq!(mem[611], 0x0A);
        assert_eq!(mem[612], 0x8C);
        assert_eq!(mem[613], 0x02);
        assert_eq!(mem[614], 0x01);
        assert_eq!(mem[615], 0x91);
        assert_eq!(mem[616], 0x00);
        assert_eq!(mem[0], 0x01);
        assert_eq!(mem[1], 0x02);
        assert_eq!(mem[523], 0x02); //0x20B
    }
}
