use crate::apu::*;
use crate::cartridge::*;
use crate::ppu::*;

pub struct Mmu<'ppu> {
    pub apu: Apu,
    pub ppu: Option<&'ppu mut Ppu>,

    pub cartridge: Option<Box<dyn Cartridge>>,
    pub work_ram: [u8; 0x2000],
    pub video_ram: [u8; 0x2000],
    pub high_ram: [u8; 0x100],
    pub boot_rom: [u8; 0x100],

    pub in_boot_rom: bool,
}

impl<'ppu> Mmu<'ppu> {
    pub fn new(ppu: &'ppu mut Ppu) -> Self {
        Self {
            apu: Apu::new(),
            ppu: Some(ppu),

            cartridge: None,
            work_ram: [0; 0x2000],
            video_ram: [0; 0x2000],
            high_ram: [0; 0x100],
            boot_rom: *include_bytes!("dmg_boot.bin"),

            in_boot_rom: true,
        }
    }

    pub fn load_cartridge(&mut self, rom: &[u8]) {
        let mbc_type = rom[0x0147];
        let cartridge: Box<dyn Cartridge> = match mbc_type {
            0 => Box::new(NoMbc::from(rom)),
            _ => todo!("MBC not implemented: {}", mbc_type),
        };
        self.cartridge = Some(cartridge);
    }

    // Read and write to appropriate
    // sector of memory.
    //
    // [0, 0x7fff] = boot ROM, or cartridge

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0..=0x7fff => {
                if self.in_boot_rom && addr <= 0xff {
                    return self.boot_rom[addr as usize];
                }

                if let Some(cartridge) = self.cartridge.as_ref() {
                    cartridge.read(addr)
                } else {
                    panic!("Unable to read address {:#X}", addr);
                }
            }
            0x8000..=0x9fff => todo!("Implement PPU"), // TODO: In CGB mode, switchable bank 0/1
            0xa000..=0xbfff => {
                if let Some(cartridge) = self.cartridge.as_ref() {
                    cartridge.read(addr)
                } else {
                    panic!("Unable to read address {:#X}", addr);
                }
            }
            0xc000..=0xcfff => todo!("Work RAM"),
            0xd000..=0xdfff => todo!("Work RAM, in CGB mode switable bank 1 - 7"),
            0xe000..=0xfdff | 0xfea0..=0xfeff => panic!("Prohibitable address {:#X}", addr),
            0xfe00..=0xfe9f => todo!("Object attribute memory: implement PPU"),
            0xff00..=0xff7f => self.ppu.as_ref().unwrap().read(addr),
            0xffff => todo!("Interrupts"),
            _ => panic!("Invalid address {:#X}", addr),
        }
    }

    pub fn set(&mut self, addr: u16, byte: u8) {
        match addr {
            0..=0x7fff => {
                if self.in_boot_rom {
                    panic!("Attempt to write to a read-only ROM address {:#X}", addr);
                } else if let Some(cartridge) = self.cartridge.as_mut() {
                    cartridge.set(addr, byte);
                } else {
                    panic!("Invalid address {:#X}", addr);
                }
            }
            0x8000..=0x9fff => {
                self.video_ram[(addr - 0x8000) as usize] = byte;
            }
            0xa000..=0xbfff => {
                if let Some(cartridge) = self.cartridge.as_mut() {
                    cartridge.set(addr, byte);
                } else {
                    panic!("Invalid address {:#X}", addr);
                }
            }
            0xff40..=0xff55 => self.ppu.as_mut().unwrap().handle(addr, byte),
            0xff10..=0xff3f => self.apu.handle(addr, byte),
            _ => panic!("Invalid address {:#X}", addr),
        }
    }
}
