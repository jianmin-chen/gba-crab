#![allow(unused)]
use crate::mmu::Mmu;
use crate::util::{masked, signed};
use std::ops::{Sub, SubAssign};

pub mod instruction;

use instruction::{Instruction, InstructionHandler, PrefixInstruction};

pub struct Cpu<'ppu> {
    registers: Registers,
    stack_pointer: u16,
    stack: Vec<u16>,
    pub program_counter: u16,

    pub mmu: Mmu<'ppu>,
}

impl<'ppu> Cpu<'ppu> {
    pub fn new(mmu: Mmu<'ppu>) -> Self {
        Self {
            registers: Registers::empty(),
            stack_pointer: 0,
            stack: Vec::new(),
            program_counter: 0,

            mmu,
        }
    }

    pub fn read(&mut self) -> u8 {
        let byte = self.mmu.read(self.program_counter);
        self.program_counter += 1;
        byte
    }

    // Little-endian, the lower byte comes first.
    pub fn read_word(&mut self) -> u16 {
        let high: u16 = self.read().into();
        let low: u16 = self.read().into();
        (low << 8) | high
    }

    pub fn fetch(&mut self) -> Instruction {
        let opcode = self.read();
        match opcode {
            0x98 => Instruction::SBC_ADDR_A_B,
            0xb7 => Instruction::OR_ADDR_A_A,
            0x32 => Instruction::LD_HL_DEC_A,
            0x09 => Instruction::ADD_ADDR_HL_BC,
            0xc3 => Instruction::JP_A16(self.read_word()),
            0x8a => Instruction::ADC_ADDR_A_D,
            0x67 => Instruction::LD_H_A,
            0x46 => Instruction::LD_B_HL,
            0xf7 => Instruction::RST_30,
            0x4c => Instruction::LD_C_H,
            0x83 => Instruction::ADD_ADDR_A_E,
            0xbb => Instruction::CP_ADDR_A_E,
            0x31 => Instruction::LD_SP_N16(self.read_word()),
            0x24 => Instruction::INC_ADDR_H,
            0x9c => Instruction::SBC_ADDR_A_H,
            0xad => Instruction::XOR_ADDR_A_L,
            0x16 => Instruction::LD_D_N8(self.read()),
            0x07 => Instruction::RLCA_ADDR,
            0x5d => Instruction::LD_E_L,
            0x59 => Instruction::LD_E_C,
            0x81 => Instruction::ADD_ADDR_A_C,
            0xff => Instruction::RST_38,
            0x92 => Instruction::SUB_ADDR_A_D,
            0x0f => Instruction::RRCA_ADDR,
            0x90 => Instruction::SUB_ADDR_A_B,
            0xdc => Instruction::CALL_C_A16(self.read_word()),
            0xe2 => Instruction::LDH_C_A,
            0x10 => Instruction::STOP_N8(self.read()),
            0x64 => Instruction::LD_H_H,
            0xcb => Instruction::PREFIX(self.fetch_prefix()),
            0xaa => Instruction::XOR_ADDR_A_D,
            0xc8 => Instruction::RET_Z,
            0xe5 => Instruction::PUSH_HL,
            0xb9 => Instruction::CP_ADDR_A_C,
            0xb6 => Instruction::OR_ADDR_A_HL,
            0x0a => Instruction::LD_A_BC,
            0x25 => Instruction::DEC_ADDR_H,
            0x13 => Instruction::INC_DE,
            0x1f => Instruction::RRA_ADDR,
            0x50 => Instruction::LD_D_B,
            0xf8 => Instruction::LD_ADDR_HL_SP_INC_E8(self.read()),
            0x2c => Instruction::INC_ADDR_L,
            0x14 => Instruction::INC_ADDR_D,
            0x68 => Instruction::LD_L_B,
            0x88 => Instruction::ADC_ADDR_A_B,
            0xa2 => Instruction::AND_ADDR_A_D,
            0xe6 => Instruction::AND_ADDR_A_N8(self.read()),
            0xee => Instruction::XOR_ADDR_A_N8(self.read()),
            0xfe => Instruction::CP_ADDR_A_N8(self.read()),
            0x75 => Instruction::LD_HL_L,
            0xae => Instruction::XOR_ADDR_A_HL,
            0x61 => Instruction::LD_H_C,
            0x15 => Instruction::DEC_ADDR_D,
            0x23 => Instruction::INC_HL,
            0x38 => Instruction::JR_C_E8(self.read()),
            0xe0 => Instruction::LDH_A8_A(self.read()),
            0x8f => Instruction::ADC_ADDR_A_A,
            0x48 => Instruction::LD_C_B,
            0x91 => Instruction::SUB_ADDR_A_C,
            0x79 => Instruction::LD_A_C,
            0x7e => Instruction::LD_A_HL,
            0xd6 => Instruction::SUB_ADDR_A_N8(self.read()),
            0x20 => Instruction::JR_NZ_E8(self.read()),
            0x0e => Instruction::LD_C_N8(self.read()),
            0x6b => Instruction::LD_L_E,
            0x18 => Instruction::JR_E8(self.read()),
            0x9e => Instruction::SBC_ADDR_A_HL,
            0x05 => Instruction::DEC_ADDR_B,
            0x44 => Instruction::LD_B_H,
            0x21 => Instruction::LD_HL_N16(self.read_word()),
            0x47 => Instruction::LD_B_A,
            0x9f => Instruction::SBC_ADDR_A_A,
            0xd5 => Instruction::PUSH_DE,
            0x93 => Instruction::SUB_ADDR_A_E,
            0x4d => Instruction::LD_C_L,
            0x37 => Instruction::SCF_ADDR,
            0x6c => Instruction::LD_L_H,
            0x96 => Instruction::SUB_ADDR_A_HL,
            0x9a => Instruction::SBC_ADDR_A_D,
            0x7c => Instruction::LD_A_H,
            0xb4 => Instruction::OR_ADDR_A_H,
            0x80 => Instruction::ADD_ADDR_A_B,
            0x29 => Instruction::ADD_ADDR_HL_HL,
            0x49 => Instruction::LD_C_C,
            0x3d => Instruction::DEC_ADDR_A,
            0x63 => Instruction::LD_H_E,
            0x41 => Instruction::LD_B_C,
            0x5f => Instruction::LD_E_A,
            0xe1 => Instruction::POP_HL,
            0x06 => Instruction::LD_B_N8(self.read()),
            0x3e => Instruction::LD_A_N8(self.read()),
            0x69 => Instruction::LD_L_C,
            0x82 => Instruction::ADD_ADDR_A_D,
            0xcf => Instruction::RST_08,
            0x4b => Instruction::LD_C_E,
            0x04 => Instruction::INC_ADDR_B,
            0x1d => Instruction::DEC_ADDR_E,
            0x55 => Instruction::LD_D_L,
            0xd0 => Instruction::RET_NC,
            0x5c => Instruction::LD_E_H,
            0xea => Instruction::LD_A16_A(self.read_word()),
            0x36 => Instruction::LD_HL_N8(self.read()),
            0x4f => Instruction::LD_C_A,
            0xa7 => Instruction::AND_ADDR_A_A,
            0x56 => Instruction::LD_D_HL,
            0xf6 => Instruction::OR_ADDR_A_N8(self.read()),
            0xb5 => Instruction::OR_ADDR_A_L,
            0xd9 => Instruction::RETI,
            0xf9 => Instruction::LD_SP_HL,
            0xe9 => Instruction::JP_HL,
            0x3b => Instruction::DEC_SP,
            0x45 => Instruction::LD_B_L,
            0xbc => Instruction::CP_ADDR_A_H,
            0xa9 => Instruction::XOR_ADDR_A_C,
            0x53 => Instruction::LD_D_E,
            0x97 => Instruction::SUB_ADDR_A_A,
            0xca => Instruction::JP_Z_A16(self.read_word()),
            0x1b => Instruction::DEC_DE,
            0x22 => Instruction::LD_HL_INC_A,
            0xc2 => Instruction::JP_NZ_A16(self.read_word()),
            0x2e => Instruction::LD_L_N8(self.read()),
            0x72 => Instruction::LD_HL_D,
            0x33 => Instruction::INC_SP,
            0x0b => Instruction::DEC_BC,
            0xd4 => Instruction::CALL_NC_A16(self.read_word()),
            0xf0 => Instruction::LDH_A_A8(self.read()),
            0xd7 => Instruction::RST_10,
            0x52 => Instruction::LD_D_D,
            0x66 => Instruction::LD_H_HL,
            0x62 => Instruction::LD_H_D,
            0x77 => Instruction::LD_HL_A,
            0xa5 => Instruction::AND_ADDR_A_L,
            0x35 => Instruction::DEC_ADDR_HL,
            0x4a => Instruction::LD_C_D,
            0x94 => Instruction::SUB_ADDR_A_H,
            0xb2 => Instruction::OR_ADDR_A_D,
            0x8b => Instruction::ADC_ADDR_A_E,
            0xac => Instruction::XOR_ADDR_A_H,
            0x3a => Instruction::LD_A_HL_DEC,
            0x0d => Instruction::DEC_ADDR_C,
            0x9d => Instruction::SBC_ADDR_A_L,
            0xde => Instruction::SBC_ADDR_A_N8(self.read()),
            0x6a => Instruction::LD_L_D,
            0x6d => Instruction::LD_L_L,
            0x00 => Instruction::NOP,
            0x02 => Instruction::LD_BC_A,
            0x03 => Instruction::INC_BC,
            0x60 => Instruction::LD_H_B,
            0x2d => Instruction::DEC_ADDR_L,
            0x70 => Instruction::LD_HL_B,
            0x95 => Instruction::SUB_ADDR_A_L,
            0xb3 => Instruction::OR_ADDR_A_E,
            0x17 => Instruction::RLA_ADDR,
            0xa4 => Instruction::AND_ADDR_A_H,
            0x08 => Instruction::LD_A16_SP(self.read_word()),
            0x1e => Instruction::LD_E_N8(self.read()),
            0x5a => Instruction::LD_E_D,
            0xcd => Instruction::CALL_A16(self.read_word()),
            0x7f => Instruction::LD_A_A,
            0xc0 => Instruction::RET_NZ,
            0x65 => Instruction::LD_H_L,
            0x42 => Instruction::LD_B_D,
            0x78 => Instruction::LD_A_B,
            0x27 => Instruction::DAA_ADDR,
            0xbd => Instruction::CP_ADDR_A_L,
            0xf5 => Instruction::PUSH_AF,
            0x3c => Instruction::INC_ADDR_A,
            0xcc => Instruction::CALL_Z_A16(self.read_word()),
            0x86 => Instruction::ADD_ADDR_A_HL,
            0xc6 => Instruction::ADD_ADDR_A_N8(self.read()),
            0x7d => Instruction::LD_A_L,
            0xc9 => Instruction::RET,
            0xce => Instruction::ADC_ADDR_A_N8(self.read()),
            0x6f => Instruction::LD_L_A,
            0xf1 => Instruction::POP_ADDR_AF,
            0x1c => Instruction::INC_ADDR_E,
            0x26 => Instruction::LD_H_N8(self.read()),
            0x85 => Instruction::ADD_ADDR_A_L,
            0xa3 => Instruction::AND_ADDR_A_E,
            0x30 => Instruction::JR_NC_E8(self.read()),
            0xc7 => Instruction::RST_00,
            0xf2 => Instruction::LDH_A_C,
            0x58 => Instruction::LD_E_B,
            0xb8 => Instruction::CP_ADDR_A_B,
            0x11 => Instruction::LD_DE_N16(self.read_word()),
            0x34 => Instruction::INC_ADDR_HL,
            0x6e => Instruction::LD_L_HL,
            0xf3 => Instruction::DI,
            0x28 => Instruction::JR_Z_E8(self.read()),
            0x89 => Instruction::ADC_ADDR_A_C,
            0x0c => Instruction::INC_ADDR_C,
            0x2b => Instruction::DEC_HL,
            0x19 => Instruction::ADD_ADDR_HL_DE,
            0xc1 => Instruction::POP_BC,
            0x9b => Instruction::SBC_ADDR_A_E,
            0xd2 => Instruction::JP_NC_A16(self.read_word()),
            0x73 => Instruction::LD_HL_E,
            0x71 => Instruction::LD_HL_C,
            0x7b => Instruction::LD_A_E,
            0xef => Instruction::RST_28,
            0x99 => Instruction::SBC_ADDR_A_C,
            0x8d => Instruction::ADC_ADDR_A_L,
            0x39 => Instruction::ADD_ADDR_HL_SP,
            0x84 => Instruction::ADD_ADDR_A_H,
            0x43 => Instruction::LD_B_E,
            0xab => Instruction::XOR_ADDR_A_E,
            0xb1 => Instruction::OR_ADDR_A_C,
            0xd1 => Instruction::POP_DE,
            0x3f => Instruction::CCF_ADDR,
            0x8c => Instruction::ADC_ADDR_A_H,
            0x2f => Instruction::CPL_ADDR,
            0x7a => Instruction::LD_A_D,
            0x51 => Instruction::LD_D_C,
            0xa6 => Instruction::AND_ADDR_A_HL,
            0xd8 => Instruction::RET_C,
            0x57 => Instruction::LD_D_A,
            0x5b => Instruction::LD_E_E,
            0x8e => Instruction::ADC_ADDR_A_HL,
            0x2a => Instruction::LD_A_HL_INC,
            0x01 => Instruction::LD_BC_N16(self.read_word()),
            0xa1 => Instruction::AND_ADDR_A_C,
            0x4e => Instruction::LD_C_HL,
            0x5e => Instruction::LD_E_HL,
            0x76 => Instruction::HALT,
            0xba => Instruction::CP_ADDR_A_D,
            0xa0 => Instruction::AND_ADDR_A_B,
            0xfb => Instruction::EI,
            0x40 => Instruction::LD_B_B,
            0xc4 => Instruction::CALL_NZ_A16(self.read_word()),
            0xc5 => Instruction::PUSH_BC,
            0x12 => Instruction::LD_DE_A,
            0x54 => Instruction::LD_D_H,
            0xda => Instruction::JP_C_A16(self.read_word()),
            0xe7 => Instruction::RST_20,
            0xa8 => Instruction::XOR_ADDR_A_B,
            0x1a => Instruction::LD_A_DE,
            0x74 => Instruction::LD_HL_H,
            0xbf => Instruction::CP_ADDR_A_A,
            0xb0 => Instruction::OR_ADDR_A_B,
            0xe8 => Instruction::ADD_ADDR_SP_E8(self.read()),
            0xaf => Instruction::XOR_ADDR_A_A,
            0xfa => Instruction::LD_A_A16(self.read_word()),
            0x87 => Instruction::ADD_ADDR_A_A,
            0xdf => Instruction::RST_18,
            0xbe => Instruction::CP_ADDR_A_HL,
            _ => panic!("Invalid opcode {:#X}", opcode),
        }
    }

    fn fetch_prefix(&mut self) -> PrefixInstruction {
        let opcode = self.read();
        match opcode {
            0x71 => PrefixInstruction::BIT_6_C,
            0x78 => PrefixInstruction::BIT_7_B,
            0x7f => PrefixInstruction::BIT_7_A,
            0x13 => PrefixInstruction::RL_E,
            0x88 => PrefixInstruction::RES_1_B,
            0xc0 => PrefixInstruction::SET_0_B,
            0xf5 => PrefixInstruction::SET_6_L,
            0x9e => PrefixInstruction::RES_3_HL,
            0x24 => PrefixInstruction::SLA_H,
            0x57 => PrefixInstruction::BIT_2_A,
            0x03 => PrefixInstruction::RLC_E,
            0x10 => PrefixInstruction::RL_B,
            0x0c => PrefixInstruction::RRC_H,
            0xe6 => PrefixInstruction::SET_4_HL,
            0xb7 => PrefixInstruction::RES_6_A,
            0x81 => PrefixInstruction::RES_0_C,
            0x20 => PrefixInstruction::SLA_B,
            0xd8 => PrefixInstruction::SET_3_B,
            0xef => PrefixInstruction::SET_5_A,
            0x41 => PrefixInstruction::BIT_0_C,
            0x7d => PrefixInstruction::BIT_7_L,
            0x0f => PrefixInstruction::RRC_A,
            0x75 => PrefixInstruction::BIT_6_L,
            0xc4 => PrefixInstruction::SET_0_H,
            0xdc => PrefixInstruction::SET_3_H,
            0x8e => PrefixInstruction::RES_1_HL,
            0x62 => PrefixInstruction::BIT_4_D,
            0xdd => PrefixInstruction::SET_3_L,
            0xed => PrefixInstruction::SET_5_L,
            0x69 => PrefixInstruction::BIT_5_C,
            0x47 => PrefixInstruction::BIT_0_A,
            0x8a => PrefixInstruction::RES_1_D,
            0x42 => PrefixInstruction::BIT_0_D,
            0xa6 => PrefixInstruction::RES_4_HL,
            0xcc => PrefixInstruction::SET_1_H,
            0x02 => PrefixInstruction::RLC_D,
            0xd1 => PrefixInstruction::SET_2_C,
            0x83 => PrefixInstruction::RES_0_E,
            0xf3 => PrefixInstruction::SET_6_E,
            0xe1 => PrefixInstruction::SET_4_C,
            0x37 => PrefixInstruction::SWAP_A,
            0x85 => PrefixInstruction::RES_0_L,
            0x0d => PrefixInstruction::RRC_L,
            0x86 => PrefixInstruction::RES_0_HL,
            0x80 => PrefixInstruction::RES_0_B,
            0xb9 => PrefixInstruction::RES_7_C,
            0x68 => PrefixInstruction::BIT_5_B,
            0x27 => PrefixInstruction::SLA_A,
            0x18 => PrefixInstruction::RR_B,
            0x65 => PrefixInstruction::BIT_4_L,
            0x2c => PrefixInstruction::SRA_H,
            0x05 => PrefixInstruction::RLC_L,
            0x0a => PrefixInstruction::RRC_D,
            0x12 => PrefixInstruction::RL_D,
            0x6a => PrefixInstruction::BIT_5_D,
            0x09 => PrefixInstruction::RRC_C,
            0x01 => PrefixInstruction::RLC_C,
            0xf9 => PrefixInstruction::SET_7_C,
            0x90 => PrefixInstruction::RES_2_B,
            0x61 => PrefixInstruction::BIT_4_C,
            0xa7 => PrefixInstruction::RES_4_A,
            0x91 => PrefixInstruction::RES_2_C,
            0xaf => PrefixInstruction::RES_5_A,
            0x2b => PrefixInstruction::SRA_E,
            0x73 => PrefixInstruction::BIT_6_E,
            0xe7 => PrefixInstruction::SET_4_A,
            0x8c => PrefixInstruction::RES_1_H,
            0xc9 => PrefixInstruction::SET_1_C,
            0xf4 => PrefixInstruction::SET_6_H,
            0x1b => PrefixInstruction::RR_E,
            0xa5 => PrefixInstruction::RES_4_L,
            0x3a => PrefixInstruction::SRL_D,
            0x92 => PrefixInstruction::RES_2_D,
            0xdf => PrefixInstruction::SET_3_A,
            0x35 => PrefixInstruction::SWAP_L,
            0x6b => PrefixInstruction::BIT_5_E,
            0xa4 => PrefixInstruction::RES_4_H,
            0xaa => PrefixInstruction::RES_5_D,
            0x67 => PrefixInstruction::BIT_4_A,
            0xd9 => PrefixInstruction::SET_3_C,
            0xbf => PrefixInstruction::RES_7_A,
            0xd5 => PrefixInstruction::SET_2_L,
            0x5a => PrefixInstruction::BIT_3_D,
            0x22 => PrefixInstruction::SLA_D,
            0x31 => PrefixInstruction::SWAP_C,
            0x9c => PrefixInstruction::RES_3_H,
            0x74 => PrefixInstruction::BIT_6_H,
            0x82 => PrefixInstruction::RES_0_D,
            0xcb => PrefixInstruction::SET_1_E,
            0x4f => PrefixInstruction::BIT_1_A,
            0xc3 => PrefixInstruction::SET_0_E,
            0x93 => PrefixInstruction::RES_2_E,
            0x8f => PrefixInstruction::RES_1_A,
            0xe5 => PrefixInstruction::SET_4_L,
            0x36 => PrefixInstruction::SWAP_HL,
            0xd6 => PrefixInstruction::SET_2_HL,
            0x2f => PrefixInstruction::SRA_A,
            0x3e => PrefixInstruction::SRL_HL,
            0x98 => PrefixInstruction::RES_3_B,
            0x5f => PrefixInstruction::BIT_3_A,
            0xea => PrefixInstruction::SET_5_D,
            0x4c => PrefixInstruction::BIT_1_H,
            0xa0 => PrefixInstruction::RES_4_B,
            0x70 => PrefixInstruction::BIT_6_B,
            0xde => PrefixInstruction::SET_3_HL,
            0x79 => PrefixInstruction::BIT_7_C,
            0x7b => PrefixInstruction::BIT_7_E,
            0xfd => PrefixInstruction::SET_7_L,
            0xcd => PrefixInstruction::SET_1_L,
            0x9f => PrefixInstruction::RES_3_A,
            0x44 => PrefixInstruction::BIT_0_H,
            0x6f => PrefixInstruction::BIT_5_A,
            0xca => PrefixInstruction::SET_1_D,
            0x49 => PrefixInstruction::BIT_1_C,
            0xfc => PrefixInstruction::SET_7_H,
            0x6d => PrefixInstruction::BIT_5_L,
            0x45 => PrefixInstruction::BIT_0_L,
            0x28 => PrefixInstruction::SRA_B,
            0x55 => PrefixInstruction::BIT_2_L,
            0x87 => PrefixInstruction::RES_0_A,
            0xae => PrefixInstruction::RES_5_HL,
            0xfa => PrefixInstruction::SET_7_D,
            0x15 => PrefixInstruction::RL_L,
            0x6c => PrefixInstruction::BIT_5_H,
            0x46 => PrefixInstruction::BIT_0_HL,
            0xb1 => PrefixInstruction::RES_6_C,
            0xbd => PrefixInstruction::RES_7_L,
            0xbc => PrefixInstruction::RES_7_H,
            0x3f => PrefixInstruction::SRL_A,
            0x00 => PrefixInstruction::RLC_B,
            0xb4 => PrefixInstruction::RES_6_H,
            0xb6 => PrefixInstruction::RES_6_HL,
            0xf7 => PrefixInstruction::SET_6_A,
            0x0e => PrefixInstruction::RRC_HL,
            0x95 => PrefixInstruction::RES_2_L,
            0xa1 => PrefixInstruction::RES_4_C,
            0x96 => PrefixInstruction::RES_2_HL,
            0x30 => PrefixInstruction::SWAP_B,
            0x51 => PrefixInstruction::BIT_2_C,
            0x29 => PrefixInstruction::SRA_C,
            0x48 => PrefixInstruction::BIT_1_B,
            0xbb => PrefixInstruction::RES_7_E,
            0xfb => PrefixInstruction::SET_7_E,
            0xb3 => PrefixInstruction::RES_6_E,
            0x58 => PrefixInstruction::BIT_3_B,
            0xf0 => PrefixInstruction::SET_6_B,
            0x84 => PrefixInstruction::RES_0_H,
            0xf1 => PrefixInstruction::SET_6_C,
            0x08 => PrefixInstruction::RRC_B,
            0x2d => PrefixInstruction::SRA_L,
            0x1a => PrefixInstruction::RR_D,
            0x3c => PrefixInstruction::SRL_H,
            0xb8 => PrefixInstruction::RES_7_B,
            0x5e => PrefixInstruction::BIT_3_HL,
            0x4b => PrefixInstruction::BIT_1_E,
            0x63 => PrefixInstruction::BIT_4_E,
            0x9b => PrefixInstruction::RES_3_E,
            0x1c => PrefixInstruction::RR_H,
            0xc1 => PrefixInstruction::SET_0_C,
            0xe3 => PrefixInstruction::SET_4_E,
            0xff => PrefixInstruction::SET_7_A,
            0x14 => PrefixInstruction::RL_H,
            0x66 => PrefixInstruction::BIT_4_HL,
            0xc2 => PrefixInstruction::SET_0_D,
            0x1e => PrefixInstruction::RR_HL,
            0xb2 => PrefixInstruction::RES_6_D,
            0xec => PrefixInstruction::SET_5_H,
            0xfe => PrefixInstruction::SET_7_HL,
            0xd0 => PrefixInstruction::SET_2_B,
            0x77 => PrefixInstruction::BIT_6_A,
            0xb0 => PrefixInstruction::RES_6_B,
            0x39 => PrefixInstruction::SRL_C,
            0xc5 => PrefixInstruction::SET_0_L,
            0x43 => PrefixInstruction::BIT_0_E,
            0xa9 => PrefixInstruction::RES_5_C,
            0xe2 => PrefixInstruction::SET_4_D,
            0x21 => PrefixInstruction::SLA_C,
            0xd2 => PrefixInstruction::SET_2_D,
            0x60 => PrefixInstruction::BIT_4_B,
            0x23 => PrefixInstruction::SLA_E,
            0x40 => PrefixInstruction::BIT_0_B,
            0x5b => PrefixInstruction::BIT_3_E,
            0xa3 => PrefixInstruction::RES_4_E,
            0xcf => PrefixInstruction::SET_1_A,
            0x4e => PrefixInstruction::BIT_1_HL,
            0x0b => PrefixInstruction::RRC_E,
            0x7a => PrefixInstruction::BIT_7_D,
            0xba => PrefixInstruction::RES_7_D,
            0x7e => PrefixInstruction::BIT_7_HL,
            0xf8 => PrefixInstruction::SET_7_B,
            0xdb => PrefixInstruction::SET_3_E,
            0xc6 => PrefixInstruction::SET_0_HL,
            0x50 => PrefixInstruction::BIT_2_B,
            0xf2 => PrefixInstruction::SET_6_D,
            0x64 => PrefixInstruction::BIT_4_H,
            0xce => PrefixInstruction::SET_1_HL,
            0xa2 => PrefixInstruction::RES_4_D,
            0x59 => PrefixInstruction::BIT_3_C,
            0xad => PrefixInstruction::RES_5_L,
            0x26 => PrefixInstruction::SLA_HL,
            0x7c => PrefixInstruction::BIT_7_H,
            0x11 => PrefixInstruction::RL_C,
            0x53 => PrefixInstruction::BIT_2_E,
            0x5c => PrefixInstruction::BIT_3_H,
            0xe8 => PrefixInstruction::SET_5_B,
            0x8b => PrefixInstruction::RES_1_E,
            0x4a => PrefixInstruction::BIT_1_D,
            0xbe => PrefixInstruction::RES_7_HL,
            0x1f => PrefixInstruction::RR_A,
            0xab => PrefixInstruction::RES_5_E,
            0x9a => PrefixInstruction::RES_3_D,
            0x6e => PrefixInstruction::BIT_5_HL,
            0xd7 => PrefixInstruction::SET_2_A,
            0x34 => PrefixInstruction::SWAP_H,
            0x89 => PrefixInstruction::RES_1_C,
            0x56 => PrefixInstruction::BIT_2_HL,
            0xd3 => PrefixInstruction::SET_2_E,
            0xf6 => PrefixInstruction::SET_6_HL,
            0x1d => PrefixInstruction::RR_L,
            0xe4 => PrefixInstruction::SET_4_H,
            0x8d => PrefixInstruction::RES_1_L,
            0xc7 => PrefixInstruction::SET_0_A,
            0x25 => PrefixInstruction::SLA_L,
            0xd4 => PrefixInstruction::SET_2_H,
            0x17 => PrefixInstruction::RL_A,
            0x97 => PrefixInstruction::RES_2_A,
            0x33 => PrefixInstruction::SWAP_E,
            0x99 => PrefixInstruction::RES_3_C,
            0x32 => PrefixInstruction::SWAP_D,
            0x54 => PrefixInstruction::BIT_2_H,
            0xe0 => PrefixInstruction::SET_4_B,
            0x07 => PrefixInstruction::RLC_A,
            0x19 => PrefixInstruction::RR_C,
            0x3d => PrefixInstruction::SRL_L,
            0xda => PrefixInstruction::SET_3_D,
            0xeb => PrefixInstruction::SET_5_E,
            0xe9 => PrefixInstruction::SET_5_C,
            0xac => PrefixInstruction::RES_5_H,
            0x4d => PrefixInstruction::BIT_1_L,
            0x06 => PrefixInstruction::RLC_HL,
            0x72 => PrefixInstruction::BIT_6_D,
            0x52 => PrefixInstruction::BIT_2_D,
            0x16 => PrefixInstruction::RL_HL,
            0xb5 => PrefixInstruction::RES_6_L,
            0x04 => PrefixInstruction::RLC_H,
            0x9d => PrefixInstruction::RES_3_L,
            0x76 => PrefixInstruction::BIT_6_HL,
            0xc8 => PrefixInstruction::SET_1_B,
            0x2a => PrefixInstruction::SRA_D,
            0x38 => PrefixInstruction::SRL_B,
            0x94 => PrefixInstruction::RES_2_H,
            0x3b => PrefixInstruction::SRL_E,
            0x2e => PrefixInstruction::SRA_HL,
            0xee => PrefixInstruction::SET_5_HL,
            0x5d => PrefixInstruction::BIT_3_L,
            0xa8 => PrefixInstruction::RES_5_B,
            _ => panic!("Invalid prefix opcode {:#X}", opcode),
        }
    }

    fn test_bit(&mut self, byte: u8, bit: u8) {
        // Test the appropriate bit in the byte,
        // updating our flags based on the value of the bit.
        //
        // Z: Set if the selected bit is 0.
        // N: 0
        // H: 1

        let mask = masked(byte, bit);
        if mask == 0 {
            self.registers.set_z_flag(true);
        } else {
            self.registers.set_z_flag(false);
        }

        self.registers.set_n_flag(false);
        self.registers.set_h_flag(true);
    }
}

impl<'ppu> InstructionHandler for Cpu<'ppu> {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::LD_SP_N16(x) => self.stack_pointer = x,
            Instruction::LD_A16_SP(x) => {
                self.mmu.set(x, (self.stack_pointer & 0xf0) as u8);
                self.mmu.set(x + 1, (self.stack_pointer & 0xf) as u8);
            }
            Instruction::XOR_ADDR_A_A => self.registers.a = 0,
            Instruction::LD_HL_N16(x) => self.registers.update_hl(x),
            Instruction::LD_HL_DEC_A => {
                // Store the value of register A
                // into the address pointed to by the HL register pair,
                // then decrement HL by 1.
                let mut addr = RegisterPair(self.registers.h, self.registers.l);
                self.mmu.set(addr.as_word(), self.registers.a);
                self.registers.update_hl(addr.as_word() - 1);
            }
            Instruction::PREFIX(prefix_instruction) => match prefix_instruction {
                PrefixInstruction::BIT_7_H => self.test_bit(self.registers.h, 7),
                PrefixInstruction::RL_C => {
                    // Rotate bits in C left, through the carry flag,
                    // and then set the carry flag to the most significant bit
                    // before the shift.
                    let msb = self.registers.c >> 7;
                    self.registers.c = self.registers.c << 1;
                    self.registers.set_z_flag(self.registers.c == 0);
                    self.registers.set_c_flag(msb == 1);
                }
                _ => {
                    panic!("Reached unknown prefix {:?}", prefix_instruction);
                }
            },
            Instruction::JR_NZ_E8(x) => {
                // Jump to x if Z flag isn't set.
                if self.registers.f & 0x80 == 0 {
                    let signed_x = signed(x);
                    self.program_counter =
                        self.program_counter.saturating_add_signed(signed_x as i16);
                }
            }
            Instruction::JR_Z_E8(x) => {
                // Jump to x if Z flag is set.
                if self.registers.f & 0x80 == 128 {
                    let signed_x = signed(x);
                    self.program_counter =
                        self.program_counter.saturating_add_signed(signed_x as i16);
                }
            }
            Instruction::JR_E8(x) => {
                let signed_x = signed(x);
                self.program_counter = self.program_counter.saturating_add_signed(signed_x as i16);
            }
            Instruction::LD_C_N8(x) => self.registers.c = x,
            Instruction::LD_A_N8(x) => self.registers.a = x,
            Instruction::LD_D_N8(x) => self.registers.d = x,
            Instruction::LD_E_N8(x) => self.registers.e = x,
            Instruction::LDH_C_A => {
                // Copy the value in register A
                // into the byte at address $FF00 + C,
                // aka a I/O register.
                self.mmu
                    .set(0xff00 + (self.registers.c as u16), self.registers.a);
            }
            Instruction::INC_ADDR_B => {
                let overflow_add = self.registers.b.overflowing_add(1);

                self.registers.set_z_flag(overflow_add.0 == 0);
                self.registers.set_n_flag(false);
                self.registers.set_h_flag(self.registers.b & 0xf == 0xf);
                self.registers.set_c_flag(false);

                self.registers.b = overflow_add.0;
            }
            Instruction::INC_ADDR_C => {
                let overflow_add = self.registers.c.overflowing_add(1);

                self.registers.set_z_flag(overflow_add.0 == 0);
                self.registers.set_n_flag(false);
                self.registers.set_h_flag(self.registers.c & 0xf == 0xf);
                self.registers.set_c_flag(false);

                self.registers.c = overflow_add.0;
            }
            Instruction::INC_ADDR_H => {
                let overflow_add = self.registers.h.overflowing_add(1);

                self.registers.set_z_flag(overflow_add.0 == 0);
                self.registers.set_n_flag(false);
                self.registers.set_h_flag(self.registers.h & 0xf == 0xf);
                self.registers.set_c_flag(false);

                self.registers.h = overflow_add.0;
            }
            Instruction::LD_HL_A => {
                let hl = RegisterPair(self.registers.h, self.registers.l);
                self.mmu.set(hl.as_word(), self.registers.a);
            }
            Instruction::LD_HL_E => {
                let hl = RegisterPair(self.registers.h, self.registers.l);
                self.mmu.set(hl.as_word(), self.registers.e);
            }
            Instruction::LDH_A8_A(x) => {
                // Copy the value in register A
                // into the byte at address 0xff00 + A8.
                self.mmu.set(0xff00 + (x as u16), self.registers.a);
            }
            Instruction::LD_DE_N16(x) => self.registers.update_de(x),
            Instruction::LD_A_DE => {
                let de = RegisterPair(self.registers.d, self.registers.e);
                self.registers.a = self.mmu.read(de.as_word());
            }
            Instruction::CALL_A16(x) => {
                // Call address A16.
                //
                // This pushes the address of the instruction after the CALL on the stack,
                // such that RET can pop it later.
                self.stack.push(self.program_counter);
                self.stack_pointer -= 1;
                self.program_counter = x;
            }
            Instruction::CALL_Z_A16(x) => {
                // Call address A16, if Z is set.
                if self.registers.f & 0x80 == 0x80 {
                    self.execute(Instruction::CALL_A16(x));
                }
            }
            Instruction::LD_C_A => self.registers.c = self.registers.a,
            Instruction::LD_B_N8(x) => self.registers.b = x,
            Instruction::PUSH_BC => {
                self.stack_pointer -= 1;
                self.stack.push(self.registers.b as u16);
                self.stack_pointer -= 1;
                self.stack.push(self.registers.c as u16);
            }
            Instruction::RLA_ADDR => {
                let msb = self.registers.a >> 7 == 1;
                self.registers.a = self.registers.a << 1;
                self.registers.set_c_flag(msb);
            }
            Instruction::POP_BC => {
                self.registers.c = self.stack.pop().unwrap() as u8;
                self.stack_pointer += 1;
                self.registers.b = self.stack.pop().unwrap() as u8;
                self.stack_pointer += 1;
            }
            Instruction::DEC_ADDR_A => {
                let overflow_sub = self.registers.a.overflowing_sub(1);
                dbg!(self.registers.a, overflow_sub);
                self.registers.a = overflow_sub.0;
                self.registers.set_z_flag(overflow_sub.0 == 0);
                self.registers.set_n_flag(true);
                self.registers.set_h_flag(self.registers.a & 0xf == 0xf);
            }
            Instruction::DEC_ADDR_B => {
                let overflow_sub = self.registers.b.overflowing_sub(1);
                self.registers.b = overflow_sub.0;
                self.registers.set_z_flag(overflow_sub.0 == 0);
                self.registers.set_n_flag(true);
                self.registers.set_h_flag(self.registers.b & 0xf == 0xf);
            }
            Instruction::DEC_ADDR_C => {
                let overflow_sub = self.registers.c.overflowing_sub(1);
                self.registers.c = overflow_sub.0;
                self.registers.set_z_flag(overflow_sub.0 == 0);
                self.registers.set_n_flag(true);
                self.registers.set_h_flag(self.registers.c & 0xf == 0xf);
            }
            Instruction::DEC_ADDR_D => {
                let overflow_sub = self.registers.d.overflowing_sub(1);
                self.registers.d = overflow_sub.0;
                self.registers.set_z_flag(overflow_sub.0 == 0);
                self.registers.set_n_flag(true);
                self.registers.set_h_flag(self.registers.d & 0xf == 0xf);
            }
            Instruction::DEC_ADDR_E => {
                let overflow_sub = self.registers.e.overflowing_sub(1);
                self.registers.e = overflow_sub.0;
                self.registers.set_z_flag(overflow_sub.0 == 0);
                self.registers.set_n_flag(true);
                self.registers.set_h_flag(self.registers.e & 0xf == 0xf);
            }
            Instruction::LD_HL_INC_A => {
                let hl = RegisterPair(self.registers.h, self.registers.l);
                self.mmu.set(hl.as_word(), self.registers.a);
            }
            Instruction::INC_HL => {
                let hl = RegisterPair(self.registers.h, self.registers.l);
                self.registers.update_hl(hl.as_word() + 1);
            }
            Instruction::INC_BC => {
                let bc = RegisterPair(self.registers.b, self.registers.c);
                self.registers.update_bc(bc.as_word() + 1);
            }
            Instruction::INC_DE => {
                let de = RegisterPair(self.registers.d, self.registers.e);
                self.registers.update_de(de.as_word() + 1);
            }
            Instruction::DEC_BC => {
                let bc = RegisterPair(self.registers.b, self.registers.c);
                self.registers.update_bc(bc.as_word() - 1);
            }
            Instruction::RET => {
                // Return from subroutine, aka POP PC.
                self.program_counter = self.stack.pop().unwrap();
                self.stack_pointer += 1;
            }
            Instruction::LD_A_E => self.registers.a = self.registers.e,
            Instruction::LD_A_H => self.registers.a = self.registers.h,
            Instruction::LD_H_A => self.registers.h = self.registers.a,
            Instruction::LD_D_A => self.registers.d = self.registers.a,
            Instruction::CP_ADDR_A_N8(x) => {
                // Compare the value in register A with the value N8.
                //
                // This subtracts the value n8 from A and sets flags accordingly,
                // but discards the result.
                let overflow_sub = self.registers.a.overflowing_sub(x);

                self.registers.set_z_flag(overflow_sub.0 == 0);
                self.registers.set_n_flag(true);
                self.registers.set_h_flag(overflow_sub.0 & 0xf == 0xf);
                self.registers.set_c_flag(overflow_sub.1);
            }
            Instruction::LD_A16_A(x) => self.mmu.set(x, self.registers.a),
            Instruction::LD_L_N8(x) => self.registers.l = x,
            Instruction::LDH_A_A8(x) => self.registers.a = self.mmu.read(0xff00 + (x as u16)),
            Instruction::NOP => {}
            Instruction::SUB_ADDR_A_B => {
                let overflow_sub = self.registers.a.overflowing_sub(self.registers.b);

                self.registers.set_z_flag(overflow_sub.0 == 0);
                self.registers.set_n_flag(true);
                self.registers.set_h_flag(overflow_sub.0 & 0xf == 0xf);
                self.registers.set_c_flag(overflow_sub.1);
            }
            Instruction::LD_H_HL => {
                let hl = RegisterPair(self.registers.h, self.registers.l);
                self.registers.h = self.mmu.read(hl.as_word());
            }
            Instruction::ADC_ADDR_A_B => {
                // ADd the value in B plus the carry flag to A.
                let carry = (self.registers.f & 0xf0 != 0) as u8;
                let half_carry =
                    ((self.registers.a & 0xf) + (self.registers.b & 0xf) + carry) > 0xf;

                let overflow_carry = self.registers.b.overflowing_add(carry);
                let overflow_add = self.registers.a.overflowing_add(overflow_carry.0);

                self.registers.set_z_flag(overflow_add.0 == 0);
                self.registers.set_n_flag(false);
                self.registers.set_h_flag(half_carry);
                self.registers
                    .set_c_flag(overflow_carry.1 || overflow_add.1);
            }
            Instruction::ADC_ADDR_A_C => {
                let carry = (self.registers.f & 0xf0 != 0) as u8;
                let half_carry = ((self.registers.a & 0xf) + (self.registers.c & 0xf) + 1) > 0xf;

                let overflow_carry = self.registers.c.overflowing_add(carry);
                let overflow_add = self.registers.a.overflowing_add(overflow_carry.0);

                self.registers.set_z_flag(overflow_add.0 == 0);
                self.registers.set_n_flag(false);
                self.registers.set_h_flag(half_carry);
                self.registers
                    .set_c_flag(overflow_carry.1 || overflow_add.1);
            }
            Instruction::ADC_ADDR_A_E => {
                let carry = (self.registers.f & 0xf0 != 0) as u8;
                let half_carry = ((self.registers.a & 0xf) + (self.registers.e & 0xf) + 1) > 0xf;

                let overflow_carry = self.registers.e.overflowing_add(carry);
                let overflow_add = self.registers.a.overflowing_add(overflow_carry.0);

                self.registers.set_z_flag(overflow_add.0 == 0);
                self.registers.set_n_flag(false);
                self.registers.set_h_flag(half_carry);
                self.registers
                    .set_c_flag(overflow_carry.1 || overflow_add.1);
            }
            Instruction::ADC_ADDR_A_N8(x) => {
                let carry = (self.registers.f & 0xf0 != 0) as u8;
                let half_carry = ((self.registers.a & 0xf) + (x & 0xf) + 1) > 0xf;

                let overflow_carry = x.overflowing_add(carry);
                let overflow_add = self.registers.a.overflowing_add(overflow_carry.0);

                self.registers.set_z_flag(overflow_add.0 == 0);
                self.registers.set_n_flag(false);
                self.registers.set_h_flag(half_carry);
                self.registers
                    .set_c_flag(overflow_carry.1 || overflow_add.1);
            }
            _ => {
                panic!("Reached unknown instruction {:?}", instruction);
            }
        }
    }
}

struct Registers {
    // Accumulator register.
    a: u8,

    // Flags register.
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
}

#[derive(Debug)]
struct Flags {
    // Lower eight bits of the F/AF register;
    // bits 3 - 0 are ignored.

    // Zero flag.
    z: bool,

    // Subtraction flag.
    n: bool,

    // Half carry flag.
    h: bool,

    // Carry flag.
    c: bool,
}

impl Flags {
    fn from(byte: u8) -> Self {
        Self {
            z: byte & 0x80 == 1,
            n: byte & 0x40 == 1,
            h: byte & 0x20 == 1,
            c: byte & 0x10 == 1,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct RegisterPair(u8, u8);

impl RegisterPair {
    fn from(word: u16) -> Self {
        Self((word >> 8) as u8, word as u8)
    }

    fn as_word(&self) -> u16 {
        ((self.0 as u16) << 8) | self.1 as u16
    }
}

impl Sub for RegisterPair {
    type Output = RegisterPair;

    fn sub(self, other: Self) -> RegisterPair {
        let difference = self.as_word() - other.as_word();
        RegisterPair::from(difference)
    }
}

impl Registers {
    fn empty() -> Self {
        Self {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
        }
    }

    fn set_z_flag(&mut self, z: bool) {
        self.f = self.f & !(1 << 7) | ((z as u8) << 7);
    }

    fn set_n_flag(&mut self, n: bool) {
        self.f = self.f & !(1 << 6) | ((n as u8) << 6);
    }

    fn set_h_flag(&mut self, h: bool) {
        self.f = self.f & !(1 << 5) | ((h as u8) << 5);
    }

    fn set_c_flag(&mut self, c: bool) {
        self.f = self.f & !(1 << 4) | ((c as u8) << 4);
    }

    fn update_bc(&mut self, word: u16) {
        let bc = RegisterPair::from(word);
        self.b = bc.0;
        self.c = bc.1;
    }

    fn update_de(&mut self, word: u16) {
        let de = RegisterPair::from(word);
        self.d = de.0;
        self.e = de.1;
    }

    fn update_hl(&mut self, word: u16) {
        let hl = RegisterPair::from(word);
        self.h = hl.0;
        self.l = hl.1;
    }
}
