#[derive(Debug, Clone, Copy)]
struct InstAddr{
    data: u16
}

impl Default for InstAddr{
    fn default() -> Self{
        InstAddr{data: 0}
    }
}
impl InstAddr {
    pub fn from_fields(funct3: u8, funct7: bool, high_bit: bool, low_nibble: u8) -> Self {
        InstAddr{
            data:  ((funct3 as u16) << 6) | ((funct7 as u16) << 5) | ((high_bit as u16) << 4) | (low_nibble as u16)
        }
    }

    pub fn from_fields_b(funct3: u8, funct7: bool, opcode: u8) -> Self{
        InstAddr{
            data:  ((funct3 as u16) << 6) | ((funct7 as u16) << 5) | (opcode as u16)
        }  
    }

    pub fn get_low_nibble(&self) -> u16 {
        ( self.data & 0b000001111 ) >> 0
    }

    pub fn get_high_bit(&self) -> u16 {
        (self.data & 0b000010000) >> 4
    }

    pub fn get_opcode(&self) -> u16 {
        (self.get_high_bit() << 4) | self.get_low_nibble()
    }

    pub fn get_funct7_state(&self) -> u16 {
        ((self.data & 0b000100000) >> 5)
    }

    pub fn get_funct3(&self) -> u16 {
        (self.data & 0b111000000) >> 6
    }

}

type ControlLine = u32;


    const PC_ENC: ControlLine       = 1 << 0;
    const PC_LOAD: ControlLine      = 1 << 1;
    const PC_OUT: ControlLine       = 1 << 2;

    const ALU_INVA: ControlLine     = 1 << 3;
    const ALU_INVB: ControlLine     = 1 << 4;
    const ALU_OP_B2: ControlLine     = 1 << 5;
    const ALU_OP_B1: ControlLine     = 1 << 6;
    const ALU_OP_B0: ControlLine     = 1 << 7;
    
    const ALU_OUTMOD_B1: ControlLine = 1 << 8;
    const ALU_OUTMOD_B0: ControlLine = 1 << 9;
    const ALU_INVOUT: ControlLine   = 1 << 10;
    const ALU_OUT: ControlLine      = 1 << 11;

    const REG_OUT: ControlLine      = 1 << 12;

    const CNTRL_IMMOUT: ControlLine = 1 << 13;

    const IR_ZR: ControlLine        = 1 << 14;
    const IR_CEN: ControlLine       = 1 << 15;
    const IR_LOAD: ControlLine      = 1 << 16;

    const RAM_ENO: ControlLine      = 1 << 17;
    const RAM_LOAD: ControlLine     = 1 << 18;
    
    const ALU_OP_ADD:   ControlLine  =         0 |         0 |         0;
    const ALU_OP_AND:   ControlLine  =         0 |         0 | ALU_OP_B0;
    const ALU_OP_XOR:   ControlLine  =         0 | ALU_OP_B1 |         0;
    const ALU_OP_LLSHFT: ControlLine =         0 | ALU_OP_B1 | ALU_OP_B0;
    const ALU_OP_RSHFT: ControlLine  = ALU_OP_B2 |         0 |         0;
    const ALU_OP_SUB:   ControlLine  = ALU_OP_B2 |         0 | ALU_OP_B0;
    const ALU_OP_6:     ControlLine  = ALU_OP_B2 | ALU_OP_B1 |         0;
    const ALU_OP_7:     ControlLine  = ALU_OP_B2 | ALU_OP_B1 | ALU_OP_B0;

    const ALU_OUTMOD_NONE: ControlLine    =             0 |             0;
    const ALU_OUTMOD_SIGNBIT: ControlLine =             0 | ALU_OUTMOD_B0;
    const ALU_OUTMOD_SUBC: ControlLine    = ALU_OUTMOD_B1 |             0;
    const ALU_OUTMOD_3: ControlLine       = ALU_OUTMOD_B1 | ALU_OUTMOD_B0;

#[derive(Debug)]
struct Rom {
    data: [ControlLine; 0b1_000_000_000] // 9 bits of address
}
impl Rom {
    pub fn new() -> Self{
        Rom{
            data: [Default::default(); 0b1_000_000_000]
        }
    }
}
use std::ops::{Index, IndexMut};

impl Index<InstAddr> for Rom {
    type Output = ControlLine;
    fn index(&self, ind: InstAddr) -> &Self::Output {
        &self.data[ind.data as usize]
    }
}

impl IndexMut<InstAddr> for Rom {
    fn index_mut(&mut self, ind: InstAddr) -> &mut Self::Output {
        &mut self.data[ind.data as usize]
    }
}


fn main(){
    let mut r = Rom::new();

    // LOAD control lines
    for addr in 0..0b1_000_000_000 {
        r.data[addr] = PC_ENC | PC_OUT | RAM_ENO | IR_CEN | IR_LOAD
    }

    // Integer Register-Immediate Computational Instructions
    const OP_IMM: u8 = 0x04;
    for ANY in 0u8 ..= 1u8 {
        r[InstAddr::from_fields_b(0, ANY != 0, OP_IMM)] |= CNTRL_IMMOUT | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_ADD;                                     // ADDI
        r[InstAddr::from_fields_b(1, ANY != 0, OP_IMM)] |= CNTRL_IMMOUT | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_LLSHFT;                                  // SLLI  ( shift left logical imm )
        r[InstAddr::from_fields_b(2, ANY != 0, OP_IMM)] |= CNTRL_IMMOUT | ALU_OUT | ALU_OUTMOD_SIGNBIT | ALU_OP_SUB;                                     // SLTI  ( set less than imm )
        r[InstAddr::from_fields_b(3, ANY != 0, OP_IMM)] |= CNTRL_IMMOUT | ALU_OUT | ALU_OUTMOD_SUBC    | ALU_OP_SUB;                                     // SLTIU ( set less than imm unsigned )
        r[InstAddr::from_fields_b(4, ANY != 0, OP_IMM)] |= CNTRL_IMMOUT | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_XOR;                                     // XORI
        r[InstAddr::from_fields_b(5, ANY != 0, OP_IMM)] |= CNTRL_IMMOUT | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_RSHFT;                                   // SRLI  ( shift right logical imm )
        r[InstAddr::from_fields_b(5, ANY != 0, OP_IMM)] |= CNTRL_IMMOUT | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_RSHFT;                                   // SRAI  ( shift right arithmetic imm )
        r[InstAddr::from_fields_b(6, ANY != 0, OP_IMM)] |= CNTRL_IMMOUT | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_AND | ALU_INVA | ALU_INVB | ALU_INVOUT;  // ORI
        r[InstAddr::from_fields_b(7, ANY != 0, OP_IMM)] |= CNTRL_IMMOUT | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_AND;                                     // ANDI
    }

   // DOWN-CLOCK: SWITCH IRS & LOAD in IR 1 & PARSE in IR 2
   // UP-CLOCK: EXECUTE in IR 2

   // println!("{:?}", r);
    use std::io::Write;
    let mut fout = std::fs::File::create("rom.bin").expect("Opening out file!");
    // Write to file
    for addr in 0..0b1_000_000_000 {
        let b1 = ((r.data[addr] & 0x000000FF) >> 8*0) as u8;
        let b2 = ((r.data[addr] & 0x0000FF00) >> 8*1) as u8;
        let b3 = ((r.data[addr] & 0x00FF0000) >> 8*2) as u8;
        let b4 = ((r.data[addr] & 0xFF000000) >> 8*3) as u8;
        fout.write_all(&[b1, b2, b3, b4]).unwrap();
    }
}