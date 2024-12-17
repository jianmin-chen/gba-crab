#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Instruction {
    SBC_ADDR_A_B,
    OR_ADDR_A_A,
    LD_HL_DEC_A,
    ADD_ADDR_HL_BC,
    JP_A16(u16),
    ADC_ADDR_A_D,
    LD_H_A,
    LD_B_HL,
    RST_30,
    LD_C_H,
    ADD_ADDR_A_E,
    CP_ADDR_A_E,
    LD_SP_N16(u16),
    INC_ADDR_H,
    SBC_ADDR_A_H,
    XOR_ADDR_A_L,
    LD_D_N8(u8),
    RLCA_ADDR,
    LD_E_L,
    LD_E_C,
    ADD_ADDR_A_C,
    RST_38,
    SUB_ADDR_A_D,
    RRCA_ADDR,
    SUB_ADDR_A_B,
    CALL_C_A16(u16),
    LDH_C_A,
    STOP_N8(u8),
    LD_H_H,
    PREFIX(PrefixInstruction),
    XOR_ADDR_A_D,
    RET_Z,
    PUSH_HL,
    CP_ADDR_A_C,
    OR_ADDR_A_HL,
    LD_A_BC,
    DEC_ADDR_H,
    INC_DE,
    RRA_ADDR,
    LD_D_B,
    LD_ADDR_HL_SP_INC_E8(u8),
    INC_ADDR_L,
    INC_ADDR_D,
    LD_L_B,
    ADC_ADDR_A_B,
    AND_ADDR_A_D,
    AND_ADDR_A_N8(u8),
    XOR_ADDR_A_N8(u8),
    CP_ADDR_A_N8(u8),
    LD_HL_L,
    XOR_ADDR_A_HL,
    LD_H_C,
    DEC_ADDR_D,
    INC_HL,
    JR_C_E8(u8),
    LDH_A8_A(u8),
    ADC_ADDR_A_A,
    LD_C_B,
    SUB_ADDR_A_C,
    LD_A_C,
    LD_A_HL,
    SUB_ADDR_A_N8(u8),
    JR_NZ_E8(u8),
    LD_C_N8(u8),
    LD_L_E,
    JR_E8(u8),
    SBC_ADDR_A_HL,
    DEC_ADDR_B,
    LD_B_H,
    LD_HL_N16(u16),
    LD_B_A,
    SBC_ADDR_A_A,
    PUSH_DE,
    SUB_ADDR_A_E,
    LD_C_L,
    SCF_ADDR,
    LD_L_H,
    SUB_ADDR_A_HL,
    SBC_ADDR_A_D,
    LD_A_H,
    OR_ADDR_A_H,
    ADD_ADDR_A_B,
    ADD_ADDR_HL_HL,
    LD_C_C,
    DEC_ADDR_A,
    LD_H_E,
    LD_B_C,
    LD_E_A,
    POP_HL,
    LD_B_N8(u8),
    LD_A_N8(u8),
    LD_L_C,
    ADD_ADDR_A_D,
    RST_08,
    LD_C_E,
    INC_ADDR_B,
    DEC_ADDR_E,
    LD_D_L,
    RET_NC,
    LD_E_H,
    LD_A16_A(u16),
    LD_HL_N8(u8),
    LD_C_A,
    AND_ADDR_A_A,
    LD_D_HL,
    OR_ADDR_A_N8(u8),
    OR_ADDR_A_L,
    RETI,
    LD_SP_HL,
    JP_HL,
    DEC_SP,
    LD_B_L,
    CP_ADDR_A_H,
    XOR_ADDR_A_C,
    LD_D_E,
    SUB_ADDR_A_A,
    JP_Z_A16(u16),
    DEC_DE,
    LD_HL_INC_A,
    JP_NZ_A16(u16),
    LD_L_N8(u8),
    LD_HL_D,
    INC_SP,
    DEC_BC,
    CALL_NC_A16(u16),
    LDH_A_A8(u8),
    RST_10,
    LD_D_D,
    LD_H_HL,
    LD_H_D,
    LD_HL_A,
    AND_ADDR_A_L,
    DEC_ADDR_HL,
    LD_C_D,
    SUB_ADDR_A_H,
    OR_ADDR_A_D,
    ADC_ADDR_A_E,
    XOR_ADDR_A_H,
    LD_A_HL_DEC,
    DEC_ADDR_C,
    SBC_ADDR_A_L,
    SBC_ADDR_A_N8(u8),
    LD_L_D,
    LD_L_L,
    NOP,
    LD_BC_A,
    INC_BC,
    LD_H_B,
    DEC_ADDR_L,
    LD_HL_B,
    SUB_ADDR_A_L,
    OR_ADDR_A_E,
    RLA_ADDR,
    AND_ADDR_A_H,
    LD_A16_SP(u16),
    LD_E_N8(u8),
    LD_E_D,
    CALL_A16(u16),
    LD_A_A,
    RET_NZ,
    LD_H_L,
    LD_B_D,
    LD_A_B,
    DAA_ADDR,
    CP_ADDR_A_L,
    PUSH_AF,
    INC_ADDR_A,
    CALL_Z_A16(u16),
    ADD_ADDR_A_HL,
    ADD_ADDR_A_N8(u8),
    LD_A_L,
    RET,
    ADC_ADDR_A_N8(u8),
    LD_L_A,
    POP_ADDR_AF,
    INC_ADDR_E,
    LD_H_N8(u8),
    ADD_ADDR_A_L,
    AND_ADDR_A_E,
    JR_NC_E8(u8),
    RST_00,
    LDH_A_C,
    LD_E_B,
    CP_ADDR_A_B,
    LD_DE_N16(u16),
    INC_ADDR_HL,
    LD_L_HL,
    DI,
    JR_Z_E8(u8),
    ADC_ADDR_A_C,
    INC_ADDR_C,
    DEC_HL,
    ADD_ADDR_HL_DE,
    POP_BC,
    SBC_ADDR_A_E,
    JP_NC_A16(u16),
    LD_HL_E,
    LD_HL_C,
    LD_A_E,
    RST_28,
    SBC_ADDR_A_C,
    ADC_ADDR_A_L,
    ADD_ADDR_HL_SP,
    ADD_ADDR_A_H,
    LD_B_E,
    XOR_ADDR_A_E,
    OR_ADDR_A_C,
    POP_DE,
    CCF_ADDR,
    ADC_ADDR_A_H,
    CPL_ADDR,
    LD_A_D,
    LD_D_C,
    AND_ADDR_A_HL,
    RET_C,
    LD_D_A,
    LD_E_E,
    ADC_ADDR_A_HL,
    LD_A_HL_INC,
    LD_BC_N16(u16),
    AND_ADDR_A_C,
    LD_C_HL,
    LD_E_HL,
    HALT,
    CP_ADDR_A_D,
    AND_ADDR_A_B,
    EI,
    LD_B_B,
    CALL_NZ_A16(u16),
    PUSH_BC,
    LD_DE_A,
    LD_D_H,
    JP_C_A16(u16),
    RST_20,
    XOR_ADDR_A_B,
    LD_A_DE,
    LD_HL_H,
    CP_ADDR_A_A,
    OR_ADDR_A_B,
    ADD_ADDR_SP_E8(u8),
    XOR_ADDR_A_A,
    LD_A_A16(u16),
    ADD_ADDR_A_A,
    RST_18,
    CP_ADDR_A_HL,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum PrefixInstruction {
    BIT_6_C,
    BIT_7_B,
    BIT_7_A,
    RL_E,
    RES_1_B,
    SET_0_B,
    SET_6_L,
    RES_3_HL,
    SLA_H,
    BIT_2_A,
    RLC_E,
    RL_B,
    RRC_H,
    SET_4_HL,
    RES_6_A,
    RES_0_C,
    SLA_B,
    SET_3_B,
    SET_5_A,
    BIT_0_C,
    BIT_7_L,
    RRC_A,
    BIT_6_L,
    SET_0_H,
    SET_3_H,
    RES_1_HL,
    BIT_4_D,
    SET_3_L,
    SET_5_L,
    BIT_5_C,
    BIT_0_A,
    RES_1_D,
    BIT_0_D,
    RES_4_HL,
    SET_1_H,
    RLC_D,
    SET_2_C,
    RES_0_E,
    SET_6_E,
    SET_4_C,
    SWAP_A,
    RES_0_L,
    RRC_L,
    RES_0_HL,
    RES_0_B,
    RES_7_C,
    BIT_5_B,
    SLA_A,
    RR_B,
    BIT_4_L,
    SRA_H,
    RLC_L,
    RRC_D,
    RL_D,
    BIT_5_D,
    RRC_C,
    RLC_C,
    SET_7_C,
    RES_2_B,
    BIT_4_C,
    RES_4_A,
    RES_2_C,
    RES_5_A,
    SRA_E,
    BIT_6_E,
    SET_4_A,
    RES_1_H,
    SET_1_C,
    SET_6_H,
    RR_E,
    RES_4_L,
    SRL_D,
    RES_2_D,
    SET_3_A,
    SWAP_L,
    BIT_5_E,
    RES_4_H,
    RES_5_D,
    BIT_4_A,
    SET_3_C,
    RES_7_A,
    SET_2_L,
    BIT_3_D,
    SLA_D,
    SWAP_C,
    RES_3_H,
    BIT_6_H,
    RES_0_D,
    SET_1_E,
    BIT_1_A,
    SET_0_E,
    RES_2_E,
    RES_1_A,
    SET_4_L,
    SWAP_HL,
    SET_2_HL,
    SRA_A,
    SRL_HL,
    RES_3_B,
    BIT_3_A,
    SET_5_D,
    BIT_1_H,
    RES_4_B,
    BIT_6_B,
    SET_3_HL,
    BIT_7_C,
    BIT_7_E,
    SET_7_L,
    SET_1_L,
    RES_3_A,
    BIT_0_H,
    BIT_5_A,
    SET_1_D,
    BIT_1_C,
    SET_7_H,
    BIT_5_L,
    BIT_0_L,
    SRA_B,
    BIT_2_L,
    RES_0_A,
    RES_5_HL,
    SET_7_D,
    RL_L,
    BIT_5_H,
    BIT_0_HL,
    RES_6_C,
    RES_7_L,
    RES_7_H,
    SRL_A,
    RLC_B,
    RES_6_H,
    RES_6_HL,
    SET_6_A,
    RRC_HL,
    RES_2_L,
    RES_4_C,
    RES_2_HL,
    SWAP_B,
    BIT_2_C,
    SRA_C,
    BIT_1_B,
    RES_7_E,
    SET_7_E,
    RES_6_E,
    BIT_3_B,
    SET_6_B,
    RES_0_H,
    SET_6_C,
    RRC_B,
    SRA_L,
    RR_D,
    SRL_H,
    RES_7_B,
    BIT_3_HL,
    BIT_1_E,
    BIT_4_E,
    RES_3_E,
    RR_H,
    SET_0_C,
    SET_4_E,
    SET_7_A,
    RL_H,
    BIT_4_HL,
    SET_0_D,
    RR_HL,
    RES_6_D,
    SET_5_H,
    SET_7_HL,
    SET_2_B,
    BIT_6_A,
    RES_6_B,
    SRL_C,
    SET_0_L,
    BIT_0_E,
    RES_5_C,
    SET_4_D,
    SLA_C,
    SET_2_D,
    BIT_4_B,
    SLA_E,
    BIT_0_B,
    BIT_3_E,
    RES_4_E,
    SET_1_A,
    BIT_1_HL,
    RRC_E,
    BIT_7_D,
    RES_7_D,
    BIT_7_HL,
    SET_7_B,
    SET_3_E,
    SET_0_HL,
    BIT_2_B,
    SET_6_D,
    BIT_4_H,
    SET_1_HL,
    RES_4_D,
    BIT_3_C,
    RES_5_L,
    SLA_HL,
    BIT_7_H,
    RL_C,
    BIT_2_E,
    BIT_3_H,
    SET_5_B,
    RES_1_E,
    BIT_1_D,
    RES_7_HL,
    RR_A,
    RES_5_E,
    RES_3_D,
    BIT_5_HL,
    SET_2_A,
    SWAP_H,
    RES_1_C,
    BIT_2_HL,
    SET_2_E,
    SET_6_HL,
    RR_L,
    SET_4_H,
    RES_1_L,
    SET_0_A,
    SLA_L,
    SET_2_H,
    RL_A,
    RES_2_A,
    SWAP_E,
    RES_3_C,
    SWAP_D,
    BIT_2_H,
    SET_4_B,
    RLC_A,
    RR_C,
    SRL_L,
    SET_3_D,
    SET_5_E,
    SET_5_C,
    RES_5_H,
    BIT_1_L,
    RLC_HL,
    BIT_6_D,
    BIT_2_D,
    RL_HL,
    RES_6_L,
    RLC_H,
    RES_3_L,
    BIT_6_HL,
    SET_1_B,
    SRA_D,
    SRL_B,
    RES_2_H,
    SRL_E,
    SRA_HL,
    SET_5_HL,
    BIT_3_L,
    RES_5_B,
}