use std::ops::{Index, IndexMut};

const MAX_MEM: usize = 1024 * 64;

/// Memory
#[derive(Debug, Clone)]
pub struct MEM {
    /// Memory data
    pub data: [u8; MAX_MEM],
}

//Read 1 byte
impl Index<usize> for MEM {
    type Output = u8;
    fn index<'a>(&'a self, i: usize) -> &'a u8 {
        &self.data[i]
    }
}

//Write 1 byte
impl IndexMut<usize> for MEM {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut u8 {
        &mut self.data[i]
    }
}

impl MEM {
    /// Create a new memory
    pub fn new() -> MEM {
        MEM { data: [0; MAX_MEM] }
    }

    /// Fill memory with data
    pub fn initalize(&mut self) {
        self.data = [0; MAX_MEM];
    }

    /// Write data to memory
    /// ## Arguments
    /// * `cycles` - Cycle to reduce [`u32`]
    /// * `value` - Value to write [`u8`]
    /// * `address` - Address to write [`u32`]
    pub fn write_word(&mut self, cycles: &mut u32, value: u16, address: u32) {
        self[address as usize] = (value as u8) & 0xff;
        self[(address + 1) as usize] = (value >> 8) as u8;
        *cycles -= 2;
    }

    /// Hex dump memory
    pub fn hex_dump(&self, start: usize, end: usize) -> String {
        let mut dump = String::new();
        for i in start..end {
            let byte = self.data[i];
            dump += &format!("{:02X} ", byte);
        }
        dump
    }
}
