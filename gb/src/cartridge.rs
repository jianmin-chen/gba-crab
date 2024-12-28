pub trait Cartridge {
    fn read(&self, addr: u16) -> u8;
    fn set(&mut self, addr: u16, value: u8);
}

#[derive(Debug)]
pub struct NoMbc {
    rom: [u8; 0x8000],
    ram: [u8; 0x2000],
}

impl NoMbc {
    pub fn from(rom: &[u8]) -> Self {
        let mut cart_rom = [0; 0x8000];
        cart_rom[0..rom.len()].copy_from_slice(rom);
        Self {
            rom: cart_rom,
            ram: [0; 0x2000],
        }
    }
}

impl Cartridge for NoMbc {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0..=0x7fff => self.rom[addr as usize],
            0xa000..=0xbfff => self.ram[addr as usize],
            _ => unreachable!(),
        }
    }

    fn set(&mut self, addr: u16, byte: u8) {
        match addr {
            0xa000..=0xbfff => self.ram[addr as usize] = byte,
            0..=0x7fff => panic!("Attempt to write to a read-only ROM address {:#X}", addr),
            _ => unreachable!(),
        }
    }
}
