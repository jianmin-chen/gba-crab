0xf6 => PrefixInstruction::SET_6_HL,
0x37 => PrefixInstruction::SWAP_A,
0x00 => PrefixInstruction::RLC_B,
0x7c => PrefixInstruction::BIT_7_H,
0x4a => PrefixInstruction::BIT_1_D,
0x71 => PrefixInstruction::BIT_6_C,
0x4f => PrefixInstruction::BIT_1_A,
0x26 => PrefixInstruction::SLA_HL,
0xdd => PrefixInstruction::SET_3_L,
0x2d => PrefixInstruction::SRA_L,
0x12 => PrefixInstruction::RL_D,
0x3f => PrefixInstruction::SRL_A,
0xd2 => PrefixInstruction::SET_2_D,
0x0e => PrefixInstruction::RRC_HL,
0xf9 => PrefixInstruction::SET_7_C,
0x65 => PrefixInstruction::BIT_4_L,
0x75 => PrefixInstruction::BIT_6_L,
0xda => PrefixInstruction::SET_3_D,
0x33 => PrefixInstruction::SWAP_E,
0x3c => PrefixInstruction::SRL_H,
0x7e => PrefixInstruction::BIT_7_HL,
0x9d => PrefixInstruction::RES_3_L,
0x93 => PrefixInstruction::RES_2_E,
0x40 => PrefixInstruction::BIT_0_B,
0x8f => PrefixInstruction::RES_1_A,
0xe3 => PrefixInstruction::SET_4_E,
0xb0 => PrefixInstruction::RES_6_B,
0xf2 => PrefixInstruction::SET_6_D,
0xd0 => PrefixInstruction::SET_2_B,
0x45 => PrefixInstruction::BIT_0_L,
0xd6 => PrefixInstruction::SET_2_HL,
0xa5 => PrefixInstruction::RES_4_L,
0x5f => PrefixInstruction::BIT_3_A,
0xcd => PrefixInstruction::SET_1_L,
0xc7 => PrefixInstruction::SET_0_A,
0xb8 => PrefixInstruction::RES_7_B,
0xd4 => PrefixInstruction::SET_2_H,
0x21 => PrefixInstruction::SLA_C,
0x0a => PrefixInstruction::RRC_D,
0xde => PrefixInstruction::SET_3_HL,
0x5d => PrefixInstruction::BIT_3_L,
0xc9 => PrefixInstruction::SET_1_C,
0x0d => PrefixInstruction::RRC_L,
0x68 => PrefixInstruction::BIT_5_B,
0xc2 => PrefixInstruction::SET_0_D,
0x48 => PrefixInstruction::BIT_1_B,
0x9b => PrefixInstruction::RES_3_E,
0x1f => PrefixInstruction::RR_A,
0x80 => PrefixInstruction::RES_0_B,
0xc1 => PrefixInstruction::SET_0_C,
0xe8 => PrefixInstruction::SET_5_B,
0xcf => PrefixInstruction::SET_1_A,
0x22 => PrefixInstruction::SLA_D,
0x28 => PrefixInstruction::SRA_B,
0x9e => PrefixInstruction::RES_3_HL,
0xb5 => PrefixInstruction::RES_6_L,
0xba => PrefixInstruction::RES_7_D,
0x2a => PrefixInstruction::SRA_D,
0xf5 => PrefixInstruction::SET_6_L,
0xf7 => PrefixInstruction::SET_6_A,
0xa8 => PrefixInstruction::RES_5_B,
0x06 => PrefixInstruction::RLC_HL,
0x24 => PrefixInstruction::SLA_H,
0x60 => PrefixInstruction::BIT_4_B,
0xce => PrefixInstruction::SET_1_HL,
0xe7 => PrefixInstruction::SET_4_A,
0x61 => PrefixInstruction::BIT_4_C,
0xfe => PrefixInstruction::SET_7_HL,
0x95 => PrefixInstruction::RES_2_L,
0xca => PrefixInstruction::SET_1_D,
0xcc => PrefixInstruction::SET_1_H,
0x38 => PrefixInstruction::SRL_B,
0x6e => PrefixInstruction::BIT_5_HL,
0x76 => PrefixInstruction::BIT_6_HL,
0xa1 => PrefixInstruction::RES_4_C,
0xc0 => PrefixInstruction::SET_0_B,
0xaa => PrefixInstruction::RES_5_D,
0xf0 => PrefixInstruction::SET_6_B,
0x8e => PrefixInstruction::RES_1_HL,
0x18 => PrefixInstruction::RR_B,
0x3a => PrefixInstruction::SRL_D,
0x4b => PrefixInstruction::BIT_1_E,
0x6f => PrefixInstruction::BIT_5_A,
0x96 => PrefixInstruction::RES_2_HL,
0x98 => PrefixInstruction::RES_3_B,
0x19 => PrefixInstruction::RR_C,
0x70 => PrefixInstruction::BIT_6_B,
0xb1 => PrefixInstruction::RES_6_C,
0x42 => PrefixInstruction::BIT_0_D,
0x09 => PrefixInstruction::RRC_C,
0x85 => PrefixInstruction::RES_0_L,
0x29 => PrefixInstruction::SRA_C,
0x35 => PrefixInstruction::SWAP_L,
0x53 => PrefixInstruction::BIT_2_E,
0x04 => PrefixInstruction::RLC_H,
0x0b => PrefixInstruction::RRC_E,
0x1e => PrefixInstruction::RR_HL,
0xa4 => PrefixInstruction::RES_4_H,
0x1a => PrefixInstruction::RR_D,
0x67 => PrefixInstruction::BIT_4_A,
0xcb => PrefixInstruction::SET_1_E,
0xd3 => PrefixInstruction::SET_2_E,
0x10 => PrefixInstruction::RL_B,
0x7f => PrefixInstruction::BIT_7_A,
0x2c => PrefixInstruction::SRA_H,
0xd8 => PrefixInstruction::SET_3_B,
0xe4 => PrefixInstruction::SET_4_H,
0xea => PrefixInstruction::SET_5_D,
0xed => PrefixInstruction::SET_5_L,
0xc5 => PrefixInstruction::SET_0_L,
0x25 => PrefixInstruction::SLA_L,
0x3b => PrefixInstruction::SRL_E,
0xae => PrefixInstruction::RES_5_HL,
0xb6 => PrefixInstruction::RES_6_HL,
0xc3 => PrefixInstruction::SET_0_E,
0x7a => PrefixInstruction::BIT_7_D,
0x79 => PrefixInstruction::BIT_7_C,
0xa9 => PrefixInstruction::RES_5_C,
0xac => PrefixInstruction::RES_5_H,
0xf8 => PrefixInstruction::SET_7_B,
0xdc => PrefixInstruction::SET_3_H,
0xc4 => PrefixInstruction::SET_0_H,
0x8a => PrefixInstruction::RES_1_D,
0x9c => PrefixInstruction::RES_3_H,
0x1d => PrefixInstruction::RR_L,
0xb2 => PrefixInstruction::RES_6_D,
0x89 => PrefixInstruction::RES_1_C,
0xd7 => PrefixInstruction::SET_2_A,
0x50 => PrefixInstruction::BIT_2_B,
0x23 => PrefixInstruction::SLA_E,
0x11 => PrefixInstruction::RL_C,
0x17 => PrefixInstruction::RL_A,
0xa7 => PrefixInstruction::RES_4_A,
0x32 => PrefixInstruction::SWAP_D,
0x34 => PrefixInstruction::SWAP_H,
0xad => PrefixInstruction::RES_5_L,
0xb7 => PrefixInstruction::RES_6_A,
0xbf => PrefixInstruction::RES_7_A,
0x5b => PrefixInstruction::BIT_3_E,
0x41 => PrefixInstruction::BIT_0_C,
0x6c => PrefixInstruction::BIT_5_H,
0xe1 => PrefixInstruction::SET_4_C,
0xee => PrefixInstruction::SET_5_HL,
0x63 => PrefixInstruction::BIT_4_E,
0xb3 => PrefixInstruction::RES_6_E,
0xef => PrefixInstruction::SET_5_A,
0x6d => PrefixInstruction::BIT_5_L,
0x07 => PrefixInstruction::RLC_A,
0x73 => PrefixInstruction::BIT_6_E,
0x57 => PrefixInstruction::BIT_2_A,
0xd9 => PrefixInstruction::SET_3_C,
0x8d => PrefixInstruction::RES_1_L,
0x66 => PrefixInstruction::BIT_4_HL,
0x05 => PrefixInstruction::RLC_L,
0x0f => PrefixInstruction::RRC_A,
0x8b => PrefixInstruction::RES_1_E,
0x72 => PrefixInstruction::BIT_6_D,
0xa3 => PrefixInstruction::RES_4_E,
0xdf => PrefixInstruction::SET_3_A,
0x51 => PrefixInstruction::BIT_2_C,
0x74 => PrefixInstruction::BIT_6_H,
0x1b => PrefixInstruction::RR_E,
0xa2 => PrefixInstruction::RES_4_D,
0xf3 => PrefixInstruction::SET_6_E,
0x30 => PrefixInstruction::SWAP_B,
0x14 => PrefixInstruction::RL_H,
0x56 => PrefixInstruction::BIT_2_HL,
0x13 => PrefixInstruction::RL_E,
0x2b => PrefixInstruction::SRA_E,
0x83 => PrefixInstruction::RES_0_E,
0x16 => PrefixInstruction::RL_HL,
0xdb => PrefixInstruction::SET_3_E,
0x97 => PrefixInstruction::RES_2_A,
0xa6 => PrefixInstruction::RES_4_HL,
0xe6 => PrefixInstruction::SET_4_HL,
0xeb => PrefixInstruction::SET_5_E,
0xfc => PrefixInstruction::SET_7_H,
0x52 => PrefixInstruction::BIT_2_D,
0x88 => PrefixInstruction::RES_1_B,
0x7d => PrefixInstruction::BIT_7_L,
0x90 => PrefixInstruction::RES_2_B,
0x92 => PrefixInstruction::RES_2_D,
0xb9 => PrefixInstruction::RES_7_C,
0x2f => PrefixInstruction::SRA_A,
0x58 => PrefixInstruction::BIT_3_B,
0x5c => PrefixInstruction::BIT_3_H,
0x39 => PrefixInstruction::SRL_C,
0xa0 => PrefixInstruction::RES_4_B,
0xab => PrefixInstruction::RES_5_E,
0xfd => PrefixInstruction::SET_7_L,
0xc6 => PrefixInstruction::SET_0_HL,
0x82 => PrefixInstruction::RES_0_D,
0xbc => PrefixInstruction::RES_7_H,
0x69 => PrefixInstruction::BIT_5_C,
0x6a => PrefixInstruction::BIT_5_D,
0xf1 => PrefixInstruction::SET_6_C,
0xff => PrefixInstruction::SET_7_A,
0x36 => PrefixInstruction::SWAP_HL,
0x54 => PrefixInstruction::BIT_2_H,
0x6b => PrefixInstruction::BIT_5_E,
0xec => PrefixInstruction::SET_5_H,
0x3d => PrefixInstruction::SRL_L,
0x84 => PrefixInstruction::RES_0_H,
0x1c => PrefixInstruction::RR_H,
0x47 => PrefixInstruction::BIT_0_A,
0x81 => PrefixInstruction::RES_0_C,
0x55 => PrefixInstruction::BIT_2_L,
0x91 => PrefixInstruction::RES_2_C,
0x78 => PrefixInstruction::BIT_7_B,
0xbe => PrefixInstruction::RES_7_HL,
0x02 => PrefixInstruction::RLC_D,
0x43 => PrefixInstruction::BIT_0_E,
0x64 => PrefixInstruction::BIT_4_H,
0x86 => PrefixInstruction::RES_0_HL,
0x3e => PrefixInstruction::SRL_HL,
0x20 => PrefixInstruction::SLA_B,
0x03 => PrefixInstruction::RLC_E,
0x46 => PrefixInstruction::BIT_0_HL,
0xfb => PrefixInstruction::SET_7_E,
0x15 => PrefixInstruction::RL_L,
0x08 => PrefixInstruction::RRC_B,
0x7b => PrefixInstruction::BIT_7_E,
0xe5 => PrefixInstruction::SET_4_L,
0x59 => PrefixInstruction::BIT_3_C,
0x87 => PrefixInstruction::RES_0_A,
0xd5 => PrefixInstruction::SET_2_L,
0xf4 => PrefixInstruction::SET_6_H,
0x5a => PrefixInstruction::BIT_3_D,
0xe0 => PrefixInstruction::SET_4_B,
0x27 => PrefixInstruction::SLA_A,
0x8c => PrefixInstruction::RES_1_H,
0xfa => PrefixInstruction::SET_7_D,
0x4e => PrefixInstruction::BIT_1_HL,
0x49 => PrefixInstruction::BIT_1_C,
0x31 => PrefixInstruction::SWAP_C,
0x77 => PrefixInstruction::BIT_6_A,
0xe2 => PrefixInstruction::SET_4_D,
0x99 => PrefixInstruction::RES_3_C,
0x4c => PrefixInstruction::BIT_1_H,
0x0c => PrefixInstruction::RRC_H,
0x94 => PrefixInstruction::RES_2_H,
0xb4 => PrefixInstruction::RES_6_H,
0x01 => PrefixInstruction::RLC_C,
0x9f => PrefixInstruction::RES_3_A,
0xbd => PrefixInstruction::RES_7_L,
0x62 => PrefixInstruction::BIT_4_D,
0x5e => PrefixInstruction::BIT_3_HL,
0xd1 => PrefixInstruction::SET_2_C,
0x9a => PrefixInstruction::RES_3_D,
0x2e => PrefixInstruction::SRA_HL,
0xc8 => PrefixInstruction::SET_1_B,
0xe9 => PrefixInstruction::SET_5_C,
0x4d => PrefixInstruction::BIT_1_L,
0xbb => PrefixInstruction::RES_7_E,
0xaf => PrefixInstruction::RES_5_A,
0x44 => PrefixInstruction::BIT_0_H,
