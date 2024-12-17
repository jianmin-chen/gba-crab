use crate::cartridge::*;

pub struct Mmu {
    pub cartridge: Option<Box<dyn Cartridge>>,
    pub work_ram: [u8; 0x2000],
    pub high_ram: [u8; 0x100],
    pub boot_rom: [u8; 0x100],

    pub in_boot_rom: bool,
}

impl Mmu {
    pub fn new() -> Self {
        Self {
            cartridge: None,
            work_ram: [0; 0x2000],
            high_ram: [0; 0x100],
            boot_rom: *include_bytes!("dmg_boot.bin"),

            in_boot_rom: true,
        }
    }

    pub fn load_cartridge(rom: &[u8]) -> Self {
        let mbc_type = rom[0x0147];
        let cartridge: Box<dyn Cartridge> = match mbc_type {
            0 => Box::new(NoMbc::from(rom)),
            _ => todo!("MBC not implemented: {}", mbc_type),
        };
        Self {
            cartridge: Some(cartridge),
            work_ram: [0; 0x2000],
            high_ram: [0; 0x100],
            boot_rom: *include_bytes!("dmg_boot.bin"),

            in_boot_rom: true,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0..=0x7fff => {
                if self.in_boot_rom {
                    self.boot_rom[addr as usize]
                } else {
                    if let Some(cartridge) = self.cartridge.as_ref() {
                        cartridge.read(addr)
                    } else {
                        panic!("Unable to read address {:#X}", addr);
                    }
                }
            }
            0x8000..=0x9fff => todo!("Implement PPU"),
            0xa000..=0xbfff => {
                if let Some(cartridge) = self.cartridge.as_ref() {
                    cartridge.read(addr)
                } else {
                    panic!("Unable to read address {:#X}", addr);
                }
            }
            0xc000..=0xcfff => self.work_ram[addr as usize],
            0xd000..=0xdfff => self.work_ram[addr as usize], // TODO: In CGB mode, switchable bank 1 - 7
            0xe000..=0xfdff | 0xfea0..=0xfeff => panic!("Prohibited address {:#X}", addr),
            0xfe00..=0xfe9f => todo!("Object attribute memory: implement PPU"),
            0xff00..=0xff7f => todo!("I/O registers"),
            0xffff => todo!("Interrupts"),
            _ => panic!("Invalid address {:#X}", addr),
        }
    }
}
