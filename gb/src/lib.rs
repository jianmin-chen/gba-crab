pub mod apu;
pub mod cartridge;
pub mod cpu;
pub mod mmu;
pub mod ppu;
pub mod util;

use cpu::Cpu;
use mmu::Mmu;
use ppu::Ppu;

pub struct Emu<'ppu> {
    pub cpu: Cpu<'ppu>,
    pub ppu: Ppu,
}

impl<'ppu> Emu<'ppu> {
    pub fn from(rom: &[u8]) -> Self {
        let mut ppu = Ppu::new();
        let mut mmu = Mmu::new(&mut ppu);
        mmu.load_cartridge(rom);
        Self {
            ppu,
            cpu: Cpu::new(mmu),
        }
    }
}
