pub mod cartridge;
pub mod cpu;
pub mod mmu;

use cpu::Cpu;
use mmu::Mmu;

pub struct Emu {
    pub cpu: Cpu,
}

impl Emu {
    pub fn from(rom: &[u8]) -> Self {
        Self {
            cpu: Cpu::new(Mmu::load_cartridge(rom)),
        }
    }
}
